/*
test
https://atcoder.jp/contests/practice2/submissions/53040053

*/

use num::traits::{NumAssign, Zero};

pub struct FenwickTree<T: Copy + Zero + NumAssign> {
    n: usize,
    array: Vec<T>,
}

impl<T: Copy + Zero + NumAssign> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            array: vec![T::zero(); n + 1],
        }
    }

    // data[i] += x (0-indexed)
    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.n {
            self.array[i] += x;
            i += i & (!i + 1);
        }
    }

    // sum of data[0] + data[1] + ... + data[i-1] (0-indexed)
    fn sum(&self, mut i: usize) -> T {
        let mut s = T::zero();
        while i > 0 {
            s += self.array[i];
            i -= i & (!i + 1);
        }
        s
    }

    pub fn range_sum<R>(&self, range: R) -> T
    where
        R: std::ops::RangeBounds<usize>,
    {
        let r = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.n,
        };

        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => return self.sum(r),
        };

        self.sum(r) - self.sum(l)
    }
}
