extern crate diesel;
extern crate r2d2;
extern crate actix;
extern crate actix_web;

use kullanicilar::actix::Handler;
use kullanicilar::actix::SyncContext;
use kullanicilar::actix::Actor;
use kullanicilar::actix::Message;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use models::Kullanici;
use schema::kullanici::dsl::*;
use super::schema::kullanici;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct KullaniciSorgula{
    pub atlanacak_kayit_sayisi: u32,
    pub gosterilecek_kayit_sayisi: u32,
    pub adi: Option<String>,
}

impl Message for KullaniciSorgula {
    type Result = Result<Vec<Kullanici>, Error>;
}    

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<KullaniciSorgula> for DbExecutor {
    type Result = Result<Vec<Kullanici>, Error>;

    fn handle(&mut self, msg: KullaniciSorgula, _: &mut Self::Context) -> Self::Result {

        let baglanti: &PgConnection = &self.0.get().unwrap();

        let mut sorgu = kullanici.into_boxed();
        
        if let Some(ad) = msg.adi {
            sorgu = sorgu.filter(adi.eq(ad));
        }
        
        let kayitlar = sorgu
            .limit(msg.gosterilecek_kayit_sayisi as i64)
            .offset(msg.atlanacak_kayit_sayisi as i64)
            .load::<Kullanici>(baglanti)
            .expect("Kullanıcılar yüklenirken hata oluştu!");

        Ok(kayitlar)
    }
}

#[derive(Insertable)]
#[table_name="kullanici"]
pub struct KullaniciEkle {
    pub adi: String,
    pub soyadi: String,
    pub kullanici_adi: String,
    pub sifre: String,
    pub eposta: String,
}

impl Message for KullaniciEkle {
    type Result = Result<Kullanici, Error>;
}

impl Handler<KullaniciEkle> for DbExecutor {
    type Result = Result<Kullanici, Error>;

    fn handle(&mut self, msg: KullaniciEkle, _: &mut Self::Context) -> Self::Result {

        let baglanti: &PgConnection = &self.0.get().unwrap();

        let kayit_edilen_id: i32 = diesel::insert_into(kullanici)
            .values(&msg)
            .returning(id)
            .get_result(baglanti)
            .expect("Kullanıcı oluşturulurken hata oluştu!");

        let mut kayit = kullanici
            .filter(id.eq(&kayit_edilen_id))
            .load::<Kullanici>(baglanti)
            .expect("Kullanıcı Oluşturuldu. Fakat kullanıcı bilgileri okunamadı!");

        Ok(kayit.pop().unwrap())
    }
}

impl Message for Kullanici {
    type Result = Result<Kullanici, Error>;
}

impl Handler<Kullanici> for DbExecutor {
    type Result = Result<Kullanici, Error>;

    fn handle(&mut self, msg: Kullanici, _: &mut Self::Context) -> Self::Result  {

        let baglanti: &PgConnection = &self.0.get().unwrap();

        let guncellenen_kullanici = diesel::update(kullanici.find(msg.id))
            .set(&msg)
            .get_result::<Kullanici>(baglanti)?;
        
        Ok(guncellenen_kullanici)
    }
}

pub struct KullaniciSil {
    pub id: u32,
}

impl Message for KullaniciSil {
    type Result = Result<bool, Error>;
}

impl Handler<KullaniciSil> for DbExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: KullaniciSil, _: &mut Self::Context) -> Self::Result  {

        let baglanti: &PgConnection = &self.0.get().unwrap();

        let silinen_kayit_sayisi = diesel::delete(kullanici.filter(id.eq(msg.id as i32)))
            .execute(baglanti)
            .expect("Kullanıcı silinirken hata oluştu!");
        
        Ok(silinen_kayit_sayisi == 1)
    }
}



