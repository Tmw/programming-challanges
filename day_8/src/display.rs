use command::Command;
use dimension::Dimension;
use rotation::Rotation;
use target::Target;
use grid::Grid;

use utils::rotate_vec;

pub struct Display {
    grid: Grid<bool>,
}

impl Display {
    pub fn new(rows: usize, columns: usize) -> Self {
        Display { grid: Grid::new(rows, columns, false) }
    }

    fn draw(&mut self, dimension: Dimension) {
        for x in 0..dimension.x {
            for y in 0..dimension.y {
                self.grid.set_value(x, y, true);
            }
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        match rotation.target {
            Target::Row(r) => {
                let row: &Vec<bool> = &self.grid.get_row(r).unwrap();
                let result = rotate_vec(rotation.rotation, row);
                self.grid.set_row(r, result);
            }

            Target::Column(c) => {
                let column: &Vec<bool> = &self.grid.get_column(c).unwrap();
                let result = rotate_vec(rotation.rotation, column);
                self.grid.set_column(c, result);
            }
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Draw(dim) => self.draw(dim),
            Command::Rotate(rot) => self.rotate(rot),
        }
    }

    pub fn pixels_lit(&self) -> usize {
        let data = &self.grid.to_vec();
        let res: Vec<&bool> = data.iter().filter(|px| **px).collect();
        res.len()
    }
}

#[cfg(test)]
mod display_tests {
    use super::Display;

    use command::Command;
    use dimension::Dimension;
    use target::Target;
    use rotation::Rotation;

    #[test]
    fn it_should_execute_draw_command() {
        let mut display = Display::new(4, 4);
        let cmd = Command::Draw(Dimension { x: 2, y: 2 });
        display.execute(cmd);

        let expected = vec![true, true, false, false, true, true, false, false, false, false,
                            false, false, false, false, false, false];

        assert_eq!(display.grid.to_vec(), expected);
    }

    #[test]
    fn it_should_execute_rotate_command_row() {
        let mut display = Display::new(4, 4);
        let draw_cmd = Command::Draw(Dimension { x: 1, y: 1 });
        let rotate_cmd = Command::Rotate(Rotation {
            target: Target::Row(0),
            rotation: 1,
        });

        display.execute(draw_cmd);
        display.execute(rotate_cmd);

        let expected = vec![false, true, false, false, false, false, false, false, false, false,
                            false, false, false, false, false, false];

        assert_eq!(display.grid.to_vec(), expected);
    }

    #[test]
    fn it_should_execute_rotate_command_column() {

        let mut display = Display::new(4, 4);
        let draw_cmd = Command::Draw(Dimension { x: 1, y: 1 });
        let rotate_cmd = Command::Rotate(Rotation {
            target: Target::Column(0),
            rotation: 1,
        });

        display.execute(draw_cmd);
        display.execute(rotate_cmd);

        let expected = vec![false, false, false, false, true, false, false, false, false, false,
                            false, false, false, false, false, false];

        assert_eq!(display.grid.to_vec(), expected);
    }

    #[test]
    fn test_pixels_lit() {
        let mut display = Display::new(4, 4);

        display.execute(Command::Draw(Dimension { x: 1, y: 1 }));
        assert_eq!(display.pixels_lit(), 1);

        display.execute(Command::Draw(Dimension { x: 2, y: 2 }));
        assert_eq!(display.pixels_lit(), 4);
    }

}
