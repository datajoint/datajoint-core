use datajoint_core::util::IntegerEnum;
use num_traits::FromPrimitive;

/// Three-state boolean for representing `Option<bool>` in an FFI-compatible manner.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive)]
pub enum OptionalBool {
    None = -1,
    False = 0,
    True = 1,
}

impl OptionalBool {
    /// Converts the enum into its Rust type.
    pub fn into(self) -> Option<bool> {
        match self {
            OptionalBool::None => None,
            OptionalBool::False => Some(false),
            OptionalBool::True => Some(true),
        }
    }

    /// Converts an optional boolean value into the enum type.
    pub fn from_option(from: Option<bool>) -> OptionalBool {
        match from {
            None => OptionalBool::None,
            Some(false) => OptionalBool::False,
            Some(true) => OptionalBool::True,
        }
    }
}

impl IntegerEnum<i32> for OptionalBool {
    fn from_int(val: i32) -> Option<Self> {
        FromPrimitive::from_i32(val)
    }
}
