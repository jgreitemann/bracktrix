use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Point,
    pub rotation: Rotation,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translation: Point::zero(),
            rotation: Rotation::Deg0,
        }
    }

    pub fn shifted_by(&self, delta: &Point) -> Self {
        Self {
            translation: self.translation + *delta,
            rotation: self.rotation,
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        use Rotation::*;
        Self {
            translation: self.translation,
            rotation: match self.rotation {
                Deg0 => Deg90,
                Deg90 => Deg180,
                Deg180 => Deg270,
                Deg270 => Deg0,
            },
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        use Rotation::*;
        Self {
            translation: self.translation,
            rotation: match self.rotation {
                Deg0 => Deg270,
                Deg90 => Deg0,
                Deg180 => Deg90,
                Deg270 => Deg180,
            },
        }
    }
}

pub struct Render {
    pub console: usize,
    pub z_order: usize,
    pub viewport: Rect,
}

pub struct Pixels(pub Vec<Pixel>);
