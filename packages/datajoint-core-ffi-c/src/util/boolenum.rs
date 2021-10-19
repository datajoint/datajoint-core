/// Generalized types supported by DataJoint.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalBool {
    True,
    False,
    None,
}