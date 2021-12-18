use std::borrow::Cow;
use std::ops::{Index, IndexMut};

use crate::result::Result;
use crate::error::Error;

#[derive(Debug)]
pub struct Rom {
    original: Vec<i32>,
    memory: Vec<i32>,
}

impl Rom {
    pub fn new<'a, const N: usize>(memory: impl Into<&'a [i32; N]>) -> Self {
        let original = memory.into().to_vec();
        Rom {
            original: original.clone(),
            memory: original,
        }
    }

    pub fn from_string(tape: impl AsRef<str>) -> Self {
        let original: Vec<i32> = tape.as_ref()
            .split(",")
            .map(str::trim)
            .filter_map(|x| x.parse().ok())
            .collect();
        Self {
            original: original.clone(),
            memory: original,
        }
    }

    pub fn reset(&mut self) {
        self.memory = self.original.clone();
    }
}

impl Index<usize> for Rom {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
       &self.memory[index]
    }
}

impl IndexMut<usize> for Rom {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}
