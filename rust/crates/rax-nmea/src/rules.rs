mod nmea_coord;
mod nmea_date;
mod nmea_degree;
mod nmea_time;
mod nmea_validate;
use rax::str_parser::filters::CharSetFilter;
use rax::str_parser::rules::{UntilChar, UntilMode, UntilOneInCharSet};

use crate::rules::nmea_coord::NmeaCoord;
use crate::rules::nmea_date::NmeaDate;
use crate::rules::nmea_degree::NmeaDegree;
use crate::rules::nmea_time::NmeaTime;
use crate::rules::nmea_validate::NmeaValidate;

pub const UNTIL_COMMA_DISCARD: UntilChar<','> = UntilChar {
    mode: UntilMode::Discard,
};
pub const UNTIL_STAR_DISCARD: UntilChar<'*'> = UntilChar {
    mode: UntilMode::Discard,
};
pub const UNTIL_NEW_LINE_DISCARD: UntilChar<'\n'> = UntilChar {
    mode: UntilMode::Discard,
};

pub const NMEA_COORD: NmeaCoord = NmeaCoord();
pub const NMEA_DATE: NmeaDate = NmeaDate();
pub const NMEA_TIME: NmeaTime = NmeaTime();
pub const NMEA_VALIDATE: NmeaValidate = NmeaValidate();
pub const NMEA_DEGREE: NmeaDegree = NmeaDegree();

pub const UNTIL_COMMA_OR_STAR_DISCARD: UntilOneInCharSet<2> = UntilOneInCharSet {
    filter: &CharSetFilter::new([',', '*']),
    mode: UntilMode::Discard,
};
