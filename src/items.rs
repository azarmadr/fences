use crate::BoardGeom;
use std::fmt;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Fence(pub(crate) Option<bool>);

impl TryFrom<char> for Fence {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'y' | '-' => Ok(Fence(Some(true))),
            'n' | 'x' => Ok(Fence(Some(false))),
            '.' => Ok(Fence(None)),
            _ => Err("Not a valid char for fence"),
        }
    }
}

impl fmt::Debug for Fence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Fence").field(&char::from(*self)).finish()
    }
}

impl From<Fence> for char {
    fn from(value: Fence) -> char {
        match value.0 {
            Some(true) => '-',
            Some(false) => 'x',
            None => '.',
        }
    }
}

use std::ops::{Deref, DerefMut};

macro_rules! deref_impls {
    ($a:ident, $b:path) => {
        impl Deref for $a {
            type Target = $b;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $a {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
deref_impls! {Fence, Option<bool>}

struct Fences {
    cols: usize,
    rows: usize,
    _fences: Vec<Fence>,
}

impl BoardGeom for Fences {
    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    fn rotate(&mut self) {
        unimplemented!()
    }
}
