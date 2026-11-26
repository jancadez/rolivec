macro_rules! impl_vec {
    ($t:ident, $s:ty, { $($f:ident),+ }) => {

        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Default, Debug)]
        pub struct $t {
            $(pub $f: $s),+
        }

        impl $t {
            pub const fn new($($f: $s),+) -> Self { Self { $($f),+ } }
            pub const fn splat(n: $s) -> Self { Self { $($f: n),+ } }

            pub const fn zero() -> Self { Self { $($f: 0 as $s),+ } }
            pub const fn one() -> Self { Self { $($f: 1 as $s),+ } }

            $(
            pub const fn $f() -> Self {
                let mut v = $t::zero();
                v.$f = 1 as $s;
                v
            }
            )+

            pub fn min(self, rhs: Self) -> Self { Self { $($f: self.$f.min(rhs.$f)),+ } }
            pub fn max(self, rhs: Self) -> Self { Self { $($f: self.$f.max(rhs.$f)),+ } }
            pub fn clamp(self, min: Self, max: Self) -> Self { self.min(max).max(min) }
            pub fn dot(self, rhs: Self) -> $s {(0 as $s) $(+ self.$f * rhs.$f)+}
        }

        impl std::ops::Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self { Self { $($f: self.$f + rhs.$f),+ } }
        }
        impl std::ops::AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) { *self = Self { $($f: self.$f + rhs.$f),+ } }
        }

        impl std::ops::Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self { Self { $($f: self.$f - rhs.$f),+ } }
        }
        impl std::ops::SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) { *self = Self { $($f: self.$f - rhs.$f),+ } }
        }

        impl std::ops::Mul<$s> for $t {
            type Output = Self;
            fn mul(self, rhs: $s) -> Self { Self { $($f: self.$f * rhs),+ } }
        }
        impl std::ops::MulAssign<$s> for $t {
            fn mul_assign(&mut self, rhs: $s) { *self = Self { $($f: self.$f * rhs),+ } }
        }

        impl std::ops::Div<$s> for $t {
            type Output = Self;
            fn div(self, rhs: $s) -> Self { Self { $($f: self.$f / rhs),+ } }
        }
        impl std::ops::DivAssign<$s> for $t {
            fn div_assign(&mut self, rhs: $s) { *self = Self { $($f: self.$f / rhs),+ } }
        }

        impl std::ops::Mul<$t> for $s {
            type Output = $t;
            fn mul(self, rhs: $t) -> $t { rhs * self }
        }

        impl std::ops::Neg for $t {
            type Output = Self;
            fn neg(self) -> Self { Self { $($f: -self.$f),+ } }
        }
    };
}

macro_rules! impl_mat {
    ($m:ident, $c:ident, { $($f:ident),+ }) => {
        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub struct $m { $(pub $f: $c),+ }

        impl $m {
            pub const fn new($($f: $c),+) -> Self { Self { $($f),+ } }
            pub const fn identity() -> Self { Self { $($f: $c::$f()),+ } }
            pub const fn zero() -> Self { Self { $($f: $c::zero()),+ } }
        }

        impl std::ops::Mul<$c> for $m {
            type Output = $c;
            fn mul(self, v: $c) -> $c { $c::zero() $(+ self.$f * v.$f)+ }
        }

        impl std::ops::Mul for $m {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self { Self { $($f: self * rhs.$f),+ } }
        }

        impl std::default::Default for $m {
            fn default() -> Self {
                Self::identity()
            }
        }
    };
}

pub(crate) use impl_vec;
pub(crate) use impl_mat;
