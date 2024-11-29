#![deny(clippy::all)]
#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::implicit_hasher,
    clippy::match_on_vec_items,
    clippy::maybe_infinite_iter,
    clippy::if_not_else,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::bool_to_int_with_if,
    clippy::missing_errors_doc,
    clippy::struct_field_names
)]

#[cfg(test)]
mod template;

mod utils;

pub mod y2019;
pub mod y2020;
pub mod y2021;
pub mod y2022;
pub mod y2023;

#[allow(dead_code)] // False positive detection :(
pub(crate) trait IntoResult<T> {
    fn into_result(self) -> anyhow::Result<T>;
}

impl<T> IntoResult<T> for anyhow::Result<T> {
    #[inline]
    fn into_result(self) -> anyhow::Result<T> {
        self
    }
}

#[macro_export]
macro_rules! into_result_impl {
    ($type:path $(,)?) => {
        impl IntoResult<$type> for $type {
            #[inline]
            fn into_result(self) -> anyhow::Result<$type> { Ok(self) }
        }
    };
    ($type:path $(, $types:path)+ $(,)?) => {
        into_result_impl!($type);
        into_result_impl!($($types, )+);
    };
}
into_result_impl!(u32, u64, usize, i32, i64, isize, String);

#[cfg(not(feature = "ignore-sanity"))]
#[macro_export]
macro_rules! aoc_test_suite {
    ($func:path, ($name:ident, $expected:expr $(, $input:expr)+ $(,)?) $(,)?) => {
        #[test]
        fn $name() -> anyhow::Result<()>{
            let expected = $expected;
            let start = std::time::Instant::now();
            let actual = $func($($input, )+);
            let elapsed = start.elapsed();
            eprintln!("Test {}::{} ran in {:#?}", module_path!(), stringify!($name), elapsed);
            let actual = $crate::IntoResult::into_result(actual)?;
            assert_eq!(expected, actual);
            Ok(())
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
