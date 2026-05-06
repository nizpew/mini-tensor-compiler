pub mod pass;
pub mod fusion;
pub mod constant_folding;
pub mod dead_code;

pub use pass::*;
pub use fusion::*;
pub use constant_folding::*;
pub use dead_code::*;
