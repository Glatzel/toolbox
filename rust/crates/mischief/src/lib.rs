#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::{Debug, Write};
extern crate alloc;
use alloc::collections::LinkedList;
use alloc::string::String;

#[cfg(feature = "fancy")]
use owo_colors::OwoColorize;

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

impl Debug for Report {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut output = String::new();
        let msgs_len = self.msgs.len();

        // Iterate over the messages in the LinkedList and apply color
        for (i, msg) in self.msgs.iter().enumerate() {
            output.push('\n'); // Add a newline between messages
            if i == 0 {
                #[cfg(feature = "fancy")]
                output.push_str("x ".red().to_string().as_str());
                #[cfg(not(feature = "fancy"))]
                output.push_str("x ");
            } else if i == msgs_len - 1 {
                #[cfg(feature = "fancy")]
                output.push_str("╰─▶ ".red().to_string().as_str());
                #[cfg(not(feature = "fancy"))]
                output.push_str("|-> ");
            } else {
                #[cfg(feature = "fancy")]
                output.push_str("├─▶ ".red().to_string().as_str());
                #[cfg(not(feature = "fancy"))]
                output.push_str("|-> ");
            }
            output.push_str(msg);
        }

        // Print the final styled output
        write!(f, "{}", output)
    }
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
        let result = initial_err
            .wrap_err("First wrap")
            .wrap_err("second wrap")
            .wrap_err("third wrap");

        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(report) => {
                println!("{report:?}")
            }
        }
    }
}
