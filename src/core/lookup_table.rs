
struct LookupTable3d<T: Clone> {
    table: Vec<T>,
    right: usize,
    center: usize,
    left: usize,
}

impl<T: Clone> LookupTable3d<T> {
    fn new(table: Vec<T>, right: usize, center: usize, left: usize) -> Self {
        Self {
            table,
            right,
            center,
            left,
        }
    }

    fn get(&self, right: usize, center: usize, left: usize) -> T {
        let index = self.index(right, center, left);
        self.table[index].clone()
    }

    fn index(&self, right: usize, center: usize, left: usize) -> usize {
        let index = right * self.right + center * self.center + left * self.left;
        index
    }
}