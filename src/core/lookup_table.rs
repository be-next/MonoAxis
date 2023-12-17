#[derive(Debug, PartialEq)]
pub struct LookupTable3d<T: Copy + PartialEq> {
    data: Vec<T>,
    left: usize,
    center: usize,
    right: usize,
}

#[derive(Debug, PartialEq)]
pub enum LookupTable3dError {
    IndexOutOfBounds,
}

macro_rules! get_index_or_err {
    ($self:ident, $left:expr, $center:expr, $right:expr) => {
        match $self.index($left, $center, $right) {
            Some(i) => i,
            None => return Err(LookupTable3dError::IndexOutOfBounds),
        }
    };
}

impl<T: Copy + PartialEq> LookupTable3d<T> {
    pub fn new(left: usize, center: usize, right: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; left * center * right],
            left,
            center,
            right,
        }
    }

    pub fn set(&mut self, left: usize, center: usize, right: usize, value: T) -> Result<&mut Self, LookupTable3dError> {
        let index = get_index_or_err!(self, left, center, right);
        self.data[index] = value;
        Ok(self)
    }

    pub fn get(&self, left: usize, center: usize, right: usize) -> Result<&T, LookupTable3dError> {
        let index = get_index_or_err!(self, left, center, right);
       Ok(self.data.get(index).unwrap())
    }

    fn index(&self, left: usize, center: usize, right: usize) -> Option<usize> {
        return if left >= self.left || center >= self.center || right >= self.right {
            None
        } else {
            let index = left + center * self.left + right * self.left * self.center;
            Some(index)
        };
    }

    pub fn collection_size(&self) -> usize {
        self.data.len()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item=(usize, usize, usize)> {
        let left = self.left;
        let center = self.center;
        let right = self.right;

        (0..left).flat_map(move |r| {
            (0..center).flat_map(move |c| {
                (0..right).map(move |l| (r, c, l))
            })
        })
    }

    pub fn replace_values(&mut self, from_value: T, to_value: T) -> &mut Self {
        self.data.iter_mut().for_each(|v| {
            if *v == from_value {
                *v = to_value;
            }
        });
        self
    }
}