use core::str::FromStr;

use hashbrown::HashMap;
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::data::{Identifier, Talker};

/// Dispatcher reads and groups sentences, handling both single and multi-line
/// messages.
pub struct Dispatcher {
    buffer: HashMap<(Talker, Identifier), String>, // (total count, accumulated sentence)
}

impl Default for Dispatcher {
    fn default() -> Self { Self::new() }
}

impl Dispatcher {
    /// Create a new dispatcher with the given reader.
    pub fn new() -> Self {
        Self {
            buffer: HashMap::new(),
        }
    }

    /// Read and parse a line, returning its talker, identifier, and the
    /// sentence.
    fn preprocess(&mut self, sentence: String) -> Option<(Talker, Identifier, String)> {
        let talker = match Talker::from_str(&sentence) {
            Ok(t) => t,
            Err(_e) => {
                clerk::warn!("{:?}", _e);
                return None;
            }
        };
        let identifier = match Identifier::from_str(&sentence) {
            Ok(i) => i,
            Err(_e) => {
                clerk::warn!("{:?}", _e);
                return None;
            }
        };
        Some((talker, identifier, sentence))
    }

    /// Handle multi-line sentences (e.g., GSV, TXT).
    fn process_multilines(
        &mut self,
        talker: Talker,
        identifier: Identifier,
        sentence: String,
    ) -> Option<(Talker, Identifier, String)> {
        let parts: Vec<&str> = sentence.split(',').collect();
        let count: Option<usize> = parts.get(1).and_then(|s| s.parse().ok());
        let idx: Option<usize> = parts.get(2).and_then(|s| s.parse().ok());
        let (count, idx) = match (count, idx) {
            (Some(c), Some(i)) => (c, i),
            _ => {
                clerk::warn!("Malformed sentence: {}", sentence);
                return None;
            }
        };

        match (
            idx == 1,
            count == idx,
            self.buffer.get(&(talker, identifier)),
        ) {
            (true, true, _) => Some((talker, identifier, sentence)),
            // First line of multi-line, buffer it
            (true, false, None) => {
                self.buffer.insert((talker, identifier), sentence);
                None
            }
            // Newer first line arrived, replace old buffer
            (true, false, Some(_old)) => {
                clerk::warn!(
                    "A newer `{}{}` arrived, remove older one: {}",
                    talker,
                    identifier,
                    _old
                );
                self.buffer.insert((talker, identifier), sentence);
                None
            }
            // Last line, combine with buffer and return
            (false, true, Some(v)) => {
                clerk::debug!("`{}{}` is complete.", talker, identifier);
                let combined = format!("{v}{sentence}");
                self.buffer.remove(&(talker, identifier));
                Some((talker, identifier, combined))
            }
            // Out-of-order line, skip
            (false, _, None) => {
                clerk::warn!(
                    "Former `{}{}` doesn't exist, will skip this sentence: {}",
                    talker,
                    identifier,
                    sentence
                );
                None
            }
            // Middle line, append to buffer
            (false, false, Some(_)) => {
                clerk::debug!(
                    "Append new sentence to `{}{}`: {}",
                    talker,
                    identifier,
                    sentence
                );
                if let Some(entry) = self.buffer.get_mut(&(talker, identifier)) {
                    entry.push_str(&sentence);
                }
                None
            }
        }
    }

    /// Dispatches sentences, handling both single and multi-line types.
    pub fn dispatch(&mut self, sentence: String) -> Option<(Talker, Identifier, String)> {
        if let Some((talker, identifier, sentence)) = self.preprocess(sentence) {
            match identifier {
                // Single-line sentences
                Identifier::DHV
                | Identifier::DTM
                | Identifier::GBQ
                | Identifier::GBS
                | Identifier::GGA
                | Identifier::GLL
                | Identifier::GLQ
                | Identifier::GNQ
                | Identifier::GNS
                | Identifier::GPQ
                | Identifier::GRS
                | Identifier::GSA
                | Identifier::GST
                | Identifier::RMC
                | Identifier::THS
                | Identifier::VLW
                | Identifier::VTG
                | Identifier::ZDA => Some((talker, identifier, sentence)),

                // Multi-line sentences
                Identifier::GSV | Identifier::TXT => {
                    self.process_multilines(talker, identifier, sentence)
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::fs::File;
    use std::io;

    use clerk::{LogLevel, init_log_with_level};
    use rax::io::{IRaxReader, RaxReader};

    use crate::Dispatcher;
    #[test]
    fn test_dispatcher() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);

        for f in [
            "data/nmea1.log",
            "data/nmea2.log",
            "data/nmea_with_sat_info.log",
        ] {
            let file = File::open(f)?;
            let mut reader = RaxReader::new(io::BufReader::new(file));
            let mut dispatcher = Dispatcher::new();

            while let Some(line) = reader.read_line()? {
                dispatcher.dispatch(line);
            }
        }

        Ok(())
    }
}
