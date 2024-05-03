/*
Test
https://atcoder.jp/contests/typical90/submissions/53044247

https://atcoder.jp/contests/practice2/submissions/53044649
*/

/// Segment Treeに乗せるデータ
#[derive(Clone, Debug)]
struct Data {
    data: i64,
}

/// 作用素のデータ
#[derive(Clone, Debug)]
struct Lazy {
    b: i64,
    c: i64,
}

/// モノイド演算の定義
impl Monoid for Data {
    type S = Data;
    fn identity() -> Self::S {
        Data { data: 0 }
    }
    fn op(a: &Self::S, other: &Self::S) -> Self::S {
        Data {
            data: (a.data + other.data) % 998244353,
        }
    }
}

/// 遅延伝播の定義
impl MapMonoid for Data {
    type M = Data;
    type F = Lazy;
    fn identity_map() -> Self::F {
        Lazy { b: 1, c: 0 }
    }
    fn mapping(x: &Self::M, f: &Self::F, l: usize) -> Self::M {
        Data {
            data: (f.b * x.data + f.c) % 998244353,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        Lazy {
            b: f.b * g.b % 998244353,
            c: (f.b * g.c + f.c) % 998244353,
        }
    }
}

trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn op(a: &Self::S, other: &Self::S) -> Self::S;
}

trait MapMonoid {
    type M: Monoid;
    type F: Clone;
    fn identity_element() -> <Self::M as Monoid>::S {
        Self::M::identity()
    }
    fn op(a: &<Self::M as Monoid>::S, b: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        Self::M::op(a, b)
    }
    /// identity map
    fn identity_map() -> Self::F;
    /// f(x)
    fn mapping(x: &<Self::M as Monoid>::S, f: &Self::F, l: usize) -> <Self::M as Monoid>::S;
    /// f ◦ g
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

struct LazySegTree<T: MapMonoid> {
    size: usize,
    log: usize,
    data: Vec<<T::M as Monoid>::S>,
    /// lazy[k]: ノードkが子に伝播していない作用素
    lazy: Vec<T::F>,
}

impl<T: MapMonoid> LazySegTree<T> {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let log = size.ilog2() as usize;
        let data: Vec<<T::M as Monoid>::S> = vec![T::identity_element(); size << 1 | 1];
        let lazy: Vec<<T as MapMonoid>::F> = vec![T::identity_map(); 2 * size + 1];
        Self {
            size,
            log,
            data,
            lazy,
        }
    }

    // vector vで初期化
    fn set(&mut self, v: &Vec<<T::M as Monoid>::S>) {
        for i in 0..v.len() {
            self.data[i + self.size] = v[i].clone();
        }
        for i in (1..self.size).rev() {
            self.data[i] = <T as MapMonoid>::op(&self.data[i << 1], &self.data[i << 1 | 1]);
        }
    }

    /// xを作用
    fn apply<R>(&mut self, r: R, x: T::F)
    where
        R: std::ops::RangeBounds<usize>,
    {
        let l = {
            match r.start_bound() {
                std::ops::Bound::Included(&l) => l + self.size,
                std::ops::Bound::Excluded(&l) => l + self.size + 1,
                std::ops::Bound::Unbounded => self.size,
            }
        };
        let r = {
            match r.end_bound() {
                std::ops::Bound::Included(&r) => r + self.size + 1,
                std::ops::Bound::Excluded(&r) => r + self.size,
                std::ops::Bound::Unbounded => self.size << 1,
            }
        };

        // 親から子へ遅延伝播
        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i, 1 << i);
            }
            if (r >> i) << i != r {
                self.push(r >> i, 1 << i);
            }
        }

        // 範囲に対応するノードに対して作用を適用
        let mut l2 = l;
        let mut r2 = r;
        while l2 < r2 {
            if l2 & 1 == 1 {
                self.all_apply(l2, x.clone(), 1);
                l2 += 1;
            }
            if r2 & 1 == 1 {
                r2 -= 1;
                self.all_apply(r2, x.clone(), 1);
            }
            l2 >>= 1;
            r2 >>= 1;
        }

        // 親の再計算
        for i in 1..=self.log {
            if (l >> i) << i != l {
                self.update(l >> i);
            }
            if (r >> i) << i != r {
                self.update(r >> i);
            }
        }
    }

    /// 演算結果を取得
    fn prod<R>(&mut self, r: R) -> <T::M as Monoid>::S
    where
        R: std::ops::RangeBounds<usize>,
    {
        let mut l = {
            match r.start_bound() {
                std::ops::Bound::Included(&l) => l + self.size,
                std::ops::Bound::Excluded(&l) => l + self.size + 1,
                std::ops::Bound::Unbounded => self.size,
            }
        };
        let mut r = {
            match r.end_bound() {
                std::ops::Bound::Included(&r) => r + self.size + 1,
                std::ops::Bound::Excluded(&r) => r + self.size,
                std::ops::Bound::Unbounded => self.size << 1,
            }
        };
        // 親から子へ遅延伝播
        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i, 1 << i);
            }
            if (r >> i) << i != r {
                self.push(r >> i, 1 << i);
            }
        }

        let mut v_l = T::identity_element();
        let mut v_r = T::identity_element();

        while l < r {
            if l & 1 == 1 {
                v_l = T::op(&v_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                v_r = T::op(&self.data[r], &v_r);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&v_l, &v_r)
    }

    /// p番目の要素を取得
    fn get(&mut self, mut p: usize) -> <T::M as Monoid>::S {
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i, 1 << i);
        }
        self.data[p].clone()
    }

    // ノードkの再計算
    fn update(&mut self, k: usize) {
        self.data[k] = T::op(&self.data[k << 1], &self.data[k << 1 | 1]);
    }

    // ノードkに作用Fを適用
    fn all_apply(&mut self, k: usize, f: T::F, width: usize) {
        self.data[k] = <T as MapMonoid>::mapping(&self.data[k], &f, width);
        if k < self.size {
            self.lazy[k] = <T as MapMonoid>::composition(&f, &self.lazy[k]);
        }
    }

    // ノードkの遅延伝播
    fn push(&mut self, k: usize, width: usize) {
        self.all_apply(k << 1, self.lazy[k].clone(), width);
        self.all_apply(k << 1 | 1, self.lazy[k].clone(), width);
        self.lazy[k] = T::identity_map();
    }
}
