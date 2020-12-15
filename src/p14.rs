#[derive(Clone, Copy, Debug)]
enum Compass {
    North,
    South,
    East,
    West,
}

impl Compass {
    fn turn_left(&self) -> Self {
        match self {
            Compass::North => Compass::West,
            Compass::West  => Compass::South,
            Compass::South => Compass::East,
            Compass::East  => Compass::North,
        }
    }

    fn turn_left_times(&self, n: usize) -> Self {
        let mut compass = *self;
        for _ in 0 .. (n % 4) {
            compass = compass.turn_left();
        }
        compass
    }
}

#[derive(Clone, Copy, Debug)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Move(Compass, usize),
    Turn(TurnDirection, usize),
    Forward(usize),
}

impl Step {
    fn from_str(s: &str) -> Self {
        let first_char = s.chars().next().unwrap();
        let num: usize = s[1..].parse().unwrap();
        match first_char {
            'N' => Self::Move(Compass::North, num),
            'S' => Self::Move(Compass::South, num),
            'E' => Self::Move(Compass::East, num),
            'W' => Self::Move(Compass::West, num),
            'L' => Self::Turn(TurnDirection::Left, num),
            'R' => Self::Turn(TurnDirection::Right, num),
            'F' => Self::Forward(num),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ship {
    x: i64,
    y: i64,
    facing: Compass,
}

impl Ship {
    fn run_step(&mut self, step: &Step) {
        match step {
            Step::Move(dir, amt) => {
                match dir {
                    Compass::North => self.y += *amt as i64,
                    Compass::South => self.y -= *amt as i64,
                    Compass::East  => self.x += *amt as i64,
                    Compass::West  => self.x -= *amt as i64,
                }
            },
            Step::Turn(td, degrees) => {
                let times = degrees / 90;
                match td {
                    TurnDirection::Left  => self.facing = self.facing.turn_left_times(times),
                    TurnDirection::Right => self.facing = self.facing.turn_left_times(times * 3),
                }
            },
            Step::Forward(times) => {
                self.run_step(&Step::Move(self.facing, *times));
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ShipWithWaypoint {
    ship_x: i64,
    ship_y: i64,
    //offsets
    way_x: i64,
    way_y: i64,
}

impl ShipWithWaypoint {
    fn run_step(&mut self, step: &Step) {
        match step {
            Step::Move(dir, amt) => {
                match dir {
                    Compass::North => self.way_y += *amt as i64,
                    Compass::South => self.way_y -= *amt as i64,
                    Compass::East  => self.way_x += *amt as i64,
                    Compass::West  => self.way_x -= *amt as i64,
                }
            },
            Step::Turn(td, degrees) => {
                let times = degrees / 90;
                let times = times * if let TurnDirection::Left = td {
                    1
                } else {
                    3
                };

                for _ in 0 .. times {
                    let (new_x, new_y) = (-self.way_y, self.way_x);
                    self.way_x = new_x;
                    self.way_y = new_y;
                }

            },
            Step::Forward(times) => {
                self.ship_x += self.way_x * *times as i64;
                self.ship_y += self.way_y * *times as i64;
            },
        }
    }
}

pub fn main() {
    let input = crate::input(14);
    let mut ship = Ship {
        x: 0, y: 0, facing: Compass::East,
    };

    let mut shipwp = ShipWithWaypoint {
        ship_x: 0, ship_y: 0,
        way_x: 10, way_y: 1,
    };

    for step in input.lines().map(Step::from_str) {
        ship.run_step(&step);
        shipwp.run_step(&step);
    }

    dbg!(ship.x.abs() + ship.y.abs());
    dbg!(shipwp.ship_x.abs() + shipwp.ship_y.abs());
}
