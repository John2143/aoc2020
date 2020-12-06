struct Map<'a>(Vec<&'a str>);

struct SlopeIter<'a> {
    map: &'a Map<'a>,
    dx: usize,
    dy: usize,
    cx: usize,
    cy: usize,
}

impl Iterator for SlopeIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.map.xy(self.cx, self.cy);
        self.cx += self.dx;
        self.cy += self.dy;

        char
    }
}

impl<'a> Map<'a> {
    fn new(s: &'a str) -> Self {
        Self(s.lines().collect())
    }

    fn xy(&self, x: usize, y: usize) -> Option<char> {
        let line = self.0.get(y)?;
        let xc = x % line.len();
        for (i, c) in line.chars().enumerate() {
            use colour::*;
            match (i, c) {
                (q, _) if q == xc => red!("X"),
                (_, '.') => yellow!("."),
                (_, '#') => yellow!("#"),
                (_, _) => unimplemented!(),

            }
        }

        println!();
        line.chars().nth(xc)
    }

    fn slope_iter(&'a self, dx: usize, dy: usize) -> SlopeIter<'a> {
        SlopeIter {
            map: &self,
            dx, dy,
            cx: 0, cy: 0,
        }
    }
}

pub fn main(){
    let input = crate::input(5);
    let map = Map::new(&input);

    let slopes = [
        (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)
    ];

    let sum_slope: usize = slopes.iter().map(|(xslope, yslope)| {
        let path = map.slope_iter(*xslope, *yslope).filter(|&x| x == '#').count();
        println!("{}, {} = {}", xslope, yslope, path);
        path
    }).product();

    println!("{}", &sum_slope);
}
