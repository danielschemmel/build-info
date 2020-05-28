use proc_macro_hack::proc_macro_hack;

pub use build_info_common::*;

#[cfg(feature = "runtime")]
pub use lazy_static::lazy_static; // used by the proc macro
#[cfg(feature = "runtime")]
pub use build_info_proc::build_info;

#[cfg_attr(not(feature = "nested"), proc_macro_hack)]
#[cfg_attr(feature = "nested", proc_macro_hack(support_nested))]
pub use build_info_proc::format;
