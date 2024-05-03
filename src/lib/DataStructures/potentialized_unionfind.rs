pub struct PtUnionFind {
    parent_or_size: Vec<i32>,
    potential: Vec<i64>,
}

impl PtUnionFind {
    pub fn new(n: usize) -> Self {
        PtUnionFind {
            parent_or_size: vec![-1; n],
            potential: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent_or_size[x] < 0 {
            x
        } else {
            let parent = self.parent_or_size[x] as usize;
            let root = self.find(parent);
            self.parent_or_size[x] = root as i32;
            self.potential[x] += self.potential[parent];
            root
        }
    }

    // potential[y] = potential[x] + w
    pub fn unite(&mut self, x: usize, y: usize, w: i64) -> bool {
        let r_x = self.find(x);
        let r_y = self.find(y);
        if r_x == r_y {
            return self.potential[y] == self.potential[x] + w;
        } else {
            if self.size(x) < self.size(y) {
                self.parent_or_size[r_y] += self.parent_or_size[r_x];
                self.parent_or_size[r_x] = r_y as i32;
                self.potential[r_x] = self.potential[y] - self.potential[x] - w;
                true
            } else {
                self.parent_or_size[r_x] += self.parent_or_size[r_y];
                self.parent_or_size[r_y] = r_x as i32;
                self.potential[r_y] = self.potential[x] - self.potential[y] + w;
                true
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent_or_size[root] as usize
    }
}
