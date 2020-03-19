// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::array::ArrayMut;
use core::{cmp::Ordering, ops::Index};
use rand_core::{RngCore, SeedableRng};

fn gen_range<R: RngCore>(rng: &mut R, min: usize, max: usize) -> usize {
    let range = max - min;
    min + (rng.next_u64() as usize % range)
}

// Adapted from the Java version at
//    http://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf
// with semi-randomised pivot points.
// Should be O(n) to O(n log n)
fn do_quicksort<Arr, F, R>(array: &mut Arr, left: usize, right: usize, cmp: &mut F, rng: &mut R)
where
    Arr: ArrayMut + ?Sized,
    <Arr as Index<usize>>::Output: Sized,
    F: FnMut(&<Arr as Index<usize>>::Output, &<Arr as Index<usize>>::Output) -> Ordering,
    R: RngCore,
{
    if right <= left {
        return;
    }

    let l = left as isize;
    let r = right as isize;
    let p = gen_range(rng, left, right + 1) as isize;
    let mut l1 = l;
    let mut r1 = r;
    let mut l2 = l - 1;
    let mut r2 = r;

    array.swap(r as usize, p as usize);
    loop {
        while l1 != r && array.map_pair(l1 as usize, r as usize, |a, b| cmp(a, b)) == Ordering::Less
        {
            l1 += 1;
        }

        r1 -= 1;
        while r1 != r && array.map_pair(r as usize, r1 as usize, |a, b| cmp(a, b)) == Ordering::Less
        {
            if r1 == l {
                break;
            }
            r1 -= 1;
        }
        if l1 >= r1 {
            break;
        }
        array.swap(l1 as usize, r1 as usize);
        if l1 != r && array.map_pair(l1 as usize, r as usize, |a, b| cmp(a, b)) == Ordering::Equal {
            l2 += 1;
            array.swap(l2 as usize, l1 as usize);
        }
        if r1 != r && array.map_pair(r as usize, r1 as usize, |a, b| cmp(a, b)) == Ordering::Equal {
            r2 -= 1;
            array.swap(r1 as usize, r2 as usize);
        }
    }
    array.swap(l1 as usize, r as usize);

    r1 = l1 - 1;
    l1 += 1;
    let mut k = l;
    while k < l2 {
        array.swap(k as usize, r1 as usize);
        r1 -= 1;
        k += 1;
    }
    k = r - 1;
    while k > r2 {
        array.swap(l1 as usize, k as usize);
        k -= 1;
        l1 += 1;
    }

    if r1 >= 0 {
        do_quicksort(array, left, r1 as usize, cmp, rng);
    }
    do_quicksort(array, l1 as usize, right, cmp, rng);
}

pub(crate) fn quicksort<Arr, F>(array: &mut Arr, left: usize, right: usize, mut cmp: F)
where
    Arr: ArrayMut + ?Sized,
    <Arr as Index<usize>>::Output: Sized,
    F: FnMut(&<Arr as Index<usize>>::Output, &<Arr as Index<usize>>::Output) -> Ordering,
{
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    do_quicksort(array, left, right, &mut cmp, &mut rng);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::array::Array;
    use std::collections::VecDeque;

    #[test]
    fn test_quicksort() {
        let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(1337);
        let mut vec: VecDeque<_> = std::iter::from_fn(move || Some(rng.next_u64()))
            .take(16384)
            .collect();
        let last = vec.len() - 1;
        quicksort(&mut vec, 0, last, &Ord::cmp);
        assert!(vec.is_sorted());
    }
}
