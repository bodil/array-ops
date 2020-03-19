// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};

/// Trait for data structures which have a length.
pub trait HasLength {
    /// Return the length of the data structure.
    fn len(&self) -> usize;

    /// Return whether the data structure is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Trait for data structures which are indexed like arrays.
///
/// Types implementing this trait must have populated indexes from
/// `0` up to but not including `self.len()`.
pub trait Array: HasLength + Index<usize> {
    /// Get a reference to the element at the given index.
    fn get(&self, index: usize) -> Option<&<Self as Index<usize>>::Output> {
        if index >= self.len() {
            None
        } else {
            Some(&self[index])
        }
    }

    /// Get a reference to the first element in the array.
    fn first(&self) -> Option<&<Self as Index<usize>>::Output> {
        self.get(0)
    }

    /// Get a reference to the last element in the array.
    fn last(&self) -> Option<&<Self as Index<usize>>::Output> {
        if self.is_empty() {
            None
        } else {
            self.get(self.len() - 1)
        }
    }

    /// Return true if an element equivalent to `target` exists in the array.
    fn contains(&self, target: &<Self as Index<usize>>::Output) -> bool
    where
        <Self as Index<usize>>::Output: PartialEq,
    {
        for index in 0..self.len() {
            if &self[index] == target {
                return true;
            }
        }
        false
    }

    /// Perform a binary search for `target`.
    fn binary_search(&self, target: &<Self as Index<usize>>::Output) -> Result<usize, usize>
    where
        <Self as Index<usize>>::Output: Ord,
    {
        self.binary_search_by(|value| value.cmp(target))
    }

    /// Perform a binary search using a comparator function.
    fn binary_search_by<F>(&self, mut compare: F) -> Result<usize, usize>
    where
        F: FnMut(&<Self as Index<usize>>::Output) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return Err(0);
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = compare(&s[mid]);
            base = if cmp == Ordering::Greater { base } else { mid };
            size -= half;
        }
        let cmp = compare(&s[base]);
        if cmp == Ordering::Equal {
            Ok(base)
        } else {
            Err(base + (cmp == Ordering::Less) as usize)
        }
    }

    /// Perform a binary search using a key and a key extractor function.
    fn binary_search_by_key<K, F>(&self, key: &K, mut extract: F) -> Result<usize, usize>
    where
        F: FnMut(&<Self as Index<usize>>::Output) -> K,
        K: Ord,
    {
        self.binary_search_by(|i| extract(i).cmp(key))
    }

    /// Test whether the array is sorted.
    fn is_sorted(&self) -> bool
    where
        <Self as Index<usize>>::Output: PartialOrd,
    {
        self.is_sorted_by(|l, r| l.partial_cmp(r))
    }

    /// Test whether the array is sorted using a comparator function.
    fn is_sorted_by<F>(&self, mut compare: F) -> bool
    where
        F: FnMut(
            &<Self as Index<usize>>::Output,
            &<Self as Index<usize>>::Output,
        ) -> Option<Ordering>,
    {
        if self.len() < 2 {
            true
        } else {
            for i in 1..self.len() {
                if compare(&self[i - 1], &self[i]) == Some(Ordering::Greater) {
                    return false;
                }
            }
            true
        }
    }

    /// Test whether the array is sorted using a key extractor function.
    fn is_sorted_by_key<K, F>(&self, mut extract: F) -> bool
    where
        F: FnMut(&<Self as Index<usize>>::Output) -> K,
        K: PartialOrd<K>,
    {
        self.is_sorted_by(|l, r| extract(l).partial_cmp(&extract(r)))
    }

    /// Test whether the array starts with the elements in `slice`.
    fn starts_with(&self, slice: &[<Self as Index<usize>>::Output]) -> bool
    where
        <Self as Index<usize>>::Output: PartialEq + Sized,
    {
        if slice.len() > self.len() {
            return false;
        }
        for i in 0..slice.len() {
            if self[i] != slice[i] {
                return false;
            }
        }
        true
    }

    /// Test whether the array ends with the elements in `slice`.
    fn ends_with(&self, slice: &[<Self as Index<usize>>::Output]) -> bool
    where
        <Self as Index<usize>>::Output: PartialEq + Sized,
    {
        if slice.len() > self.len() {
            return false;
        }
        let offset = self.len() - slice.len();
        for i in 0..slice.len() {
            if self[offset + i] != slice[i] {
                return false;
            }
        }
        true
    }
}

/// Trait for arrays with mutable indexes.
pub trait ArrayMut: Array + IndexMut<usize> {
    /// Get a mutable reference to the element at the given index.
    fn get_mut(&mut self, index: usize) -> Option<&mut <Self as Index<usize>>::Output> {
        if index >= self.len() {
            None
        } else {
            Some(&mut self[index])
        }
    }

    /// Get a mutable reference to the first element in the array.
    fn first_mut(&mut self) -> Option<&mut <Self as Index<usize>>::Output> {
        self.get_mut(0)
    }

    /// Get a mutable reference to the last element in the array.
    fn last_mut(&mut self) -> Option<&mut <Self as Index<usize>>::Output> {
        if self.is_empty() {
            None
        } else {
            self.get_mut(self.len() - 1)
        }
    }

    /// Set the value of the element at the given index.
    ///
    /// Returns the previous value, or `None` if the index is out of bounds.
    fn set(
        &mut self,
        index: usize,
        value: <Self as Index<usize>>::Output,
    ) -> Option<<Self as Index<usize>>::Output>
    where
        <Self as Index<usize>>::Output: Sized,
    {
        self.get_mut(index).map(|p| std::mem::replace(p, value))
    }

    /// Swap the elements at two indexes.
    fn swap(&mut self, index1: usize, index2: usize)
    where
        <Self as Index<usize>>::Output: Sized,
    {
        if index1 != index2 {
            let ptr1: *mut <Self as Index<usize>>::Output = &mut self[index1];
            let ptr2: *mut <Self as Index<usize>>::Output = &mut self[index2];
            unsafe { std::ptr::swap(ptr1, ptr2) };
        }
    }

    /// Get mutable references to the elements at two indexes and call a function on them.
    ///
    /// This provides a safe way to get two mutable references into an array at the same time,
    /// which would normally be disallowed by the borrow checker.
    fn map_pair<F, A>(&mut self, index1: usize, index2: usize, mut f: F) -> A
    where
        F: FnMut(&mut <Self as Index<usize>>::Output, &mut <Self as Index<usize>>::Output) -> A,
    {
        if index1 == index2 {
            panic!("ArrayMut::map_pair: indices cannot be equal!");
        }
        let pa: *mut <Self as Index<usize>>::Output = self.index_mut(index1);
        let pb: *mut <Self as Index<usize>>::Output = self.index_mut(index2);
        unsafe { f(&mut *pa, &mut *pb) }
    }

    /// Sort the elements of the array.
    fn sort_unstable(&mut self)
    where
        <Self as Index<usize>>::Output: Ord + Sized,
    {
        self.sort_unstable_by(|l, r| l.cmp(r))
    }

    /// Sort the elements of the array using a comparator function.
    fn sort_unstable_by<F>(&mut self, mut compare: F)
    where
        <Self as Index<usize>>::Output: Sized,
        F: FnMut(&<Self as Index<usize>>::Output, &<Self as Index<usize>>::Output) -> Ordering,
    {
        crate::sort::quicksort(self, 0, self.len() - 1, |a, b| compare(a, b));
    }

    /// Sort the elements of the array using a key extractor function.
    fn sort_unstable_by_key<F, K>(&mut self, mut extract: F)
    where
        F: FnMut(&<Self as Index<usize>>::Output) -> K,
        K: Ord,
        <Self as Index<usize>>::Output: Sized,
    {
        self.sort_unstable_by(|l, r| extract(l).cmp(&extract(r)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestVec<A>(Vec<A>);

    impl<A> HasLength for TestVec<A> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl<A> Index<usize> for TestVec<A> {
        type Output = A;
        fn index(&self, index: usize) -> &A {
            &self.0[index]
        }
    }

    impl<A> IndexMut<usize> for TestVec<A> {
        fn index_mut(&mut self, index: usize) -> &mut A {
            &mut self.0[index]
        }
    }

    impl<A> Array for TestVec<A> {}
    impl<A> ArrayMut for TestVec<A> {}

    impl<A> FromIterator<A> for TestVec<A> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = A>,
        {
            Self(Vec::from_iter(iter))
        }
    }

    impl<A> From<Vec<A>> for TestVec<A> {
        fn from(vec: Vec<A>) -> Self {
            Self(vec)
        }
    }

    #[test]
    fn ops() {
        let mut vec = TestVec::from_iter(1..=3);
        assert_eq!(3, vec.len());
        assert_eq!(Some(&1), vec.first());
        assert_eq!(Some(&2), vec.get(1));
        assert_eq!(Some(&3), vec.last());
        *vec.first_mut().unwrap() = 3;
        *vec.last_mut().unwrap() = 1;
        *vec.get_mut(1).unwrap() = 5;
        vec.swap(0, 1);
        assert_eq!(TestVec::from(vec![5, 3, 1]), vec);
        assert!(!vec.is_sorted());
        vec.sort_unstable();
        assert_eq!(TestVec::from(vec![1, 3, 5]), vec);
        assert!(vec.is_sorted());
        assert_eq!(Ok(1), vec.binary_search(&3));
        assert_eq!(Err(1), vec.binary_search(&2));
        assert_eq!(Err(0), vec.binary_search(&0));
        assert_eq!(Err(3), vec.binary_search(&1337));
        assert!(vec.contains(&1));
        assert!(!vec.contains(&2));
        assert!(vec.contains(&3));
        assert!(!vec.contains(&4));
        assert!(vec.contains(&5));
        assert!(vec.starts_with(&[1, 3]));
        assert!(!vec.starts_with(&[1, 2, 3]));
        assert!(vec.ends_with(&[3, 5]));
        assert!(!vec.ends_with(&[3, 4, 5]));
    }
}
