/// Type of component values in a pixel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// Unsigned 8-bit integer (0-255)
    U8,

    /// Unsigned 16-bit integer (0-65535)
    U16,

    /// 16-bit half-precision floating point
    F16,

    /// 32-bit floating point (0.0-1.0)
    F32,
}

impl ComponentType {
    /// Size in bytes of this component type
    pub fn size_bytes(&self) -> usize {
        match self {
            ComponentType::U8 => 1,
            ComponentType::U16 => 2,
            ComponentType::F16 => 2,
            ComponentType::F32 => 4,
        }
    }
}
