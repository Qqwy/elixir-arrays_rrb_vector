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
pub struct MutableTermBox
{
    inner: std::sync::Arc<std::sync::Mutex<MutableTermBoxContents>>,
}

struct MutableTermBoxContents
{
    owned_env: OwnedEnv,
    saved_term: SavedTerm
}

impl MutableTermBox {
    pub fn new(term: &Term) -> Self {
        Self{inner: std::sync::Arc::new(std::sync::Mutex::new(MutableTermBoxContents::new(term)))}
    }

    pub fn get<'a>(&self, env: Env<'a>) -> Term<'a> {
        let inner = self.inner.lock().unwrap();

        // Copy over term from owned environment to the target environment
        inner.owned_env.run(|inner_env| {
            let term = inner.saved_term.load(inner_env);
            term.in_env(env)
        })
    }

    pub fn set(&self, term: Term) -> Atom {
        let mut term_ptr = self.inner.lock().unwrap();
        term_ptr.owned_env.clear();
        term_ptr.saved_term = term_ptr.owned_env.save(term);

        atoms::ok()
    }
}

impl MutableTermBoxContents {
    fn new(term: &Term) -> Self {
        let owned_env = OwnedEnv::new();
        let saved_term = owned_env.save(*term);
        Self{owned_env: owned_env, saved_term: saved_term}
    }
}



#[derive(Clone)]
pub enum SupportedTerm {
    Integer(i64),
    Float(f64),
    AnAtom(Atom),
    Tuple(Vec<SupportedTerm>),
    EmptyList(),
    // Exception(),
    // Function(),
    List(Vec<SupportedTerm>),
    Bitstring(String),
    Pid(LocalPid),
    Other(MutableTermBox),
}

pub struct TermVector(Vector<SupportedTerm>);
type VectorResource = ResourceArc<TermVector>;

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(TermVector, env);
    true
}

impl Encoder for SupportedTerm {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            SupportedTerm::Integer(inner) => inner.encode(env),
            SupportedTerm::Float(inner) => inner.encode(env),
            SupportedTerm::AnAtom(inner) => inner.encode(env),
            SupportedTerm::Tuple(inner) => {
                let terms: Vec<_> = inner.iter().map(|t| t.encode(env)).collect();
                make_tuple(env, terms.as_ref()).encode(env)
            }
            SupportedTerm::EmptyList() => rustler::Term::list_new_empty(env),
            SupportedTerm::List(inner) => inner.encode(env),
            SupportedTerm::Bitstring(inner) => inner.encode(env),
            SupportedTerm::Pid(inner) => inner.encode(env),
            SupportedTerm::Other(inner) => inner.get(env),
        }
    }
}


fn convert_to_supported_term(term: &Term) -> SupportedTerm {
    match term.get_type() {
        rustler::TermType::Atom => term.decode().map(SupportedTerm::AnAtom).unwrap(),
        rustler::TermType::Binary => term.decode().map(SupportedTerm::Bitstring).unwrap(),
        rustler::TermType::EmptyList => SupportedTerm::EmptyList(),
        rustler::TermType::List => {
            let items = term.decode::<Vec<Term>>().unwrap();
            let converted_items =
                items
                .into_iter()
                .map(|item: Term| convert_to_supported_term(&item))
                .collect();

            SupportedTerm::List(converted_items)
        },
        rustler::TermType::Number =>
            term.decode::<i64>().map(SupportedTerm::Integer)
            .or(term.decode::<f64>().map(SupportedTerm::Float))
            .unwrap_or(SupportedTerm::Other(MutableTermBox::new(term))),
        rustler::TermType::Tuple => {
            let elems = get_tuple(*term).unwrap();
            let converted_elems =
                elems
                .into_iter()
                .map(|item: Term| convert_to_supported_term(&item))
                .collect();
            SupportedTerm::Tuple(converted_elems)
        }
        rustler::TermType::Pid => term.decode().map(SupportedTerm::Pid).unwrap(),
        _other => SupportedTerm::Other(MutableTermBox::new(term)),
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
fn to_list_impl(vector: VectorResource) -> Vec<SupportedTerm> {
    vector.0.iter().cloned().collect()
}

rustler::init!("Elixir.ArraysRRBVector", [empty_impl, append_impl, size_impl, to_list_impl], load = load);
