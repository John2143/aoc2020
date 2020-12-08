use std::str::FromStr;

#[derive(Debug)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct CPU {
    a: i32,
    pc: usize,
    program: Vec<Instr>,
}

impl CPU {
    fn step(&mut self) {
        match self.program[self.pc] {
            Instr::Acc(n) => {
                self.a += n;
                self.pc += 1;
            },
            Instr::Nop(_) => {
                self.pc += 1;
            },
            Instr::Jmp(n) => {
                if n > 0 {
                    self.pc += n as usize;
                } else {
                    self.pc -= (-n) as usize;
                }
            },
        }
    }

    fn is_valid(&mut self) -> bool {
        let mut seen = vec![false; self.program.len()];

        loop {
            if self.pc >= self.program.len() {
                return true;
            } else if seen[self.pc] {
                return false;
            } else {
                seen[self.pc] = true;
                self.step();
            }
        }
    }
}

impl FromStr for Instr {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        let left = s.next().unwrap();
        let right = s.next().unwrap();

        Ok(match left {
            "nop" => Instr::Nop(right.parse().unwrap()),
            "acc" => Instr::Acc(right.parse().unwrap()),
            "jmp" => Instr::Jmp(right.parse().unwrap()),
            _ => panic!(),
        })
    }
}

impl Instr {
    fn swap(&mut self) {
        *self = match *self {
            Instr::Nop(n) => Instr::Jmp(n),
            Instr::Jmp(n) => Instr::Nop(n),
            Instr::Acc(n) => Instr::Acc(n),
        }
    }
}

pub fn main(){
    let mut cpu = CPU {
        program: crate::input(10).lines().map(|x| x.parse::<Instr>().unwrap()).collect(),
        a: 0,
        pc: 0,
    };

    dbg!(&cpu.is_valid());
    println!("a: {}", cpu.a);

    for i in 0..cpu.program.len() {
        cpu.a = 0;
        cpu.pc = 0;

        cpu.program[i].swap();
        if cpu.is_valid() {
            println!("valid {}, a is {}", i, cpu.a);
        }
        cpu.program[i].swap();
    }
}
