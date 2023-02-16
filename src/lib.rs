use std::ops::{AddAssign, SubAssign};

pub trait Signed: AddAssign + SubAssign
where
    Self: Sized + Copy,
{
    type Unsigned;
    const ZERO: Self;

    fn get_absolute(&mut self) -> Absolute<'_, Self>;

    fn abs_diff(self, other: Self) -> Self::Unsigned;

    fn abs(self) -> Self;

    fn is_positive(self) -> bool;

    fn is_negative(self) -> bool;

    fn signum(self) -> Self;
}

pub struct Absolute<'a, I: Signed> {
    num: &'a mut I,
}

impl Signed for i8 {
    type Unsigned = u8;
    const ZERO: i8 = 0;

    #[inline]
    fn get_absolute(&mut self) -> Absolute<'_, Self> {
        Absolute { num: self }
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn abs_diff(self, other: Self) -> Self::Unsigned {
        self.abs_diff(other)
    }

    #[inline]
    fn is_positive(self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.is_negative()
    }

    #[inline]
    fn signum(self) -> Self {
        self.signum()
    }
}

impl Signed for i16 {
    type Unsigned = u16;
    const ZERO: i16 = 0;

    #[inline]
    fn get_absolute(&mut self) -> Absolute<'_, Self> {
        Absolute { num: self }
    }

    #[inline]
    fn abs_diff(self, other: Self) -> Self::Unsigned {
        self.abs_diff(other)
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn is_positive(self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.is_negative()
    }

    #[inline]
    fn signum(self) -> Self {
        self.signum()
    }
}

impl Signed for i32 {
    type Unsigned = u32;
    const ZERO: i32 = 0;

    #[inline]
    fn get_absolute(&mut self) -> Absolute<'_, Self> {
        Absolute { num: self }
    }

    #[inline]
    fn abs_diff(self, other: Self) -> Self::Unsigned {
        self.abs_diff(other)
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn is_positive(self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.is_negative()
    }

    #[inline]
    fn signum(self) -> Self {
        self.signum()
    }
}

impl Signed for i64 {
    type Unsigned = u64;
    const ZERO: i64 = 0;

    #[inline]
    fn get_absolute(&mut self) -> Absolute<'_, Self> {
        Absolute { num: self }
    }

    #[inline]
    fn abs_diff(self, other: Self) -> Self::Unsigned {
        self.abs_diff(other)
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn is_positive(self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.is_negative()
    }

    fn signum(self) -> Self {
        self.signum()
    }
}

impl Signed for i128 {
    type Unsigned = u128;
    const ZERO: i128 = 0;

    #[inline]
    fn get_absolute(&mut self) -> Absolute<'_, Self> {
        Absolute { num: self }
    }

    #[inline]
    fn abs_diff(self, other: Self) -> Self::Unsigned {
        self.abs_diff(other)
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn is_positive(self) -> bool {
        self.is_positive()
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.is_negative()
    }

    #[inline]
    fn signum(self) -> Self {
        self.signum()
    }
}

impl<'a, I: Signed> Absolute<'a, I> {
    /// Get the value of an absolute of a number this absolute value has been taken of
    pub fn get_value(&self) -> I::Unsigned {
        self.num.abs_diff(I::ZERO)
    }
}

impl<'a, I: Signed> AddAssign<&I> for Absolute<'a, I> {
    fn add_assign(&mut self, rhs: &I) {
        if self.num.is_negative() {
            self.num.sub_assign(*rhs)
        } else {
            self.num.add_assign(*rhs)
        };
    }
}

impl<'a, I: Signed> SubAssign<&I> for Absolute<'a, I> {
    fn sub_assign(&mut self, rhs: &I) {
        if self.num.is_negative() {
            self.num.add_assign(*rhs)
        } else {
            self.num.sub_assign(*rhs)
        }
    }
}
