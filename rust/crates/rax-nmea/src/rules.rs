mod coord;
mod date;
mod degree;
mod gsv_line_count;
mod identifier;
mod talker;
mod time;
mod txt_line_count;
mod validate;

use rax::text::filters::AsciiCharSetFilter;
use rax::text::rules::{UntilChar, UntilMode, UntilOneInCharSet};

pub use crate::rules::coord::NmeaCoord;
pub use crate::rules::date::NmeaDate;
pub use crate::rules::degree::NmeaDegree;
pub use crate::rules::gsv_line_count::NmeaGsvLineCount;
pub use crate::rules::identifier::NmeaIdentifier;
pub use crate::rules::talker::NmeaTalker;
pub use crate::rules::time::NmeaTime;
pub use crate::rules::txt_line_count::NmeaTxtLineCount;
pub use crate::rules::validate::{NmeaValidate, NmeaValidateMultiLine};

/// Consumes input up to and including the next comma, discarding the
/// delimiter.
pub const UNTIL_COMMA_DISCARD: UntilChar<',', true> = UntilChar {
    mode: UntilMode::Discard,
};
/// Consumes input up to the next comma, keeping the delimiter in the
/// remaining input.
pub const UNTIL_COMMA_KEEP_RIGHT: UntilChar<',', true> = UntilChar {
    mode: UntilMode::KeepInRest,
};
/// Consumes input up to and including the next `M` (meters) unit marker,
/// discarding the delimiter.
pub const UNTIL_M_DISCARD: UntilChar<'M', true> = UntilChar {
    mode: UntilMode::Discard,
};
/// Consumes input up to and including the `*` checksum delimiter, discarding
/// the delimiter.
pub const UNTIL_STAR_DISCARD: UntilChar<'*', true> = UntilChar {
    mode: UntilMode::Discard,
};
/// Consumes input up to and including the next newline, discarding the
/// delimiter.
pub const UNTIL_NEW_LINE_DISCARD: UntilChar<'\n', true> = UntilChar {
    mode: UntilMode::Discard,
};

/// Consumes input up to and including the next comma or `*` checksum
/// delimiter, discarding the delimiter.
pub const UNTIL_COMMA_OR_STAR_DISCARD: UntilOneInCharSet<'_, true, 2, AsciiCharSetFilter<2>> =
    UntilOneInCharSet {
        filter: &AsciiCharSetFilter::new([',', '*']),
        mode: UntilMode::Discard,
    };
/// Consumes input up to the next comma or `*` checksum delimiter, keeping the
/// delimiter in the remaining input.
pub const UNTIL_COMMA_OR_STAR_KEEP_RIGHT: UntilOneInCharSet<'_, true, 2, AsciiCharSetFilter<2>> =
    UntilOneInCharSet {
        filter: &AsciiCharSetFilter::new([',', '*']),
        mode: UntilMode::KeepInRest,
    };
