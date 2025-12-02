use crate::utils::{file_reader::FileReader, solver::Solver};

pub struct Day01 {
    data: String,
}

struct CombinationSafe {
    pos: i16,
    land_zero_clicks: i16,
    rotated_zero_clicks: i16,
}

impl Default for CombinationSafe {
    fn default() -> Self {
        CombinationSafe {
            pos: 50,
            land_zero_clicks: 0,
            rotated_zero_clicks: 0,
        }
    }
}

impl CombinationSafe {
    fn process_dials(&mut self, dials: Vec<&str>) {
        for next in dials {
            let (direction, amount) = next.split_at(1);
            let n = amount.parse().unwrap();
            match direction {
                "L" => self.dial_left(n),
                "R" => self.dial_right(n),
                _ => panic!("PARSING ERROR, did not find directional assignemnt of R or L"),
            }
        }
    }

    fn dial_left(&mut self, mut n: i16) {
        while n != 0 {
            self.pos -= 1;
            n -= 1;
            match self.pos {
                0 => {
                    if n != 0 {
                        self.rotated_zero_clicks += 1;
                    }
                }
                -1 => {
                    self.pos = 99;
                }
                _ => continue,
            };
        }

        if self.pos == 0 {
            self.land_zero_clicks += 1;
        }
    }

    fn dial_right(&mut self, mut n: i16) {
        while n != 0 {
            self.pos += 1;
            n -= 1;
            match self.pos {
                100 => {
                    self.pos = 0;
                    if n != 0 {
                        self.rotated_zero_clicks += 1;
                    }
                }
                _ => continue,
            };
        }

        if self.pos == 0 {
            self.land_zero_clicks += 1;
        }
    }
}

impl Solver for Day01 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        Ok(Day01 { data })
    }

    fn part_one_solution(&self) -> u32 {
        let mut combination_safe: CombinationSafe = CombinationSafe::default();
        let dials: Vec<&str> = self.data.split("\n").collect();
        combination_safe.process_dials(dials);
        combination_safe.land_zero_clicks as u32
    }

    fn part_two_solution(&self) -> u32 {
        let mut combination_safe: CombinationSafe = CombinationSafe::default();
        let dials: Vec<&str> = self.data.split("\n").collect();
        combination_safe.process_dials(dials);
        combination_safe.rotated_zero_clicks as u32 + combination_safe.land_zero_clicks as u32
    }
}
