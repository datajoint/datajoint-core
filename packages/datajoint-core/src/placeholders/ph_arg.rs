use crate::types::NativeType;

pub struct PlaceholderArgument {
    pub arg: NativeType,
}

impl PlaceholderArgument {
    pub fn new(arg: NativeType) -> Self {
        PlaceholderArgument { arg }
    }

    pub fn data(&self) -> &NativeType {
        &self.arg
    }

    pub fn into_data(self) -> NativeType {
        self.arg
    }
}
