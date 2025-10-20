#[cfg(test)]
mod tests;

// FIXME: It would be better to generate these from build.rs rather than checking in a huge Rust source file
// FIXME: there is some kind of syntax error in the generated doc comments which prevents the generated code from compiling
// with rustc.  Maybe a bindgen bug to report
// bindgen --merge-extern-blocks plasma-sys/all-the-headers.h -o plasma-sys/src/generated.rs --no-doc-comments -- -I plasma-sys/
#[allow(warnings)]
mod generated;

pub use generated::*;

#[allow(non_camel_case_types)]
pub type slaw_guts = _slaw;
