// use std::cmp::min;
// use std::cmp::Ordering;


use rustler::types::atom::Atom;
use rustler::types::tuple::make_tuple;
use rustler::Encoder;
use rustler::Env;
use rustler::Term;
use rustler::ResourceArc;
use rustler::LocalPid;
use rustler::types::tuple::get_tuple;

use rustler::env::OwnedEnv;
use rustler::env::SavedTerm;

use owning_ref::OwningHandle;
use core::ops::Deref;

use im::Vector;

mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,
        unsupported_type,
        empty,
    }
}

// Put this in a ResourceArc:
#[derive(Clone)]
pub struct TermBox
{
    inner: std::sync::Arc<TermBoxContents>,
}

struct TermBoxContents
{
    owned_env: OwnedEnv,
    saved_term: SavedTerm
}
// I believe this is OK since we never alter the TermBox
// once it is created.
unsafe impl Sync for TermBoxContents {}

impl TermBox {
    pub fn new(term: &Term) -> Self {
        Self{inner: std::sync::Arc::new(TermBoxContents::new(term))}
    }

    pub fn get<'a>(&self, env: Env<'a>) -> Term<'a> {

        // Copy over term from owned environment to the target environment
        self.inner.owned_env.run(|inner_env| {
            let term = self.inner.saved_term.load(inner_env);
            term.in_env(env)
        })
    }
}

impl TermBoxContents {
    fn new(term: &Term) -> Self {
        let owned_env = OwnedEnv::new();
        let saved_term = owned_env.save(*term);
        Self{owned_env: owned_env, saved_term: saved_term}
    }
}



#[derive(Clone)]
pub enum StoredTerm {
    Integer(i64),
    Float(f64),
    AnAtom(Atom),
    Tuple(Vec<StoredTerm>),
    EmptyList(),
    List(Vec<StoredTerm>),
    Bitstring(String),
    Pid(LocalPid),
    Other(TermBox),
}

pub struct TermVector(Vector<StoredTerm>);
type VectorResource = ResourceArc<TermVector>;

pub struct VectorIteratorPair(OwningHandle<Box<Vector<StoredTerm>>, Box::<std::sync::Mutex<im::vector::Iter<'static, StoredTerm>>>>);
pub struct VectorReverseIteratorPair(OwningHandle<Box<Vector<StoredTerm>>, Box::<std::sync::Mutex<std::iter::Rev<im::vector::Iter<'static, StoredTerm>>>>>);
// pub struct VectorIteratorPair<'a> {
//     vector: Vector<StoredTerm>,
//     iterator: im::vector::Iter<'a, StoredTerm>,
// }

// pub struct VectorIterator<'a>(im::vector::Iter<'a, StoredTerm>);
type VectorIteratorPairResource<'a> = ResourceArc<VectorIteratorPair>;
type VectorReverseIteratorPairResource<'a> = ResourceArc<VectorReverseIteratorPair>;

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(TermVector, env);
    rustler::resource!(VectorIteratorPair, env);
    rustler::resource!(VectorReverseIteratorPair, env);
    // rustler::resource!(VectorIterator, env);
    true
}

impl Encoder for StoredTerm {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            StoredTerm::Integer(inner) => inner.encode(env),
            StoredTerm::Float(inner) => inner.encode(env),
            StoredTerm::AnAtom(inner) => inner.encode(env),
            StoredTerm::Tuple(inner) => {
                let terms: Vec<_> = inner.iter().map(|t| t.encode(env)).collect();
                make_tuple(env, terms.as_ref()).encode(env)
            }
            StoredTerm::EmptyList() => rustler::Term::list_new_empty(env),
            StoredTerm::List(inner) => inner.encode(env),
            StoredTerm::Bitstring(inner) => inner.encode(env),
            StoredTerm::Pid(inner) => inner.encode(env),
            StoredTerm::Other(inner) => inner.get(env),
        }
    }
}


fn convert_to_supported_term(term: &Term) -> StoredTerm {
    match term.get_type() {
        rustler::TermType::Atom => term.decode().map(StoredTerm::AnAtom).unwrap(),
        rustler::TermType::Binary => term.decode().map(StoredTerm::Bitstring).unwrap(),
        rustler::TermType::EmptyList => StoredTerm::EmptyList(),
        rustler::TermType::List => {
            let items = term.decode::<Vec<Term>>().unwrap();
            let converted_items =
                items
                .iter()
                .map(convert_to_supported_term)
                .collect();

            StoredTerm::List(converted_items)
        },
        rustler::TermType::Number =>
            term.decode::<i64>().map(StoredTerm::Integer)
            .or(term.decode::<f64>().map(StoredTerm::Float))
            .unwrap_or(StoredTerm::Other(TermBox::new(term))),
        rustler::TermType::Tuple => {
            let elems = get_tuple(*term).unwrap();
            let converted_elems =
                elems
                .iter()
                .map(convert_to_supported_term)
                .collect();
            StoredTerm::Tuple(converted_elems)
        }
        rustler::TermType::Pid => term.decode().map(StoredTerm::Pid).unwrap(),
        _other => StoredTerm::Other(TermBox::new(term)),
    }
}

#[rustler::nif]
fn empty_impl() -> VectorResource {
    let new_vector = Vector::new();
    ResourceArc::new(TermVector(new_vector))
}

#[rustler::nif]
fn size_impl(vector: VectorResource) -> usize {
    vector.0.len()
}

#[rustler::nif]
fn append_impl(vector: VectorResource, term: Term) -> Result<VectorResource, Atom> {
    // let item = match convert_to_supported_term(&term) {
    //     None => return Err(atoms::unsupported_type()),
    //     Some(term) => term,
    // };
    let item = convert_to_supported_term(&term);
    let mut new_vector = vector.0.clone();
    new_vector.push_back(item);
    Ok(ResourceArc::new(TermVector(new_vector)))
}

#[rustler::nif]
fn extract_impl(vector: VectorResource) -> Result<(StoredTerm, VectorResource), Atom> {
    let mut new_vector : Vector<StoredTerm> = vector.deref().0.clone();
    match new_vector.pop_back() {
        Some(item) => Ok((item, ResourceArc::new(TermVector(new_vector)))),
        None => Err(atoms::empty()),
    }
}

#[rustler::nif]
fn to_list_impl(vector: VectorResource) -> Vec<StoredTerm> {
    vector.0.iter().cloned().collect()
}

#[rustler::nif]
fn to_iterator(vector: VectorResource) -> VectorIteratorPairResource<'static> {
    let new_vector = Box::new(vector.0.clone());
    let oh = OwningHandle::new_with_fn(
        new_vector,
        unsafe { |vec| Box::new(std::sync::Mutex::new((*vec).iter())) }
    );
    ResourceArc::new(VectorIteratorPair(oh))
}

#[rustler::nif]
fn to_reverse_iterator(vector: VectorResource) -> VectorReverseIteratorPairResource<'static> {
    let new_vector = Box::new(vector.0.clone());
    let oh = OwningHandle::new_with_fn(
        new_vector,
        unsafe { |vec| Box::new(std::sync::Mutex::new((*vec).iter().rev())) }
    );
    ResourceArc::new(VectorReverseIteratorPair(oh))
}

#[rustler::nif]
fn iterator_next(iterator_pair: VectorIteratorPairResource<'static>) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().map(|x| x.clone()) {
        Some(val) => Ok(val),
        None =>  Err(atoms::empty()),
    }
}

#[rustler::nif]
fn reverse_iterator_next(iterator_pair: VectorReverseIteratorPairResource<'static>) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().map(|x| x.clone()) {
        Some(val) => Ok(val),
        None =>  Err(atoms::empty()),
    }
}

#[rustler::nif]
fn get_impl(vector: VectorResource, index: usize) -> StoredTerm {
    vector.0[index].clone()
}

#[rustler::nif]
fn replace_impl(vector: VectorResource, index: usize, term: Term) -> VectorResource {
    let item = convert_to_supported_term(&term);
    let mut new_vector = vector.0.clone();
    new_vector.set(index, item);
    ResourceArc::new(TermVector(new_vector))
}

rustler::init!(
    "Elixir.ArraysRRBVector",
    [empty_impl,

     append_impl,
     extract_impl,

     size_impl,

     to_list_impl,

     to_iterator,
     to_reverse_iterator,
     iterator_next,
     reverse_iterator_next,

     get_impl,
     replace_impl,
    ], load = load);
