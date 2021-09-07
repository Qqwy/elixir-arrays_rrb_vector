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

use im::Vector;

mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,
        unsupported_type,
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
        }
    }
}


fn convert_to_supported_term(term: &Term) -> SupportedTerm {
    match term.get_type() {
        rustler::TermType::Atom => term.decode().map(SupportedTerm::AnAtom).unwrap(),
        rustler::TermType::Binary => term.decode().map(SupportedTerm::Bitstring).unwrap(),
        rustler::TermType::EmptyList => SupportedTerm::EmptyList(),
        // rustler::TermType::Exception => SupportedTerm::Exception(),
        // rustler::TermType::Fun => SupportedTerm::Function(),
        rustler::TermType::List => {
            let items = term.decode::<Vec<Term>>().unwrap();
            let converted_items =
                items
                .into_iter()
                .map(|item: Term| convert_to_supported_term(&item))
                .collect();

            SupportedTerm::List(converted_items)
        },
        rustler::TermType::Number => match term.decode::<i64>() {
            Ok(val) => SupportedTerm::Integer(val),
            Err(_) => term.decode::<f64>().map(SupportedTerm::Float).unwrap(),
        },
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
        // Map => todo!,
        // Pid => todo!,
        // Port => todo!,
        // Ref => todo!,
        // Unknown => todo!,
        _other => {todo!("Support functions, exceptions, ports, maps, refs, unknown types")},
    }
    
    
    // if term.is_number() {
    //     match term.decode() {
    //         Ok(i) => Some(SupportedTerm::Integer(i)),
    //         Err(_) => match term.decode() {
    //             Ok(f) => Some(SupportedTerm::Float(f)),
    //             Err(_) => None,
    //         },
    //     }
    // } else if term.is_atom() {
    //     match term.atom_to_string() {
    //         Ok(a) => Some(SupportedTerm::Atom(a)),
    //         Err(_) => None,
    //     }
    // } else if term.is_tuple() {
    //     match get_tuple(*term) {
    //         Ok(t) => {
    //             let initial_length = t.len();
    //             let inner_terms: Vec<SupportedTerm> = t
    //                 .into_iter()
    //                 .filter_map(|i: Term| convert_to_supported_term(&i))
    //                 .collect();
    //             if initial_length == inner_terms.len() {
    //                 Some(SupportedTerm::Tuple(inner_terms))
    //             } else {
    //                 None
    //             }
    //         }
    //         Err(_) => None,
    //     }
    // } else if term.is_list() {
    //     match term.decode::<Vec<Term>>() {
    //         Ok(l) => {
    //             let initial_length = l.len();
    //             let inner_terms: Vec<SupportedTerm> = l
    //                 .into_iter()
    //                 .filter_map(|i: Term| convert_to_supported_term(&i))
    //                 .collect();
    //             if initial_length == inner_terms.len() {
    //                 Some(SupportedTerm::List(inner_terms))
    //             } else {
    //                 None
    //             }
    //         }
    //         Err(_) => None,
    //     }
    // } else if term.is_binary() {
    //     match term.decode() {
    //         Ok(b) => Some(SupportedTerm::Bitstring(b)),
    //         Err(_) => None,
    //     }
    // } else {
    //     None
    // }
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
