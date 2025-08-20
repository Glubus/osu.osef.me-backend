use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;

pub fn from_f32(value: f32) -> BigDecimal {
    BigDecimal::from_f32(value).unwrap()
}

pub fn from_f64(value: f64) -> BigDecimal {
    BigDecimal::from_f64(value).unwrap()
}