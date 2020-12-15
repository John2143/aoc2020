use std::{collections::hash_map::DefaultHasher, fmt::Display, hash::{Hash, Hasher}};

#[derive(Debug, Clone, Copy, Hash)]
enum CellState {
    Floor,
    Empty,
    Full,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            CellState::Empty => 'L',
            CellState::Floor => '.',
            CellState::Full => '#',
        };

        write!(f, "{}", text)
    }
}

impl CellState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Full,
            _ => panic!(),
        }
    }

    fn iterate(&self, neighbors: usize) -> Self {
        if let Self::Floor = self {
            *self
        } else if let Self::Empty = self {
            match neighbors {
                0 => Self::Full,
                _ => *self,
            }
        } else if let Self::Full = self {
            match neighbors {
                5..=8 => Self::Empty,
                _ => *self,
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Hash)]
struct Board(Vec<Vec<CellState>>);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{}", col)?;
            }
        }
        Ok(())
    }
}

impl Board {
    fn from_str(s: &str) -> Self {
        Self(s.lines().map(|row| row.chars().map(CellState::from_char).collect::<Vec<_>>()).collect())
    }

    fn get(&self, x: i64, y: i64) -> CellState {
        if x < 0 || x as usize >= self.0[0].len() || y as usize >= self.0.len() || y < 0 {
            return CellState::Empty;
        }

        self.0[y as usize][x as usize]
    }

    fn neighbors(&self, x: i64, y: i64) -> usize {
        let grid = [
            (-1,  1), (0,  1), (1,  1),
            (-1,  0),          (1,  0),
            (-1, -1), (0, -1), (1, -1),
        ];

        let count = grid.iter().map(|(x_o, y_o)| {
            let (mut cx, mut cy) = (x, y);
            loop {
                cx += x_o;
                cy += y_o;

                let seat = self.get(cx, cy);
                match seat {
                    CellState::Empty => return seat,
                    CellState::Full => return seat,
                    CellState::Floor => {},
                }
            }
        }).filter(|item| match item {
            CellState::Floor => false,
            CellState::Empty => false,
            CellState::Full => true,
        }).count();

        count
    }

    fn iterate(&self) -> Self {
        Board(self.0.iter().enumerate().map(|(y, row)| row.iter().enumerate().map(|(x, col)| {
            col.iterate(self.neighbors(x as i64, y as i64))
        }).collect()).collect())
    }
}

pub fn main(){
    let input = crate::input(13);

    let mut board = Board::from_str(&input);

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    let mut last_hash = 0;

    loop {
        board = board.iterate();

        let new_hash = calculate_hash(&board);

        if new_hash == last_hash {
            let count = board.to_string().chars().filter(|&x| x == '#').count();
            dbg!(&count);
            break;
        } else {
            last_hash = new_hash;
        }
    }
}
