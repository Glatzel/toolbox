#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Write;
extern crate alloc;
use alloc::collections::LinkedList;
use alloc::string::String;

#[derive(Debug)]
pub struct Report {
    msgs: LinkedList<String>,
}

impl Report {
    // Helper method to create a new Report from a single message
    pub fn new(msg: String) -> Self {
        let mut msgs = LinkedList::new();
        msgs.push_front(msg); // Add the initial error message
        Report { msgs }
    }

    // Helper method to append a new error message to the Report
    pub fn append_error(&mut self, msg: String) {
        self.msgs.push_front(msg); // Add the new error message to the front
    }
}

impl From<&str> for Report {
    fn from(msg: &str) -> Self { Report::new(String::from(msg)) }
}

pub trait IntoMischief<T> {
    fn into_mischief(self) -> Result<T>;
}

pub type Result<T, E = Report> = core::result::Result<T, E>;

impl<T, E: core::fmt::Debug> IntoMischief<T> for core::result::Result<T, E> {
    fn into_mischief(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut msg: String = String::new();
                write!(msg, "{:?}", e).ok();
                Err(Report::from(msg.as_str()))
            }
        }
    }
}

pub trait WrapErr<T> {
    fn wrap_err(self, msg: &'static str) -> Result<T, Report>;
}

impl<T> WrapErr<T> for Result<T, Report> {
    /// Wraps the error with a custom message and returns a `Result`.
    fn wrap_err(self, msg: &'static str) -> Result<T, Report> {
        match self {
            Ok(v) => Ok(v),
            Err(mut e) => {
                let mut final_msg = String::new();
                // Add custom message + formatted error
                write!(final_msg, "{}", msg).ok();
                e.append_error(final_msg); // Append the new message to the Report
                Err(e)
            }
        }
    }
}
#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_err_creates_error_chain() {
        let initial_err: Result<i32> = Err(Report::from("Initial error"));

        // Wrap the error with a custom message
        let result = initial_err.wrap_err("First wrap").wrap_err("second wrap");

        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(report) => {
                // Test that the error chain contains the expected messages
                println!("{report:?}")
            }
        }
    }
}
