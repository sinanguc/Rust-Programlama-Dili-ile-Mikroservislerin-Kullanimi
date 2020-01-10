extern crate serde;

use super::schema::kullanici;

use diesel::deserialize::Queryable;


use models::serde::ser::{Serialize, Serializer, SerializeStruct};

use DB;

#[derive(Identifiable)]
#[derive(AsChangeset)]
#[table_name="kullanici"]
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Kullanici {
    pub id: i32,
    pub adi: String,
    pub soyadi: String,
    pub kullanici_adi: String,
    pub sifre: String,
    pub eposta: String,
}

impl Queryable<kullanici::SqlType, DB> for Kullanici {
    type Row = (i32, String, String, String, String, String);

    fn build(row: Self::Row) -> Self {
        Kullanici {
            id: row.0,
            adi  : row.1,
            soyadi : row.2,
            kullanici_adi   : row.3,
            sifre   : row.4,
            eposta      : row.5,
        }
    }
}

impl Serialize for Kullanici {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 6 sayısı Kullanici struct'ta oluşturduğumuz alan sayısıdır.
        let mut state = serializer.serialize_struct("Kullanici", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("adi", &self.adi)?;
        state.serialize_field("soyadi", &self.soyadi)?;
        state.serialize_field("kullanici_adi", &self.kullanici_adi)?;
        state.serialize_field("sifre", &self.sifre)?;
        state.serialize_field("eposta", &self.eposta)?;
        state.end()
    }
}