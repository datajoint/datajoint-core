// Enum for setters and getters in connection/settings.rs. Used instead of a Option<bool>.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalBool {
    True,
    False,
    None,
}
