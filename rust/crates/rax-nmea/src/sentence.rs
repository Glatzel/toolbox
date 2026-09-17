mod dhv;
mod dtm;
mod gbq;
mod gbs;
mod gga;
mod gll;
mod glq;
mod gnq;
mod gns;
mod gpq;
mod grs;
mod gsa;
mod gst;
mod gsv;
mod rmc;
mod ths;
mod txt;
mod vlw;
mod vtg;
mod zda;

pub use dhv::*;
pub use dtm::*;
pub use gbq::*;
pub use gbs::*;
pub use gga::*;
pub use gll::*;
pub use glq::*;
pub use gnq::*;
pub use gns::*;
pub use gpq::*;
pub use grs::*;
pub use gsa::*;
pub use gst::*;
pub use gsv::*;
pub use rmc::*;
pub use ths::*;
pub use txt::*;
pub use vlw::*;
pub use vtg::*;
pub use zda::*;

#[cfg(test)]
#[cfg_attr(test, macro_export)]
macro_rules! test_sentence {
    ($name:ident,$index:literal,$identifier:tt,$input:literal ) => {
        #[test]
        fn $name() -> mischief::Result<()> {
            extern crate std;
            use std::println;

            clerk::init_log_with_level(clerk::LevelFilter::TRACE);

            let dhv = $identifier::parse_str($input)?;
            println!("{dhv:?}");
            insta::assert_json_snapshot!(stringify!($index), dhv);
            Ok(())
        }
    };
}
