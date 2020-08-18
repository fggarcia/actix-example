use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DomainQuery {
    pub num: i8,
}