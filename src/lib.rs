#![deny(clippy::all)]
#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
)]

#[cfg(test)]
mod template;

mod utils;

pub mod y2019;
pub mod y2020;
pub mod y2021;

#[cfg(not(feature = "ignore-sanity"))]
#[macro_export]
macro_rules! aoc_test_suite {
    ($func:path, ($name:ident, $expected:expr $(, $input:expr)+ $(,)?) $(,)?) => {
        #[test]
        fn $name() {
            let expected = $expected;
            let start = std::time::Instant::now();
            let actual = $func($($input, )+);
            let elapsed = start.elapsed();
            eprintln!("Test {}::{} ran in {:#?}", module_path!(), stringify!($name), elapsed);
            assert_eq!(expected, actual);
        }
    };
    ($func:path, ($name:ident, $expected:expr $(, $input:expr)+ $(,)?) $(, ($name_tail:ident, $expected_tail:expr $(, $input_tail:expr)+ $(,)?))+ $(,)?) => {
        aoc_test_suite!($func, ($name, $expected $(, $input)+));
        aoc_test_suite!($func $(, ($name_tail, $expected_tail $(, $input_tail)+))+);
    };
}

#[cfg(feature = "ignore-sanity")]
#[macro_export]
macro_rules! aoc_test_suite {
    ($func:path, ($name:ident, $expected:expr $(, $input:expr)+ $(,)?) $(, ($name_tail:ident, $expected_tail:expr $(, $input_tail:expr)+ $(,)?))* $(,)?) => {
        #[test]
        fn $name() {
            let expected = $expected;
            let start = std::time::Instant::now();
            let actual = $func($($input, )+);
            let elapsed = start.elapsed();
            eprintln!(
                "Test {}::{} ran in {:#?}",
                module_path!(),
                stringify!($name),
                elapsed
            );
            assert_eq!(expected, actual);
        }
    };
}
