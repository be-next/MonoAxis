
struct LookupTable3d<T: Clone> {
    table: Vec<T>,
    right: usize,
    center: usize,
    left: usize,
}

impl<T: Clone> LookupTable3d<T> {
    fn new(right: usize, center: usize, left: usize, default_value: T) -> Self {
        Self {
            table: vec![default_value; right * center * left],
            right,
            center,
            left,
        }
    }

    fn set(&mut self, right: usize, center: usize, left: usize, value: T) {
        let index = self.index(right, center, left);
        self.table[index] = value;
    }

    fn get(&self, right: usize, center: usize, left: usize) -> T {
        let index = self.index(right, center, left);
        self.table[index].clone()
    }

    //TODO: make this private
    //TODO: fix index calculation

    fn index(&self, right: usize, center: usize, left: usize) -> usize {
        let index = right * self.right + center * self.center + left * self.left;
        index
    }
}