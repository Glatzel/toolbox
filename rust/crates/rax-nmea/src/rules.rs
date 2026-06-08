mod coord;
mod date;
mod degree;
mod identifier;
mod talker;
mod time;
mod validate;
mod txt_count;
mod gsv_count;

use rax::string::filters::CharSetFilter;
use rax::string::rules::{UntilChar, UntilMode, UntilOneInCharSet};

pub use crate::rules::coord::NmeaCoord;
pub use crate::rules::date::NmeaDate;
pub use crate::rules::degree::NmeaDegree;
pub use crate::rules::gsv_count::NmeaGsvCount;
pub use crate::rules::identifier::NmeaIdentifier;
pub use crate::rules::talker::NmeaTalker;
pub use crate::rules::time::NmeaTime;
pub use crate::rules::txt_count::NmeaTxtCount;
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
