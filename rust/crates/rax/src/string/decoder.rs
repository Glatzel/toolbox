extern crate alloc;

use alloc::string::String;

use crate::string::Parser;

pub trait IDecode<E>: Sized {
    fn decode(parser: &mut Parser) -> Result<Self, E>;
}
pub struct Decoder {
    parser: Parser,
}
impl Decoder {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }
    pub fn decode<D, E>(&mut self, data: String) -> Result<D, E>
    where
        D: IDecode<E>,
    {
        self.parser.init(data);
        D::decode(&mut self.parser)
    }
}
