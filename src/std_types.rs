// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::VecDeque;
use std::ops::Index;

use crate::array::{Array, ArrayMut, HasLength};

// VecDeque

impl<A> HasLength for VecDeque<A> {
    fn len(&self) -> usize {
        VecDeque::len(self)
    }
}

impl<A> Array for VecDeque<A> {
    fn get(&self, index: usize) -> Option<&<Self as Index<usize>>::Output> {
        VecDeque::get(self, index)
    }

    fn contains(&self, target: &<Self as Index<usize>>::Output) -> bool
    where
        <Self as Index<usize>>::Output: PartialEq,
    {
        VecDeque::contains(self, target)
    }
}

impl<A> ArrayMut for VecDeque<A> {
    fn get_mut(&mut self, index: usize) -> Option<&mut <Self as Index<usize>>::Output> {
        VecDeque::get_mut(self, index)
    }

    fn swap(&mut self, index1: usize, index2: usize)
    where
        <Self as Index<usize>>::Output: Sized,
    {
        VecDeque::swap(self, index1, index2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_deque() {
        let mut vec: VecDeque<_> = vec![3, 2, 1].into();
        assert_eq!(3, HasLength::len(&vec));
        assert_eq!(Some(&3), Array::first(&vec));
        assert_eq!(Some(&1), Array::last(&vec));
        ArrayMut::sort_unstable(&mut vec);
        assert_eq!(Some(&1), Array::first(&vec));
        assert_eq!(Some(&3), Array::last(&vec));
    }
}
