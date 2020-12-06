use std::str::FromStr;

struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn seat_id(&self) -> usize { 
        usize::from(self.row) * 8 + usize::from(self.col)
    }
}

impl FromStr for Seat {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rows, cols) = s.split_at(7);

        let mut seat = Self {
            row: 0, col: 0,
        };

        for (mask, rowchar) in rows.chars().enumerate() {
            let set = match rowchar {
                'B' => 1,
                'F' => 0,
                _ => panic!(),
            };

            //X01234567

            seat.row |= set << (6 - mask);
        }

        for (mask, colchar) in cols.chars().enumerate() {
            let set = match colchar {
                'R' => 1,
                'L' => 0,
                _ => panic!(),
            };

            //XXXXX012
            seat.col |= set << (2 - mask);
        }

        //println!("{}, {}", rows, cols);

        assert!(seat.row < 128);
        assert!(seat.col < 8);

        Ok(seat)
    }
}

pub fn main(){
    let passes = crate::input(7);

    for pass in passes.lines() {
        println!("{}", pass.parse::<Seat>().unwrap().seat_id());
    }
}
