// use std::cmp::min;
// use std::cmp::Ordering;

mod stored_term;

use rustler::types::atom::Atom;
use rustler::Env;
use rustler::ResourceArc;
use rustler::Term;

use core::ops::Deref;
use im::Vector;
use owning_ref::OwningHandle;

use crate::stored_term::StoredTerm;

mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,
        unsupported_type,
        empty,
    }
}

pub struct TermVector(Vector<StoredTerm>);
type VectorResource = ResourceArc<TermVector>;

pub struct VectorIteratorPair(
    OwningHandle<
        Box<Vector<StoredTerm>>,
        Box<std::sync::Mutex<im::vector::Iter<'static, StoredTerm>>>,
    >,
);
pub struct VectorReverseIteratorPair(
    OwningHandle<
        Box<Vector<StoredTerm>>,
        Box<std::sync::Mutex<std::iter::Rev<im::vector::Iter<'static, StoredTerm>>>>,
    >,
);

type VectorIteratorPairResource<'a> = ResourceArc<VectorIteratorPair>;
type VectorReverseIteratorPairResource<'a> = ResourceArc<VectorReverseIteratorPair>;

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(TermVector, env);
    rustler::resource!(VectorIteratorPair, env);
    rustler::resource!(VectorReverseIteratorPair, env);
    // rustler::resource!(VectorIterator, env);
    true
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
fn append_impl(vector: VectorResource, item: StoredTerm) -> Result<VectorResource, Atom> {
    // let item = match convert_to_stored_term(&term) {
    //     None => return Err(atoms::unsupported_type()),
    //     Some(term) => term,
    // };
    let mut new_vector = vector.0.clone();
    new_vector.push_back(item);
    Ok(ResourceArc::new(TermVector(new_vector)))
}

#[rustler::nif]
fn extract_impl(vector: VectorResource) -> Result<(StoredTerm, VectorResource), Atom> {
    let mut new_vector: Vector<StoredTerm> = vector.deref().0.clone();
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
    let oh = OwningHandle::new_with_fn(new_vector, unsafe {
        |vec| Box::new(std::sync::Mutex::new((*vec).iter()))
    });
    ResourceArc::new(VectorIteratorPair(oh))
}

#[rustler::nif]
fn to_reverse_iterator(vector: VectorResource) -> VectorReverseIteratorPairResource<'static> {
    let new_vector = Box::new(vector.0.clone());
    let oh = OwningHandle::new_with_fn(new_vector, unsafe {
        |vec| Box::new(std::sync::Mutex::new((*vec).iter().rev()))
    });
    ResourceArc::new(VectorReverseIteratorPair(oh))
}

#[rustler::nif]
fn iterator_next(iterator_pair: VectorIteratorPairResource<'static>) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().map(|x| x.clone()) {
        Some(val) => Ok(val),
        None => Err(atoms::empty()),
    }
}

#[rustler::nif]
fn reverse_iterator_next(
    iterator_pair: VectorReverseIteratorPairResource<'static>,
) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().map(|x| x.clone()) {
        Some(val) => Ok(val),
        None => Err(atoms::empty()),
    }
}

#[rustler::nif]
fn get_impl(vector: VectorResource, index: usize) -> StoredTerm {
    vector.0[index].clone()
}

#[rustler::nif]
fn replace_impl(vector: VectorResource, index: usize, item: StoredTerm) -> VectorResource {
    let mut new_vector = vector.0.clone();
    new_vector.set(index, item);
    ResourceArc::new(TermVector(new_vector))
}

rustler::init!(
    "Elixir.ArraysRRBVector",
    [
        empty_impl,
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
    ],
    load = load
);
