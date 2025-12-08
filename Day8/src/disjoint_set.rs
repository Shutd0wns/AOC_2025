use std::cmp::Ordering;

pub struct DisjointSets {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: usize,
    num_of_sets: usize,
}


impl DisjointSets {
    pub fn new(size: usize) -> DisjointSets {
        let mut result = DisjointSets {
            parent: Vec::with_capacity(size),
            rank: vec![0; size],
            size,
            num_of_sets: size,
        };
        for index in 0..size {
            result.parent.push(index)
        }
        result
    }

    pub fn find(&self, elem: usize) -> usize {
        let root = self.parent[elem];

        if self.parent[root] != elem {
            self.find(root)
        }
        else {
            root
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            Ordering::Less => {
                self.parent[x_root] = y_root;
            }
            Ordering::Greater => {
                self.parent[y_root] = x_root;
            }
            Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }

        self.num_of_sets -= 1;
    }

    pub fn get_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.size];
        for &elem in self.parent.iter() {
            let index = self.find(elem);
            sizes[index] += 1;
        }
        sizes.sort();
        sizes.into_iter().rev().filter(|&x| x != 0).collect()
    }

    pub fn get_number_of_sets(&self) -> usize {
        self.num_of_sets
    }
}