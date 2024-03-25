use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct U2(Option<[bool; 2]>);

impl std::ops::Deref for U2 {
    type Target = Option<[bool; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for U2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("U2")
            .field(&char::from(self.clone()))
            .finish()
    }
}

impl From<char> for U2 {
    fn from(value: char) -> Self {
        match value {
            '0' => U2(Some([false; 2])),
            '1' => U2(Some([false, true])),
            '2' => U2(Some([true, false])),
            '3' => U2(Some([true, true])),
            ' ' => U2(None),
            _ => unreachable!("U2 can't be guessed from {value}"),
        }
    }
}

impl From<U2> for char {
    fn from(value: U2) -> char {
        match value {
            U2(Some([false, false])) => '0',
            U2(Some([false, true])) => '1',
            U2(Some([true, false])) => '2',
            U2(Some([true, true])) => '3',
            U2(None) => ' ',
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Fence(Option<bool>);

impl From<char> for Fence {
    fn from(value: char) -> Self {
        match value {
            'y' => Fence(Some(true)),
            'n' => Fence(Some(false)),
            'u' => Fence(None),
            _ => unreachable!(),
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
            Some(true) => 'y',
            Some(false) => 'n',
            None => 'u',
        }
    }
}
