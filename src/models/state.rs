use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Decimal};

#[derive(Deserialize,Debug,Clone,FromRow,Serialize)]
pub struct State {
    pub id : i32,
    pub name : String,
    pub country_id : i32,
    pub country_code : String,
    pub fips_code : Option<String>,
    pub iso2 : Option<String>,
    pub r#type : Option<String>,
    pub latitude : Option<Decimal>,
    pub longitude : Option<Decimal>,
    pub created_at : chrono::DateTime<chrono::Utc>,
    pub updated_at : chrono::DateTime<chrono::Utc>,
    pub flag : bool,
    pub wikidataid : Option<String>,
}
