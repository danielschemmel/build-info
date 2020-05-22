use proc_macro_hack::proc_macro_hack;

pub use versionator_common::*;

pub use lazy_static::lazy_static; // used by the proc macro
pub use versionator_proc::versionator;

#[proc_macro_hack]
pub use versionator_proc::version;

#[proc_macro_hack]
pub use versionator_proc::format;
