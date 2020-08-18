use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub vec: Vec<usize>,
}