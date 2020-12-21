//! # Advent Of Code 2020
//!
//! Solutions in Rust.
//!
//! This module contains general utilities that are not associated
//! to the challenge of a particular day, such as reading input data
//! from a file.

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_2;
pub mod day_20;
pub mod day_21;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

/// Read the data from the input file.
///
/// The path of the file is given by `path`.
///
/// The type into which each line shall be parsed
/// is given by `T`.
pub fn read_data<T, P: AsRef<Path>>(path: P) -> io::Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path)?;
    let vec = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().trim().parse::<T>().unwrap())
        .collect();

    Ok(vec)
}
