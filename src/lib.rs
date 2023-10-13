#![cfg_attr(not(feature = "std"), no_std)]

use compile_warning::compile_warning;

#[cfg(feature = "std")]
pub use std;

#[cfg(not(feature = "std"))]
pub use core as std;

compile_warning!(This library is not suitable for production usage);