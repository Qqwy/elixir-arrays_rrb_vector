mod stored_term;

use rustler::types::atom::Atom;
use rustler::Env;
use rustler::ResourceArc;
use rustler::Term;

use core::ops::Deref;
use std::cmp::Ordering;
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

/// An immutable 'RRB'-vector which stores arbitrary Elixir/Erlang terms.
pub struct TermVector(Vector<StoredTerm>);
type VectorResource = ResourceArc<TermVector>;

/// A left-to-right iterator which is able to reference the vector it originated from.
// TODO There might be ways to clean this up;
// the type wrapping seems _very_ complex.
pub struct VectorIteratorPair(
    OwningHandle<
        Box<Vector<StoredTerm>>,
        Box<std::sync::Mutex<im::vector::Iter<'static, StoredTerm>>>,
    >,
);
type VectorIteratorPairResource<'a> = ResourceArc<VectorIteratorPair>;

/// A right-to-left iterator which is able to reference the vector it originated from.
// TODO There might be ways to clean this up;
// the type wrapping seems _very_ complex.
type VectorReverseIterator = std::iter::Rev<im::vector::Iter<'static, StoredTerm>>;
pub struct VectorReverseIteratorPair(
    OwningHandle<
        Box<Vector<StoredTerm>>,
        Box<std::sync::Mutex<VectorReverseIterator>>,
    >,
);
type VectorReverseIteratorPairResource<'a> = ResourceArc<VectorReverseIteratorPair>;

// /// A left-to-right mutable iterator which is able to reference the vector it originated from.
// /// Used for mapping
// // TODO There might be ways to clean this up;
// // the type wrapping seems _very_ complex.
// pub struct VectorIteratorMutPair(
//     OwningHandle<
//             Box<Vector<StoredTerm>>,
//         Box<std::sync::Mutex<im::vector::IterMut<'static, StoredTerm>>>,
//         >,
// );
// type VectorIteratorMutPairResource<'a> = ResourceArc<VectorIteratorMutPair>;

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(TermVector, env);
    rustler::resource!(VectorIteratorPair, env);
    rustler::resource!(VectorReverseIteratorPair, env);
    // rustler::resource!(VectorIteratorMutPair, env);
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
fn append_impl(vector: VectorResource, item: StoredTerm) -> VectorResource {
    let mut new_vector = vector.0.clone();
    new_vector.push_back(item);
    ResourceArc::new(TermVector(new_vector))
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

// #[rustler::nif]
// fn to_iterator_mut(vector: VectorResource) -> VectorIteratorMutPairResource<'static> {
//     let new_vector = Box::new(vector.0.clone());
//     let oh = OwningHandle::new_with_fn(new_vector, unsafe {
//         |vec| {
//             let mvec = vec as *mut im::Vector<stored_term::StoredTerm>; // TODO is this safe?
//             Box::new(std::sync::Mutex::new((*mvec).iter_mut()))
//         }
//     });
//     ResourceArc::new(VectorIteratorMutPair(oh))
// }

// fn iterator_mut_replace_and_next(iterator_pair: VectorIteratorMutPairResource<'static>, replacement: StoredTerm) -> Result<StoredTerm, Atom> {
//     let iterator = iterator_pair.0.lock().unwrap()

//     *iterator.get() = replacement;
//     let next_val =
//         match iterator.next().map(|x| x.clone()) {
//             Some(val) => Ok(val),
//             None => Err(atoms::empty()),
//         };
// }

#[rustler::nif]
fn iterator_next(iterator_pair: VectorIteratorPairResource<'static>) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().cloned() {
        Some(val) => Ok(val),
        None => Err(atoms::empty()),
    }
}

#[rustler::nif]
fn reverse_iterator_next(
    iterator_pair: VectorReverseIteratorPairResource<'static>,
) -> Result<StoredTerm, Atom> {
    match iterator_pair.0.lock().unwrap().next().cloned() {
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
    let new_vector = vector.0.update(index, item);
    ResourceArc::new(TermVector(new_vector))
}

#[rustler::nif]
fn resize_impl(vector: VectorResource, size: usize, default: StoredTerm) -> VectorResource {
    let mut new_vector = vector.0.clone();
    let vector_size = new_vector.len();
    match size.cmp(&vector_size) {
        Ordering::Less =>
            new_vector.truncate(size),
        Ordering::Greater =>
            new_vector = new_vector + std::iter::repeat(default).take(size - vector_size).collect(),
        Ordering::Equal => (),// Do nothing
    }

    ResourceArc::new(TermVector(new_vector))
}

#[rustler::nif]
fn slice_impl(vector: VectorResource, lower: usize, higher: usize) -> VectorResource {
    let slice_vec = vector.0.clone().slice(lower..higher);
    ResourceArc::new(TermVector(slice_vec))
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
        resize_impl,
        slice_impl,
    ],
    load = load
);
