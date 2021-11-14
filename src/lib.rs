#[derive(Debug, Clone, Copy)]
pub struct Bruch {
    zähler: usize,
    nenner: usize,
}

fn ggt(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl Bruch {
    pub fn new(zähler: usize, nenner: usize) -> Self {
        let ggt = ggt(zähler, nenner);

        Self {
            zähler: zähler / ggt,
            nenner: nenner / ggt,
        }
    }
}

use std::{fmt, ops};

impl ops::Add<Bruch> for Bruch {
    type Output = Bruch;

    fn add(self, other: Bruch) -> Bruch {
        if self.nenner == other.nenner {
            Bruch {
                zähler: self.zähler + other.zähler,
                nenner: self.nenner,
            }
        } else {
            Bruch::new(
                self.zähler * other.nenner + other.zähler * self.nenner,
                self.nenner * other.nenner,
            )
        }
    }
}

impl ops::AddAssign<Bruch> for Bruch {
    fn add_assign(&mut self, other: Bruch) {
        let result = *self + other;
        self.zähler = result.zähler;
        self.nenner = result.nenner;
    }
}

impl ops::Mul<Bruch> for Bruch {
    type Output = Bruch;

    fn mul(self, other: Bruch) -> Bruch {
        Bruch::new(self.zähler * other.zähler, self.nenner * other.nenner)
    }
}

impl From<usize> for Bruch {
    fn from(zahl: usize) -> Bruch {
        Bruch {
            zähler: zahl,
            nenner: 1,
        }
    }
}

impl fmt::Display for Bruch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ggt = ggt(self.zähler, self.nenner);
        write!(f, "{}/{}", self.zähler / ggt, self.nenner / ggt)
    }
}
