use crate::types::DecodeResult;

pub struct PlaceholderArgument {
    arg: DecodeResult,
}

impl PlaceholderArgument {
    pub fn new(arg: DecodeResult) -> Self {
        PlaceholderArgument { arg }
    }

    pub fn data(self) -> DecodeResult {
        self.arg
    }
}
