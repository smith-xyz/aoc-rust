pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn offset(&self, row_width: usize) -> i32 {
        match self {
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
            Direction::UP => -(row_width as i32),
            Direction::DOWN => row_width as i32,
            Direction::UpLeft => -(row_width as i32) - 1,
            Direction::UpRight => -(row_width as i32) + 1,
            Direction::DownLeft => row_width as i32 - 1,
            Direction::DownRight => row_width as i32 + 1,
        }
    }

    pub fn all() -> [Direction; 8] {
        [
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub pos: (usize, usize),
}

impl<T> Grid<T> {
    pub fn try_offset_index(&self, flat_index: usize, direction: &Direction) -> Option<usize> {
        let row_width = self.data.len();
        let total_len = self.data[0].len() * row_width;
        let offset = direction.offset(row_width);

        let new_index_i32 = flat_index as i32 + offset;
        if new_index_i32 < 0 || new_index_i32 >= total_len as i32 {
            return None;
        }

        let new_index = new_index_i32 as usize;
        let current_row = flat_index / row_width;
        let current_col = flat_index % row_width;
        let new_row = new_index / row_width;
        let new_col = new_index % row_width;

        let row_diff = (new_row as i32) - (current_row as i32);
        let col_diff = (new_col as i32) - (current_col as i32);

        if row_diff.abs() > 1 || col_diff.abs() > 1 {
            return None;
        }

        Some(new_index)
    }

    pub fn flat(&self) -> Vec<&T> {
        self.data.iter().flatten().collect()
    }

    pub fn get_all_neighbor_indices(&self, flat_index: usize) -> Vec<usize> {
        Direction::all()
            .iter()
            .filter_map(|dir| self.try_offset_index(flat_index, dir)) // dir, not offset
            .collect()
    }

    pub fn get_mut_by_index(&mut self, flat_index: usize) -> Option<&mut T> {
        let row_width = self.data.len();
        let row = flat_index / row_width;
        let col = flat_index % row_width;
        self.data.get_mut(row)?.get_mut(col)
    }

    pub fn set_by_index(&mut self, flat_index: usize, value: T) -> Option<()> {
        let row_width = self.data.len();
        let row = flat_index / row_width;
        let col = flat_index % row_width;
        *self.data.get_mut(row)?.get_mut(col)? = value;
        Some(())
    }

    pub fn get_by_index(&self, flat_index: usize) -> Option<&T> {
        let row_width = self.data.len();
        let row = flat_index / row_width;
        let col = flat_index % row_width;
        self.data.get(row)?.get(col)
    }
}
