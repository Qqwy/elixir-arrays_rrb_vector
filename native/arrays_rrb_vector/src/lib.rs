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

use im::Vector;

mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,
        unsupported_type,
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
    // Exception(),
    // Function(),
    List(Vec<StoredTerm>),
    Bitstring(String),
    Pid(LocalPid),
    Other(TermBox),
}

pub struct TermVector(Vector<StoredTerm>);
type VectorResource = ResourceArc<TermVector>;

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(TermVector, env);
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
                .into_iter()
                .map(|item: Term| convert_to_supported_term(&item))
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
                .into_iter()
                .map(|item: Term| convert_to_supported_term(&item))
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
fn to_list_impl(vector: VectorResource) -> Vec<StoredTerm> {
    vector.0.iter().cloned().collect()
}

rustler::init!("Elixir.ArraysRRBVector", [empty_impl, append_impl, size_impl, to_list_impl], load = load);
