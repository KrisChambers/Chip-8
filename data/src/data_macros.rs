#[macro_export]
macro_rules! format_as_hex {
    ($type_name: ty) => {
        use std::fmt;
        use std::result::Result;

        impl fmt::Display for $type_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                f.write_fmt(format_args!("{:#X}", self.0))
            }
        }

        impl fmt::Debug for $type_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                fmt::Display::fmt(&self, f)
            }
        }
    };
}

/// Creates a simple tuple type wrapping a single numeric type.
///
/// Debug and Display are formatted to display as hexidecimal
///
/// # Arguments
///
/// * **name** - The name of the type.
/// * **wrapped_type** - The type being wrapped.
/// * **filter** -  A simple function for formatting the value.
#[macro_export]
macro_rules! numeric_wrapper {
    ($name: ident, $wrapped_type: ty, $filter: expr) => {
        #[derive(PartialEq, PartialOrd, Copy, Clone, Default)]
        pub struct $name($wrapped_type);

        impl $name {
            pub fn new(raw: $wrapped_type) -> Self {
                $name($filter(raw))
            }

            pub fn get_raw(&self) -> $wrapped_type {
                self.0
            }
        }

        impl From<$wrapped_type> for $name {
            fn from(value: $wrapped_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $wrapped_type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        use std::ops;

        impl ops::BitOr for $name {
            type Output = Self;
            
            fn bitor(self, rhs: Self) -> Self {
                (self.0 | rhs.0).into()
            }   
        }

        impl ops::BitAnd for $name {
            type Output = Self;
        
            fn bitand(self, rhs: Self) -> Self {
                (self.0 & rhs.0).into()
            }
        }

        impl ops::BitXor for $name {
            type Output = Self;
        
            fn bitxor(self, rhs: Self) -> Self {
                (self.0 ^ rhs.0).into()
            }
        }

        impl<T: Into<$wrapped_type>> ops::Add<T> for $name {
            type Output = Self;
            
            fn add(self, rhs: T) -> Self {
                Self(self.0 + rhs.into())
            }
        }

        impl ops::Sub for $name {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self {
                (self.0 - rhs.0).into()
            }
        }

        impl ops::Shr<usize> for $name {
            type Output = Self;
            
            fn shr(self, rhs: usize) -> Self {
                (self.0 >> rhs).into()
            }
        }

        impl ops::Shl<usize> for $name {
            type Output = Self;
        
            fn shl(self, rhs: usize) -> Self {
                (self.0 << rhs).into()
            }
        }

        format_as_hex!($name);
    };
}
