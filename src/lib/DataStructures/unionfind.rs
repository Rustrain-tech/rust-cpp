/*
使用例
https://atcoder.jp/contests/atc001/submissions/51910479
*/

pub struct UnionFind {
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent_or_size: vec![-1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent_or_size[x] < 0 {
            x
        } else {
            let parent = self.parent_or_size[x] as usize;
            let root = self.find(parent);
            self.parent_or_size[x] = root as i32;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.parent_or_size[x] > self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent_or_size[root] as usize
    }
}
