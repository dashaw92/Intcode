use std::ops::{Index, Deref, RangeInclusive, IndexMut};

use crate::result::Result;
use crate::error::Error;

#[derive(Debug)]
pub struct Rom {
    memory: Vec<i32>,
}

impl Rom {
    pub fn new<'a, const N: usize>(memory: impl Into<&'a [i32; N]>) -> Self {
       Rom {
            memory: memory.into().to_vec(),
       }
    }

    pub fn from_string(tape: impl AsRef<str>) -> Self {
        let memory = tape.as_ref()
            .split(",")
            .filter_map(|x| x.parse().ok())
            .collect();
        Self {
            memory
        }
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
