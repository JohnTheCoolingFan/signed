use std::ops::{AddAssign, SubAssign};

/// Wrapper type around signed integers to do generic operations no matter what size they are
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

/// A pointer to absolute value of a number
///
/// ```
/// use signed::Signed;
///
/// let mut n: i32 = 42; // A number to test operations with it
///
/// let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
///
/// abs_n -= &2;
/// drop(abs_n);
///
/// assert_eq!(n, 40);
///
/// let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
///
/// abs_n += &6;
/// drop(abs_n);
///
/// assert_eq!(n, 46);
///
/// n = -42; // Let's try with negative number
///
/// // Unfortunately, due to assertions, abs_n has to be dropped before immutable use, but when n
/// // is changed, abs_n is changed accordingly
///
/// let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
///
/// abs_n -= &4;
/// drop(abs_n);
///
/// assert_eq!(n, -38); // Now that's different, we subtracted, but got a number *bigger* than the initial one.
///
/// let mut abs_n = n.get_absolute(); // Gets a "pointer" to an absolute value of a number
///
/// abs_n += &6;
/// drop(abs_n);
///
/// assert_eq!(n, -44); // And by adding to the absolute value, we get a number *smaller*.
///
/// // All of this works for i8, i16, i32, i64 and i128!
/// ```
#[derive(Debug)]
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
    #[inline]
    pub fn get_value(&self) -> I::Unsigned {
        self.num.abs_diff(I::ZERO)
    }
}

impl<'a, I: Signed> AddAssign<&I> for Absolute<'a, I> {
    #[inline]
    fn add_assign(&mut self, rhs: &I) {
        if self.num.is_negative() {
            self.num.sub_assign(*rhs)
        } else {
            self.num.add_assign(*rhs)
        };
    }
}

impl<'a, I: Signed> SubAssign<&I> for Absolute<'a, I> {
    #[inline]
    fn sub_assign(&mut self, rhs: &I) {
        if self.num.is_negative() {
            self.num.add_assign(*rhs)
        } else {
            self.num.sub_assign(*rhs)
        }
    }
}
