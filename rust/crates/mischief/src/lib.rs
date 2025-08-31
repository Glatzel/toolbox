#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::{Debug, Display, Write};
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
                write!(output, "{} ", "x".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("x ");
            } else if i == msgs_len - 1 {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "╰─▶".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("╰─▶ ");
            } else {
                #[cfg(feature = "fancy")]
                write!(output, "{} ", "├─▶".red())?;
                #[cfg(not(feature = "fancy"))]
                output.push_str("├─▶ ");
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
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static;
    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D;
}

impl<T> WrapErr<T> for Result<T, Report> {
    /// Wraps the error with a custom message and returns a `Result`.
    fn wrap_err<D>(self, msg: D) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
    {
        match self {
            Err(mut e) => {
                let mut final_msg = String::new();
                // Add custom message + formatted error
                write!(final_msg, "{}", msg).ok();
                e.append_error(final_msg); // Append the new message to the Report
                Err(e)
            }
            ok => ok,
        }
    }

    fn wrap_err_with<D, F>(self, msg: F) -> Result<T, Report>
    where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
    {
        match self {
            Err(mut e) => {
                let mut final_msg = String::new();
                // Add custom message + formatted error
                write!(final_msg, "{}", msg()).ok();
                e.append_error(final_msg); // Append the new message to the Report
                Err(e)
            }
            ok => ok,
        }
    }
}
#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    #[test]
    fn report_new_and_append_error() {
        let mut report = Report::new("Initial error".to_string());
        assert_eq!(report.msgs.len(), 1);
        report.append_error("Second error".to_string());
        assert_eq!(report.msgs.len(), 2);

        let msgs: Vec<_> = report.msgs.iter().cloned().collect();
        assert_eq!(msgs[0], "Second error");
        assert_eq!(msgs[1], "Initial error");
    }

    #[test]
    fn report_from_str() {
        let report: Report = Report::from("Error from str");
        assert_eq!(report.msgs.len(), 1);
        assert_eq!(report.msgs.front().unwrap(), "Error from str");
    }

    #[test]
    fn debug_format_basic() {
        let mut report = Report::new("First".to_string());
        report.append_error("Second".to_string());
        let debug_str = format!("{:?}", report);
        assert!(debug_str.contains("First"));
        assert!(debug_str.contains("Second"));
    }

    #[test]
    fn into_mischief_ok_and_err() {
        let ok: core::result::Result<u32, &str> = Ok(42);
        let err: core::result::Result<u32, &str> = Err("fail");
        assert_eq!(ok.into_mischief().unwrap(), 42);

        let report = err.into_mischief().unwrap_err();
        assert_eq!(report.msgs.front().unwrap(), "\"fail\"");
    }

    #[test]
    fn wrap_err_adds_message() {
        let err: Result<u32> = Err(Report::from("original"));
        let wrapped = err.wrap_err("context");
        let report = wrapped.unwrap_err();
        let msgs: Vec<_> = report.msgs.iter().cloned().collect();
        assert!(msgs.contains(&"context".to_string()));
        assert!(msgs.contains(&"original".to_string()));
    }

    #[test]
    fn wrap_err_with_adds_message() {
        let err: Result<u32> = Err(Report::from("original"));
        let wrapped = err.wrap_err_with(|| "context_with");
        let report = wrapped.unwrap_err();
        let msgs: Vec<_> = report.msgs.iter().cloned().collect();
        assert!(msgs.contains(&"context_with".to_string()));
        assert!(msgs.contains(&"original".to_string()));
    }
}
