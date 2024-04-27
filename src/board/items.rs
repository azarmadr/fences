use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct U2(pub(crate) Option<u8>);
impl U2 {
    #[inline]
    pub fn is_ok(&self, xs: usize, dashes: usize) -> bool {
        match self.0 {
            None => true,
            Some(x) => dashes as u8 <= x && xs as u8 <= 4u8 - x
        }
    }
}

impl From<char> for U2 {
    fn from(value: char) -> Self {
        match value {
            '0' => U2(Some(0)),
            '1' => U2(Some(1)),
            '2' => U2(Some(2)),
            '3' => U2(Some(3)),
            '4' => U2(Some(4)),
            ' ' | '_' | '-' => U2(None),
            _ => unreachable!("U2 can't be guessed from {value}"),
        }
    }
}

impl From<U2> for char {
    fn from(value: U2) -> char {
        match value {
            U2(Some(0)) => '0',
            U2(Some(1)) => '1',
            U2(Some(2)) => '2',
            U2(Some(3)) => '3',
            U2(Some(4)) => '4',
            U2(None) => ' ',
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
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

impl std::ops::DerefMut for Fence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Fence {
    type Target = Option<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
