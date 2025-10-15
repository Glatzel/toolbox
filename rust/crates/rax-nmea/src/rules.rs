mod nmea_coord;
mod nmea_date;
mod nmea_degree;
mod nmea_time;
mod nmea_validate;
use rax::str_parser::filters::CharSetFilter;
use rax::str_parser::rules::{UntilChar, UntilMode, UntilOneInCharSet};

pub use crate::rules::nmea_coord::NmeaCoord;
pub use crate::rules::nmea_date::NmeaDate;
pub use crate::rules::nmea_degree::NmeaDegree;
pub use crate::rules::nmea_time::NmeaTime;
pub use crate::rules::nmea_validate::NmeaValidate;

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
