use std::iter::Peekable;

#[derive(Default)]
struct Answers([bool; 26]);

impl Answers {
    fn from_input<'a, 'b, I>(input: &'a mut Peekable<I>) -> Option<Self>
    where
        I: Iterator<Item=&'b str>,
    {

        let mut group = Self([true; 26]);

        if input.peek().is_none() {
            return None;
        }

        for line in input {
            if line == "" {
                break;
            }

            let mut person = Self::default();

            for c in line.chars() {
                let m = match c {
                    'a'..='z' => (c as u8) - b'a',
                    _ => unimplemented!(),
                };

                assert!(m < 26);

                person.0[m as usize] = true;
            }

            group.0.iter_mut().zip(&person.0).for_each(|(gr, p)| *gr &= p);
        }

        Some(group)
    }

    fn sum(&self) -> usize {
        self.0
            .iter()
            .fold(0, |acc, x| if *x { acc + 1 } else { acc })
    }
}

pub fn main() {
    let input = crate::input(8);
    let mut lines = input.lines().peekable();

    let mut total_sum = 0;

    while let Some(g) = Answers::from_input(&mut lines) {
        total_sum += g.sum();
    }

    println!("{}", total_sum);
}
