/*
test(区間最大値)
https://atcoder.jp/contests/tessoku-book/submissions/53040979

test(区間和)
https://atcoder.jp/contests/tessoku-book/submissions/53041016


*/

/// Segment Treeに乗せるデータ
#[derive(Clone, Debug)]
struct Data {
    max: i64,
}

trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn op(a: &Self::S, other: &Self::S) -> Self::S;
}

impl Monoid for Data {
    type S = Data;
    fn identity() -> Self::S {
        Data { max: 0 }
    }
    fn op(a: &Self::S, other: &Self::S) -> Self::S {
        Data {
            max: std::cmp::max(a.max, other.max),
        }
    }
}

struct SegTree<M: Monoid> {
    n: usize,
    data: Vec<M::S>,
}

impl<M: Monoid> SegTree<M> {
    fn new(n: usize) -> Self {
        let n_ = n.next_power_of_two();
        SegTree {
            n: n_,
            data: vec![M::identity(); 2 * n_ + 1],
        }
    }

    /// update i-th element (0-indexed)
    fn update(&mut self, mut i: usize, x: M::S) {
        i += self.n;
        self.data[i] = x;
        while i > 1 {
            i /= 2;
            self.data[i] = M::op(&self.data[i * 2], &self.data[i * 2 + 1]);
        }
    }

    fn query<R>(&self, range: R) -> M::S
    where
        R: std::ops::RangeBounds<usize>,
    {
        let mut l = {
            match range.start_bound() {
                std::ops::Bound::Included(&l) => l + self.n,
                std::ops::Bound::Excluded(&l) => l + self.n + 1,
                std::ops::Bound::Unbounded => self.n,
            }
        };

        let mut r = {
            match range.end_bound() {
                std::ops::Bound::Included(&r) => r + self.n + 1,
                std::ops::Bound::Excluded(&r) => r + self.n,
                std::ops::Bound::Unbounded => 2 * self.n,
            }
        };

        let mut res = M::identity();

        while l < r {
            if l & 1 == 1 {
                res = M::op(&res, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = M::op(&res, &self.data[r]);
            }
            l /= 2;
            r /= 2;
        }

        res
    }
}
