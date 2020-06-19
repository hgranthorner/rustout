use std::convert::{TryFrom};

pub trait SafeSubtract {
    fn try_subtract<T>(&self, other: T) -> Result<Self, &str> where Self: TryFrom<T>;
}

impl SafeSubtract for u8 {
    fn try_subtract<T>(&self, other: T) -> Result<Self, &str> where Self: TryFrom<T>{
        match u8::try_from(other) {
            Ok(x) => {
                if x > *self {
                    Err("Overflow during safe_subtract")
                } else {
                    Ok(*self - x)
                }
            }
            Err(_) => Err("Overflow during safe_subtract")
        }
    }
}
