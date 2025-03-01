// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use std::marker::PhantomData;

pub struct Iter<T> {
    data: Vec<T>,
    index: usize,
    _marker: PhantomData<T>,
}

impl<T: Clone> Iter<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            index: 0,
            _marker: PhantomData,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.index < self.data.len() {
            let item = self.data[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<&T> {
        if self.index < self.data.len() {
            Some(&self.data[self.index])
        } else {
            None
        }
    }
}
