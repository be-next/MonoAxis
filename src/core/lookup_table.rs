use std as sdt;
use serde::{Deserialize, Serialize};

// This lookup table is used to map the neighborhood of a cell to its next state.
// The neighborhood is represented by a tuple of three values: (left, center, right).
// The next state is represented by a single value.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct M1DLookupTable {
    data: Vec<i8>,
    num_states: i8,
}

#[derive(Debug, PartialEq)]
pub enum M1DLookupTableError {
    IndexOutOfBounds,
}

macro_rules! get_index_or_err {
    ($self:ident, $left:ident, $center:ident, $right:ident) => {
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
    pub fn new(num_states: i8, default_value: i8) -> Self {
        Self {
            data: vec![default_value; (num_states * num_states * num_states) as usize],
            num_states,
        }
    }

    pub fn set(&mut self, left: i8, center: i8, right: i8, value: i8) -> Result<&mut Self, M1DLookupTableError> {
        let index = get_index_or_err!(self, left, center, right);
        self.data[index as usize] = value;
        Ok(self)
    }

    pub fn get(&self, left: i8, center: i8, right: i8) -> Result<&i8, M1DLookupTableError> {
        let index = get_index_or_err!(self, left, center, right);
        Ok(self.data.get(index as usize).unwrap())
    }

    fn index(&self, left: i8, center: i8, right: i8) -> Option<i8> {
        return if left >= self.num_states || center >= self.num_states || right >= self.num_states {
            None
        } else {
            let index = left + center * self.num_states + right * self.num_states * self.num_states;
            Some(index)
        };
    }

    pub fn collection_size(&self) -> usize {
        self.data.len()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = (i8, i8, i8)> {
        let left = self.num_states;
        let center = self.num_states;
        let right = self.num_states;

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
        let indices_to_modify: Vec<(i8, i8, i8)> = self
            .iter_indices()
            .filter(|(r, c, l)| self.get(*r, *c, *l).unwrap() == &value_to_replace)
            .collect();

        for (r, c, l) in indices_to_modify {
            let _ = self.set(r, c, l, c).unwrap();
        }

        self
    }
}
