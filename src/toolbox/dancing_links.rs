pub struct DancingLinks {
    nodes: Vec<Node>,
    column_sizes: Vec<usize>,
    solution: Vec<usize>,
}

#[derive(Clone, Copy)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    column: usize,
    row: usize,
}

impl DancingLinks {
    pub fn new(num_columns: usize) -> Self {
        let mut nodes = Vec::with_capacity(num_columns + 1);
        let column_sizes = vec![0; num_columns + 1];

        for i in 0..=num_columns {
            let node = Node {
                left: if i == 0 { num_columns } else { i - 1 },
                right: if i == num_columns { 0 } else { i + 1 },
                up: i,
                down: i,
                column: i,
                row: 0,
            };
            nodes.push(node);
        }

        DancingLinks {
            nodes,
            column_sizes,
            solution: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row_id: usize, columns: &[usize]) {
        if columns.is_empty() {
            return;
        }

        let mut node_indices = Vec::new();

        for &col in columns {
            let col_header = col + 1;
            let node_idx = self.nodes.len();
            let up_node = self.nodes[col_header].up;

            let node = Node {
                left: 0,
                right: 0,
                up: up_node,
                down: col_header,
                column: col_header,
                row: row_id,
            };

            self.nodes.push(node);
            self.nodes[up_node].down = node_idx;
            self.nodes[col_header].up = node_idx;
            self.column_sizes[col_header] += 1;
            node_indices.push(node_idx);
        }

        for (i, &node_idx) in node_indices.iter().enumerate() {
            let prev_idx = if i == 0 {
                node_indices[node_indices.len() - 1]
            } else {
                node_indices[i - 1]
            };
            let next_idx = if i == node_indices.len() - 1 {
                node_indices[0]
            } else {
                node_indices[i + 1]
            };
            self.nodes[node_idx].left = prev_idx;
            self.nodes[node_idx].right = next_idx;
        }
    }

    fn cover_column(&mut self, col: usize) {
        let right = self.nodes[col].right;
        let left = self.nodes[col].left;
        self.nodes[right].left = left;
        self.nodes[left].right = right;

        let mut i = self.nodes[col].down;
        while i != col {
            let mut j = self.nodes[i].right;
            while j != i {
                let j_down = self.nodes[j].down;
                let j_up = self.nodes[j].up;
                let j_col = self.nodes[j].column;
                self.nodes[j_down].up = j_up;
                self.nodes[j_up].down = j_down;
                self.column_sizes[j_col] -= 1;
                j = self.nodes[j].right;
            }
            i = self.nodes[i].down;
        }
    }

    fn uncover_column(&mut self, col: usize) {
        let mut i = self.nodes[col].up;
        while i != col {
            let mut j = self.nodes[i].left;
            while j != i {
                let j_col = self.nodes[j].column;
                let j_down = self.nodes[j].down;
                let j_up = self.nodes[j].up;
                self.column_sizes[j_col] += 1;
                self.nodes[j_down].up = j;
                self.nodes[j_up].down = j;
                j = self.nodes[j].left;
            }
            i = self.nodes[i].up;
        }
        let right = self.nodes[col].right;
        let left = self.nodes[col].left;
        self.nodes[right].left = col;
        self.nodes[left].right = col;
    }

    fn choose_column(&self) -> Option<usize> {
        let mut min_size = usize::MAX;
        let mut chosen = None;
        let mut col = self.nodes[0].right;

        while col != 0 {
            let size = self.column_sizes[col];
            if size < min_size {
                min_size = size;
                chosen = Some(col);
            }
            col = self.nodes[col].right;
        }

        chosen
    }

    pub fn solve(&mut self) -> bool {
        if self.nodes[0].right == 0 {
            return true;
        }

        let col = match self.choose_column() {
            Some(c) => c,
            None => return false,
        };

        if self.nodes[col].down == col {
            return false;
        }

        self.cover_column(col);

        let mut row = self.nodes[col].down;
        while row != col {
            self.solution.push(self.nodes[row].row);

            let mut j = self.nodes[row].right;
            while j != row {
                self.cover_column(self.nodes[j].column);
                j = self.nodes[j].right;
            }

            if self.solve() {
                return true;
            }

            self.solution.pop();

            let mut j = self.nodes[row].left;
            while j != row {
                self.uncover_column(self.nodes[j].column);
                j = self.nodes[j].left;
            }

            row = self.nodes[row].down;
        }

        self.uncover_column(col);
        false
    }

    pub fn get_solution(&self) -> &[usize] {
        &self.solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_exact_cover() {
        let mut dl = DancingLinks::new(4);

        dl.add_row(0, &[0, 1]);
        dl.add_row(1, &[2, 3]);

        assert!(dl.solve());
        let solution = dl.get_solution();
        assert_eq!(solution.len(), 2);
    }

    #[test]
    fn test_single_row_exact_cover() {
        let mut dl = DancingLinks::new(3);

        dl.add_row(0, &[0, 1, 2]);

        assert!(dl.solve());
        let solution = dl.get_solution();
        assert_eq!(solution.len(), 1);
        assert_eq!(solution[0], 0);
    }

    #[test]
    fn test_no_solution() {
        let mut dl = DancingLinks::new(3);

        dl.add_row(0, &[0, 1]);
        dl.add_row(1, &[1, 2]);

        assert!(!dl.solve());
    }

    #[test]
    fn test_knuth_example() {
        let mut dl = DancingLinks::new(7);

        dl.add_row(0, &[0, 3, 6]);
        dl.add_row(1, &[0, 3]);
        dl.add_row(2, &[3, 4, 6]);
        dl.add_row(3, &[2, 4, 5]);
        dl.add_row(4, &[1, 2, 5, 6]);
        dl.add_row(5, &[1, 6]);

        assert!(dl.solve());
        let solution = dl.get_solution();
        assert_eq!(solution.len(), 3);
    }
}
