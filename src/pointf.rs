use std::ops::{Add, Neg, Div, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Pointf {
    x: f32,
    y: f32
}

impl From<(f32, f32)> for Pointf {
    fn from((x, y): (f32, f32)) -> Pointf {
        Pointf::new(x, y)
    }
}

impl Into<(f32, f32)> for Pointf {
    fn into(self) -> (f32, f32) {
        (self.x(), self.y())
    }
}

impl Pointf {
    /// Creates a new Pointf from the given coordinates.
    pub fn new(x: f32, y: f32) -> Pointf {
        Pointf {
            x: x,
            y: y,
        }
    }

    pub fn slope(&self, rhs: &Pointf) -> f32 {
        (rhs.x() - self.x()) / (rhs.y() - self.y())
    }

    pub fn offset(&self, x: f32, y: f32) -> Pointf {
        Pointf::new(self.x + x, self.y + y)
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Add for Pointf {
    type Output = Pointf;

    fn add(self, rhs: Pointf) -> Pointf {
        self.offset(rhs.x(), rhs.y())
    }
}

impl Neg for Pointf {
    type Output = Pointf;

    fn neg(self) -> Pointf {
        Pointf::new(-self.x(), -self.y())
    }
}

impl Sub for Pointf {
    type Output = Pointf;

    fn sub(self, rhs: Pointf) -> Pointf {
        self.offset(-rhs.x(), -rhs.y())
    }
}

impl Mul<f32> for Pointf {
    type Output = Pointf;

    fn mul(self, rhs: f32) -> Pointf {
        Pointf::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Pointf {
    type Output = Pointf;

    fn div(self, rhs: f32) -> Pointf {
        Pointf::new(self.x() / rhs, self.y() / rhs)
    }
}