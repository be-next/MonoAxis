use crate::core::lookup_table::M1DLookupTable;

#[allow(dead_code)]
pub struct CA1D {
    pub current_world: Box<Vec<u8>>,
    pub next_world: Box<Vec<u8>>,
    pub lookup_table: M1DLookupTable,
    pub num_states: u8,
    pub num_cells: usize,
}

macro_rules! get_left {
    ($self:ident, $i:ident) => {
        $self.current_world[$i - 1] as usize
    };
}

macro_rules! get_center {
    ($self:ident, $i:ident) => {
        $self.current_world[$i] as usize
    };
}

macro_rules! get_right {
    ($self:ident, $i:ident) => {
        $self.current_world[$i + 1] as usize
    };
}

#[allow(dead_code)]
impl CA1D {
    pub fn new(
        num_states: u8,
        num_cells: usize,
        lookup_table: M1DLookupTable,
    ) -> CA1D {
        let current_world = vec![0; num_cells];
        let next_world = vec![0; num_cells];
        CA1D {
            current_world: Box::new(current_world),
            next_world: Box::new(next_world),
            lookup_table,
            num_states,
            num_cells,
        }
    }

    pub fn get_current_world(&self) -> &Vec<u8> {
        &self.current_world
    }

    pub fn get_next_world(&self) -> &Vec<u8> {
        &self.next_world
    }

    pub fn get_num_states(&self) -> u8 {
        self.num_states
    }

    pub fn get_num_cells(&self) -> usize {
        self.num_cells
    }

    pub fn get_lookup_table(&self) -> &M1DLookupTable {
        &self.lookup_table
    }

    pub fn set_current_world(&mut self, current_world: Vec<u8>) {
        self.current_world = Box::new(current_world);
    }

    pub fn set_next_world(&mut self, next_world: Vec<u8>) {
        self.next_world = Box::new(next_world);
    }

    pub fn set_num_states(&mut self, num_states: u8) {
        self.num_states = num_states;
    }

    pub fn set_num_cells(&mut self, num_cells: usize) {
        self.num_cells = num_cells;
    }

    pub fn set_lookup_table(&mut self, lookup_table: M1DLookupTable) {
        self.lookup_table = lookup_table;
    }

    pub fn step(&mut self) {
        for i in 1..self.num_cells {
            let next_state = self.lookup_table.get(
                get_left!(self, i),
                get_center!(self, i),
                get_right!(self, i));
            self.next_world[i] = *next_state.unwrap() as u8;
        }
        std::mem::swap(&mut self.current_world, &mut self.next_world);
    }

}

