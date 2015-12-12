//! A library for managing n-dimensional arrays in rust.

#[cfg(test)]
mod tests;

mod helpers;
use helpers::{index_to_position, position_to_index};

use std::borrow::Borrow;
use std::convert::AsRef;
use std::default::Default;
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

#[derive(Debug, Clone)]
/// An arbitrary n-dimensional array. Dimensions is the number of dimensions, magnitudes contains
/// the size of each dimension, and data contains the data for the array.
pub struct NArray<T> {
    pub dimensions: usize,
    pub magnitudes: Vec<usize>,
    pub data: Vec<T>
}
impl<T> NArray<T> {
    /// Returns a new NArray, with each index populated by a function of it's coordinates.)
    pub fn from_function<F>(dim: usize, mag: &[usize], func: F) -> Self 
        where F: Fn(&[usize]) -> T
    {
        if dim != mag.len() {
            panic!("Attempted to initialise NArray of dimension {} but {} magnitudes supplied",
                   dim,
                   mag.len());
        }
        let max_size = mag.clone().into_iter().fold(1, |acc, &item| acc*item);
        let mut data = Vec::<T>::with_capacity(max_size);

        for n in 0..max_size {
            let i = position_to_index(mag, n);
            data.push(func(i.borrow()));
        }

        NArray {
            dimensions: dim,
            magnitudes: mag.to_vec(),
            data: data
        }
    }
}
impl<T: Default> NArray<T> {
    /// Returns a new NArray, with each index populated by the default for a given type.
    pub fn new(dim: usize, mag: &[usize]) -> Self {
        Self::from_function(dim, mag, |_: &[usize]| -> T { T::default() })
    }
}

impl<T, S: AsRef<[usize]>> Index<S> for NArray<T> {
    type Output = T;
    fn index(&self, index: S) -> &Self::Output {
        let index = index.as_ref();
        return &self.data[index_to_position(self.dimensions, &self.magnitudes, index)]
    }
}
impl<T, S: AsRef<[usize]>> IndexMut<S> for NArray<T> {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        let index = index.as_ref();
        return &mut self.data[index_to_position(self.dimensions, &self.magnitudes, index)];
    }
}

impl<T> IntoIterator for NArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
