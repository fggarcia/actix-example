use cdrs::{frame::IntoBytes, types::from_cdrs::FromCDRSByName, types::prelude::*};
use cdrs_helpers_derive::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct StoreModel {
    pub name: String,
    pub reference: String,
    pub num: i32,
    pub num2: i32
}

#[derive(Deserialize, Debug)]
pub struct StoreModelQuery {
    pub name: String,
}