mod json;
mod json_helpers;
mod status;
mod table;
mod violations;
pub use json::print_json;
use linecheck::{
    checker::{CheckOptions, check_file},
    config::ConfigResolver,
    result::FileResult,
};
pub use status::print_status;
use std::path::PathBuf;
pub use violations::print_violations;

#[cfg(test)]
mod digits_tests;
#[cfg(test)]
mod json_tests_1a;
#[cfg(test)]
mod json_tests_1b;
#[cfg(test)]
mod json_tests_2;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod violations_message_test;
#[cfg(test)]
mod violations_tests_a;
#[cfg(test)]
mod violations_tests_b;

pub(crate) fn run<F: FnMut(&PathBuf, FileResult)>(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    mut each: F,
) {
    for f in files {
        match check_file(f, resolver.resolve(f).as_ref(), opts) {
            Ok(r) => each(f, r),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
