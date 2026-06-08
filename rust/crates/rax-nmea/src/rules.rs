mod coord;
mod date;
mod degree;
mod gsv_line_count;
mod identifier;
mod talker;
mod time;
mod txt_line_count;
mod validate;

use rax::string::filters::CharSetFilter;
use rax::string::rules::{UntilChar, UntilMode, UntilOneInCharSet};

pub use crate::rules::coord::NmeaCoord;
pub use crate::rules::date::NmeaDate;
pub use crate::rules::degree::NmeaDegree;
pub use crate::rules::gsv_line_count::NmeaGsvLineCount;
pub use crate::rules::identifier::NmeaIdentifier;
pub use crate::rules::talker::NmeaTalker;
pub use crate::rules::time::NmeaTime;
pub use crate::rules::txt_line_count::NmeaTxtLineCount;
pub use crate::rules::validate::NmeaValidate;

pub const UNTIL_COMMA_DISCARD: UntilChar<','> = UntilChar {
    mode: UntilMode::Discard,
};
pub const UNTIL_STAR_DISCARD: UntilChar<'*'> = UntilChar {
    mode: UntilMode::Discard,
};
pub const UNTIL_NEW_LINE_DISCARD: UntilChar<'\n'> = UntilChar {
    mode: UntilMode::Discard,
};

pub const UNTIL_COMMA_OR_STAR_DISCARD: UntilOneInCharSet<2> = UntilOneInCharSet {
    filter: &CharSetFilter::new([',', '*']),
    mode: UntilMode::Discard,
};
