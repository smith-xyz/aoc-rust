use crate::{
    solver::solver::Solver,
    toolbox::grid::{Direction, Grid},
    utils::file_reader::FileReader,
};

pub struct Day04 {
    grid: Grid<String>,
}

impl Solver<u32> for Day04 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let row_cols: Vec<Vec<String>> = data
            .split("\n")
            .map(|row| row.chars().map(|f| f.to_string()).collect())
            .collect();

        Ok(Day04 {
            grid: Grid::<String> {
                pos: (0, 0),
                data: row_cols,
            },
        })
    }

    fn part_one_solution(&mut self) -> u32 {
        let flat: Vec<&String> = self.grid.flat();

        self.grid
            .flat()
            .iter()
            .enumerate()
            .filter(|(idx, value)| {
                value.as_str() == "@" && {
                    let neighbor_count = Direction::all()
                        .iter()
                        .filter_map(|dir| self.grid.try_offset_index(*idx, dir)) // dir is a Direction
                        .filter_map(|neighbor_idx| flat.get(neighbor_idx))
                        .filter(|s| s.as_str() == "@")
                        .count();
                    neighbor_count < 4
                }
            })
            .count() as u32
    }

    fn part_two_solution(&mut self) -> u32 {
        let row_width = self.grid.data.len();
        let total_len = self.grid.data[0].len() * row_width;
        let mut total = 0;
        let mut changed = true;

        while changed {
            changed = false;
            let mut to_change = Vec::new();

            for idx in 0..total_len {
                if let Some(cell) = self.grid.get_by_index(idx) {
                    if cell.as_str() == "@" {
                        let neighbor_count = Direction::all()
                            .iter()
                            .filter_map(|dir| self.grid.try_offset_index(idx, dir))
                            .filter_map(|neighbor_idx| self.grid.get_by_index(neighbor_idx))
                            .filter(|s| s.as_str() == "@")
                            .count();

                        if neighbor_count < 4 {
                            to_change.push(idx);
                        }
                    }
                }
            }

            for idx in to_change {
                if let Some(cell) = self.grid.get_mut_by_index(idx) {
                    *cell = "x".to_string();
                    total += 1;
                    changed = true;
                }
            }
        }

        total
    }
}
