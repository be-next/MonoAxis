
pub struct LookupTable3d<T: Clone> {
    data: Vec<T>,
    right: usize,
    center: usize,
    left: usize,
}

impl<T: Clone> LookupTable3d<T> {
    pub fn new(right: usize, center: usize, left: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; right * center * left],
            right,
            center,
            left,
        }
    }

    pub fn set(&mut self, right: usize, center: usize, left: usize, value: T) {
        let index = self
            .index(right, center, left)
            .unwrap_or_else(||
                panic!("Index out of bounds: right: {}, center: {}, left: {}", right, center, left));
        self.data[index] = value;
    }

    pub fn get(&self, right: usize, center: usize, left: usize) -> &T {
        let index = self
            .index(right, center, left)
            .unwrap_or_else(||
                panic!("Index out of bounds: right: {}, center: {}, left: {}", right, center, left));
        self.data.get(index).unwrap()
    }

    fn index(&self, right: usize, center: usize, left: usize) -> Option<usize> {
        return if right >= self.right || center >= self.center || left >= self.left {
            None
        } else {
            let index = right + center * self.right + left * self.right * self.center;
            Some(index)
        }
    }
}