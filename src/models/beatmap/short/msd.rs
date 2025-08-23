use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MSDShort {
    pub id: Option<i32>,
    pub overall: Option<BigDecimal>,
    pub main_pattern: Option<String>,
}
