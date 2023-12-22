use std as sdt;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct M1DLookupTable {
    data: Vec<i8>,
    left: usize,
    center: usize,
    right: usize,
}

#[derive(Debug, PartialEq)]
pub enum M1DLookupTableError {
    IndexOutOfBounds,
}

macro_rules! get_index_or_err {
    ($self:ident, $left:expr, $center:expr, $right:expr) => {
        match $self.index($left, $center, $right) {
            Some(i) => i,
            None => return Err(M1DLookupTableError::IndexOutOfBounds),
        }
    };
}

impl sdt::fmt::Display for M1DLookupTable {
    fn fmt(&self, f: &mut sdt::fmt::Formatter<'_>) -> sdt::fmt::Result {
        let mut result = String::new();
        for (r, c, l) in self.iter_indices() {
            let value = self.get(r, c, l).unwrap();
            result.push_str(&format!("({}, {}, {}) -> {}\n", r, c, l, value));
        }
        write!(f, "{}", result)
    }
}

impl M1DLookupTable {
    pub fn new(left: usize, center: usize, right: usize, default_value: i8) -> Self {
        Self {
            data: vec![default_value; left * center * right],
            left,
            center,
            right,
        }
    }

    pub fn set(
        &mut self,
        left: usize,
        center: usize,
        right: usize,
        value: i8,
    ) -> Result<&mut Self, M1DLookupTableError> {
        let index = get_index_or_err!(self, left, center, right);
        self.data[index] = value;
        Ok(self)
    }

    pub fn get(&self, left: usize, center: usize, right: usize) -> Result<&i8, M1DLookupTableError> {
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

    pub fn iter_indices(&self) -> impl Iterator<Item = (usize, usize, usize)> {
        let left = self.left;
        let center = self.center;
        let right = self.right;

        (0..left)
            .flat_map(move |r| (0..center)
                .flat_map(move |c| (0..right)
                    .map(move |l| (r, c, l))))
    }

    pub fn replace_values(&mut self, from_value: i8, to_value: i8) -> &mut Self {
        self.data.iter_mut().for_each(|v| {
            if *v == from_value {
                *v = to_value;
            }
        });
        self
    }

    pub fn finalize(&mut self, value_to_replace: i8) -> &mut Self {
        let indices_to_modify: Vec<(usize, usize, usize)> = self
            .iter_indices()
            .filter(|(r, c, l)| self.get(*r, *c, *l).unwrap() == &value_to_replace)
            .collect();

        for (r, c, l) in indices_to_modify {
            let _ = self.set(r, c, l, c as i8).unwrap();
        }

        self
    }
}
