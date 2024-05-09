use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Decimal};

#[derive(Deserialize,Debug,Clone,FromRow,Serialize)]
pub struct City {
    pub id : i32,
    pub name : String,
    pub state_id : i32,
    pub state_code : String,
    pub country_id : i32,
    pub country_code : String,
    pub latitude :Decimal,
    pub longitude : Decimal,
    pub created_at : Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at : Option<chrono::DateTime<chrono::Utc>>,
    pub flag : bool,
    pub wikidataid : Option<String>
}
