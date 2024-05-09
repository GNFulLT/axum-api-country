use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Skip {
    pub skip : u32,
    pub count : u32
}