# array-ops

Ready made default method implementations for array data types.

## Overview

This crate provides a number of traits with default implementations for most of the standard
library's methods on array like data structures. All you need to do to apply them to your own array
like data structure is to implement `HasLength` and `Index<usize>` (and `IndexMut<usize>` for
mutable operations), which means you need a `len()` method and an `index()` method, and the `Array`
trait will provide default methods for everything else, implemented using just those two methods.

## Documentation

-   [API docs](https://docs.rs/array-ops)

# Example

```rust
# use array_ops::*;
# use std::ops::{Index, IndexMut};
#[derive(PartialEq, Eq, Debug)]
struct MyNewtypedVec<A>(Vec<A>);

impl<A> From<Vec<A>> for MyNewtypedVec<A> {
    fn from(vec: Vec<A>) -> Self {
        Self(vec)
    }
}

impl<A> HasLength for MyNewtypedVec<A> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<A> Index<usize> for MyNewtypedVec<A> {
    type Output = A;
    fn index(&self, index: usize) -> &A {
        self.0.index(index)
    }
}

impl<A> IndexMut<usize> for MyNewtypedVec<A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        self.0.index_mut(index)
    }
}

impl<A> Array for MyNewtypedVec<A> {}
impl<A> ArrayMut for MyNewtypedVec<A> {}

# fn main() {
let mut my_vec = MyNewtypedVec::from(vec![3, 1, 3, 3, 7]);
assert!(my_vec.starts_with(&[3, 1, 3]));
my_vec.sort_unstable();
let expected = MyNewtypedVec::from(vec![1, 3, 3, 3, 7]);
assert_eq!(expected, my_vec);
# }
```

## Licence

Copyright 2020 Bodil Stokke

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct][coc]. By
participating in this project you agree to abide by its terms.

[immutable.rs]: https://immutable.rs/
[coc]: https://github.com/bodil/sized-chunks/blob/master/CODE_OF_CONDUCT.md
