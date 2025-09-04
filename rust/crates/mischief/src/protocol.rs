extern crate alloc;

use alloc::boxed::Box;
use core::fmt::{Debug, Display};

pub trait IDiagnostic: Debug {
    fn description<'a>(&'a self) -> Option<Box<dyn Display + 'a>>;
    fn source(&self) -> Option<&dyn IDiagnostic>;
}
