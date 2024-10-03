use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl Add<WrappingU32> for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, n: WrappingU32) -> WrappingU32 {
        WrappingU32::new(self.value.wrapping_add(n.value))
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
