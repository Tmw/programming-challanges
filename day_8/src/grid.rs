#[derive(Debug)]
pub struct Grid<T: Clone> {
    data: Vec<T>,
    pub x: usize,
    pub y: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(x: usize, y: usize, initial: T) -> Grid<T> {
        Grid::<T> {
            x: x,
            y: y,
            data: vec![initial; x * y],
        }
    }

    fn get_value(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(self.index_for_coords(x, y))
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: T) {
        let index = self.index_for_coords(x, y);
        self.data[index] = value
    }

    fn index_for_coords(&self, x: usize, y: usize) -> usize {
        x + &self.x * y
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn get_column(&self, col: usize) -> Option<Vec<T>> {
        if col >= self.x {
            return None;
        }

        let data = &self.data[..];

        let col: Vec<T> = data.chunks(self.x)
            .map(|row| row[col].to_owned())
            .collect();

        Some(col)
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<T>> {
        let data = &self.data[..];
        data.chunks(self.x).nth(row).map(|i| i.to_owned())
    }

    pub fn set_row(&mut self, row: usize, data: Vec<T>) {
        for (idx, v) in data.into_iter().enumerate() {
            &self.set_value(idx, row, v);
        }
    }

    pub fn set_column(&mut self, column: usize, data: Vec<T>) {
        for (idx, v) in data.into_iter().enumerate() {
            &self.set_value(column, idx, v);
        }
    }
}

#[cfg(test)]
mod grid_test {
    use super::*;

    #[test]
    fn it_should_create_grid_with_correct_dimensions() {
        let grid = Grid::new(5, 10, false);
        assert_eq!(grid.data.len(), 5 * 10);
    }

    #[test]
    fn it_should_default_to_all_pixels_off() {
        let grid = Grid::new(2, 2, false);
        assert_eq!(grid.data, vec![false; 4]);
    }

    #[test]
    fn it_should_get_a_value() {
        let mut grid = Grid::new(2, 2, false);
        assert_eq!(grid.get_value(1, 1), Some(&false));

        grid.data[1] = true;
        assert_eq!(grid.get_value(1, 0), Some(&true));

        grid.data[3] = true;
        assert_eq!(grid.get_value(1, 1), Some(&true));
    }

    #[test]
    fn it_should_set_a_value() {
        let mut grid = Grid::new(5, 5, false);
        grid.set_value(2, 2, true);

        let example = vec![false, false, false, false, false, false, false, false, false, false,
                           false, false, true, false, false, false, false, false, false, false,
                           false, false, false, false, false];

        assert_eq!(grid.data, example);
    }

    #[test]
    fn test_get_column() {
        let example = vec![false, false, false, false, false, false, false, false, false, false,
                           false, false, true, false, false, false, false, false, false, false,
                           false, false, false, false, false];

        let grid = Grid {
            x: 5,
            y: 5,
            data: example,
        };
        let expected = vec![false, false, true, false, false];
        assert_eq!(grid.get_column(2), Some(expected));

        assert_eq!(grid.get_column(16), None);
        assert_eq!(grid.get_column(5), None);
    }

    #[test]
    fn test_get_row() {
        let example = vec![false, false, false, false, false, false, false, false, false, false,
                           false, true, false, true, false, false, false, false, false, false,
                           false, false, false, false, false];

        let grid = Grid {
            x: 5,
            y: 5,
            data: example,
        };

        let expected = vec![false, true, false, true, false];
        assert_eq!(grid.get_row(2), Some(expected));

        assert_eq!(grid.get_row(16), None);
        assert_eq!(grid.get_row(5), None);
    }

    #[test]
    fn test_set_row() {
        let example = vec![false; 5*5];
        let mut grid = Grid {
            x: 5,
            y: 5,
            data: example,
        };
        grid.set_row(2, vec![true; 5]);

        let expected = vec![false, false, false, false, false, false, false, false, false, false,
                            true, true, true, true, true, false, false, false, false, false,
                            false, false, false, false, false];

        assert_eq!(grid.data, expected);
    }

    #[test]
    fn test_set_column() {
        let example = vec![false; 5*5];
        let mut grid = Grid {
            x: 5,
            y: 5,
            data: example,
        };
        grid.set_column(3, vec![true; 5]);

        let expected = vec![false, false, false, true, false, false, false, false, true, false,
                            false, false, false, true, false, false, false, false, true, false,
                            false, false, false, true, false];

        assert_eq!(grid.data, expected);
    }
}
