#[derive(Debug, PartialEq)]
pub struct LookupTable3d<T: Copy> {
    data: Vec<T>,
    right: usize,
    center: usize,
    left: usize,
}

#[derive(Debug, PartialEq)]
pub enum LookupTable3dError {
    IndexOutOfBounds,
}

impl<T: Copy> LookupTable3d<T> {
    pub fn new(right: usize, center: usize, left: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; right * center * left],
            right,
            center,
            left,
        }
    }

    pub fn set(&mut self, right: usize, center: usize, left: usize, value: T) -> Result<&mut Self, LookupTable3dError> {
        let index = match self.index(right, center, left) {
            Some(i) => i,
            None => return Err(LookupTable3dError::IndexOutOfBounds),
        };

        self.data[index] = value;
        Ok(self)
    }

    pub fn get(&self, right: usize, center: usize, left: usize) -> Result<&T, LookupTable3dError> {
        let index = match self.index(right, center, left) {
            Some(i) => i,
            None => return Err(LookupTable3dError::IndexOutOfBounds),
        };

       Ok(self.data.get(index).unwrap())
    }

    fn index(&self, right: usize, center: usize, left: usize) -> Option<usize> {
        return if right >= self.right || center >= self.center || left >= self.left {
            None
        } else {
            let index = right + center * self.right + left * self.right * self.center;
            Some(index)
        };
    }

    pub fn collection_size(&self) -> usize {
        self.data.len()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item=(usize, usize, usize)> {
        let right = self.right;
        let center = self.center;
        let left = self.left;

        (0..right).flat_map(move |r| {
            (0..center).flat_map(move |c| {
                (0..left).map(move |l| (r, c, l))
            })
        })
    }
}