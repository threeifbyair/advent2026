use crate::Args;
use std::io;

struct IntComplex {
    real: i64,
    imag: i64,
}

impl IntComplex {
    pub const fn new(real: i64, imag: i64) -> IntComplex {
        IntComplex { real, imag }
    }

    pub fn add(&self, other: &IntComplex) -> IntComplex {
        IntComplex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn multiply(&self, other: &IntComplex) -> IntComplex {
        IntComplex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    pub fn divide(&self, other: &IntComplex) -> IntComplex {
        IntComplex {
            real: self.real / other.real,
            imag: self.imag / other.imag,
        }
    }
}

impl std::fmt::Display for IntComplex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.real, self.imag)
    }
}

const TENTEN: IntComplex = IntComplex::new(10, 10);
const HUNDREDK: IntComplex = IntComplex::new(100000, 100000);

fn perform_cycle(current: &IntComplex, a: &IntComplex, factor: &IntComplex) -> IntComplex {
    current.multiply(current).divide(factor).add(a)
}

pub fn puzzle(data: &[&str], args: &Args) -> Result<String, io::Error> {
    let splitline: Vec<&str> = data[0].split('[').collect();
    let complex: Vec<&str> = splitline[1].split(']').collect();
    let values: Vec<i64> = complex[0].split(',').map(|s| s.trim().parse().unwrap()).collect();
    let a = IntComplex::new(values[0], values[1]);
    let mut current = IntComplex::new(0, 0);
    let mut points: i64 = 0;
    if args.part_two || args.part_three {
        let limit = if args.part_three { 1001 } else { 101 };
        let factor = if args.part_three { 1 } else { 10 };
        for x in 0..limit {
            for y in 0..limit {
                let this_a = IntComplex::new(x*factor, y*factor).add(&a);
                let mut should_engrave = true;
                current.real = 0;
                current.imag = 0;
                for _ in 0..100 {
                    current = perform_cycle(&current, &this_a, &HUNDREDK);
                    if current.real < -1000000 || current.real > 1000000 || current.imag < -1000000 || current.imag > 1000000 {
                        should_engrave = false;
                        break;
                    }
                }
                //println!("x = {x}, y = {y}, this_a = {}, should_engrave = {}", this_a, should_engrave);
                if should_engrave {
                    points += 1;
                }
            }
        }
    }
    else {
        for _ in 0..3 {
            current = perform_cycle(&current, &a, &TENTEN);
        }
    }

    Ok(
        if args.part_two || args.part_three {
            format!("Points: {}", points)
        }
        else {
            format!("Current: {}", current)
        }
    )
}
