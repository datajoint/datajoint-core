use crate::types::DecodeResult;

pub enum PhArg {
    String(String),
    Int(i32)
}


pub struct PlaceHolderArgument {

    arg : DecodeResult

}


impl PlaceHolderArgument {

    pub fn new(arg : DecodeResult) -> Self {
        PlaceHolderArgument {
            arg
        }
    }

    pub fn data(self) -> DecodeResult {
        self.arg
    }

}

