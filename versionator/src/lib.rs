use proc_macro_hack::proc_macro_hack;

pub use versionator_common::*;

#[cfg(feature = "runtime")]
pub use lazy_static::lazy_static; // used by the proc macro
#[cfg(feature = "runtime")]
pub use versionator_proc::versionator;

#[cfg_attr(not(feature = "nested"), proc_macro_hack)]
#[cfg_attr(feature = "nested", proc_macro_hack(support_nested))]
pub use versionator_proc::format;
