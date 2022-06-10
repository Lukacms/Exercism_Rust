//
// EPITECH PROJECT, 2022
// exercices_rust
// File description:
// lib
//

#![allow(unused)]
#![allow(dead_code)]

use std::iter::Iterator;

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quot = dividend / divisor;
    let rem = dividend % divisor;

    return (quot, rem);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let result: dyn Iterator<Item = T> = iter.filter(|x| *x % 2 != 0);

    return result;
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        unimplemented!("implement `fn manhattan`")
    }
}
