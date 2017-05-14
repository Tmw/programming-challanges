use rotation::Rotation;
use target::Target;
use dimension::Dimension;
use std;

#[derive(Debug, PartialEq)]
pub enum Command {
    Draw(Dimension),
    Rotate(Rotation),
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut args = s.split_whitespace();
        match args.next() {

            Some("rect") => {
                // draw rect command
                if let Some(dim) = args.next() {
                    Ok(Command::Draw(dim.parse().unwrap()))
                } else {
                    Err(())
                }
            }

            Some("rotate") => {
                // rotate command

                let row_or_column: &str = args.nth(0).unwrap();
                let target: usize = args.nth(0).unwrap().split('=').last().unwrap().parse().unwrap();
                let rotation: usize = args.nth(1).unwrap().parse().unwrap();

                let target: Option<Target> = match row_or_column {
                    "row" => Some(Target::Row(target)),
                    "column" => Some(Target::Column(target)),
                    _ => None,
                };

                if let Some(target) = target {
                    Ok(Command::Rotate(Rotation {
                        target: target,
                        rotation: rotation,
                    }))
                } else {
                    Err(())
                }

            }

            Some(_) | None => Err(()),
        }

    }
}

#[cfg(test)]
mod command_test {
    use command::Command;
    use target::Target;
    use dimension::Dimension;
    use rotation::Rotation;

    #[test]
    fn it_creates_valid_draw_command() {
        let cmd = "rect 1x1".parse();
        let res = Ok(Command::Draw(Dimension { x: 1, y: 1 }));
        assert_eq!(cmd, res);

        let cmd = "rect 12x37".parse();
        let res = Ok(Command::Draw(Dimension { x: 12, y: 37 }));
        assert_eq!(cmd, res);
    }

    #[test]
    fn it_creates_valid_rotate_command() {
        let cmd: Command = "rotate row y=0 by 3".parse().unwrap();
        let res = Command::Rotate(Rotation {
            target: Target::Row(0),
            rotation: 3,
        });

        assert_eq!(cmd, res);

        let cmd: Command = "rotate column y=4 by 6".parse().unwrap();
        let res = Command::Rotate(Rotation {
            target: Target::Column(4),
            rotation: 6,
        });

        assert_eq!(cmd, res);
    }
}
