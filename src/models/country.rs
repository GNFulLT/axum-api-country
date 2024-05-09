use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Decimal};

#[derive(Deserialize,Debug,Clone,FromRow,Serialize)]
pub struct Country {
    pub id : i32,
    pub name : Option<String>,
    pub iso3 : Option<String>,
    pub numeric_code : Option<String>,
    pub iso2 : Option<String>,
    pub phonecode : Option<String>,
    pub capital: Option<String>,
    pub currency : Option<String>,
    pub currency_name : Option<String>,
    pub currency_symbol : Option<String>,
    pub tld : Option<String>,
    pub native : Option<String>,
    pub region : Option<String>,
    pub region_id : i32,
    pub subregion : Option<String>,
    pub subregion_id : i32,
    pub nationality : Option<String>,
    pub timezones : Option<String>,
    pub translations : Option<String>,
    pub latitude : Option<Decimal>,
    pub longitude : Option<Decimal>,
    pub emoji : Option<String>,
    pub emojiu : Option<String>,
    pub created_at : chrono::DateTime<chrono::Utc>,
    pub updated_at : chrono::DateTime<chrono::Utc>,
    pub flag : bool,
    pub wikidataid : Option<String>,
}
