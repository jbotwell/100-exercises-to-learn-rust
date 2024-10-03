use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct SaturatingU16 {
    value: u16,
}

impl<T> PartialEq<T> for SaturatingU16
where
    T: Into<SaturatingU16> + Clone,
{
    fn eq(&self, other: &T) -> bool {
        let m = (*other).clone();
        let n: SaturatingU16 = m.into();
        self.value == n.value
    }
}

impl From<&SaturatingU16> for SaturatingU16 {
    fn from(value: &SaturatingU16) -> Self {
        *value
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        let n: u16 = value.into();
        Self { value: n }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        let n: u16 = (*value).into();
        Self { value: n }
    }
}

impl<T> Add<T> for SaturatingU16
where
    T: Into<SaturatingU16>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        let m: SaturatingU16 = rhs.into();
        SaturatingU16 {
            value: self.value.saturating_add(m.value),
        }
    }
}
