extern crate actix;
extern crate actix_web;
extern crate kullaniciservisi;
extern crate diesel;
extern crate futures;

extern crate log;
extern crate env_logger;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate r2d2;

use actix::Addr;
use actix::SyncArbiter;

use actix_web::AsyncResponder;
use actix_web::FutureResponse;
use actix_web::HttpResponse;
use actix_web::Path;
use actix_web::Query;
use actix_web::State;

use actix_web::server;
use actix_web::App;
use actix_web::Json;
use actix_web::http;
use actix_web::middleware::Logger;

use kullaniciservisi::kullanicilar;
use futures::Future;

use kullanicilar::DbExecutor;
use kullanicilar::KullaniciSorgula;
use kullanicilar::KullaniciEkle;
use kullanicilar::KullaniciSil;

use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;

use kullaniciservisi::models::Kullanici;
use dotenv::dotenv;
use std::env;

/// DbExecutor adresini uygulama durumuna ekliyor.
struct AppState {
    db: Addr<DbExecutor>,
}

#[derive(Deserialize)]
struct Bilgi {
    ilk_sayfa_no: u32,
    gosterilecek_kayit_sayisi: u32,
    adi: Option<String>,
}

#[derive(Deserialize)]
struct KullaniciBilgi {
    adi  : String,
    soyadi : String,
    kullanici_adi   : String,
    sifre      : String,
    eposta      : String,
}

/// Metot kullanıcıları yükler
/// İstekleri asenkron olarak cevaplar
fn kullanici_listele_async(
        (sorguparametre, state): (Query<Bilgi>, State<AppState>),
    ) -> FutureResponse<HttpResponse> {
    // DbExecutor'a asenkron olarak KullaniciSorgula isteklerini iletir. 
    state
        .db
        .send(KullaniciSorgula {
            atlanacak_kayit_sayisi: sorguparametre.ilk_sayfa_no,
            gosterilecek_kayit_sayisi: sorguparametre.gosterilecek_kayit_sayisi,
            adi: sorguparametre.adi.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(kullanici) => Ok(HttpResponse::Ok().json(kullanici)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn kullanici_olustur(
                  (kullanicibilgi, state): (Query<KullaniciBilgi>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // DbExecutor'a asenkron olarak KullaniciEkle isteklerini iletir.
    state
        .db
        .send(KullaniciEkle
              { adi  : kullanicibilgi.adi.to_string(),
                soyadi : kullanicibilgi.soyadi.to_string(),
                kullanici_adi   : kullanicibilgi.kullanici_adi.to_string(),
                sifre   : kullanicibilgi.sifre.to_string(),
		        eposta      : kullanicibilgi.eposta.to_string(),
              })
        .from_err()
        .and_then(|res| match res {
            Ok(kullanici) => Ok(HttpResponse::Ok().json(kullanici)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn kullanici_guncelle(
                 (kullanicibilgi, state): (Query<Kullanici>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    let guncellenecek_kullanici = Kullanici {
        ..kullanicibilgi.into_inner()
    };

    println!("{:#?}", guncellenecek_kullanici);
    
    state
        .db
        .send(guncellenecek_kullanici)
        .from_err()
        .and_then(|res| match res {
            Ok(guncellenen_kullanici) => Ok(HttpResponse::Ok().json(guncellenen_kullanici)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

#[derive(Serialize)]
pub struct SilSonuc {
    sonuc: bool,
}

/// Kullanıcı silme isteklerini cevaplar.
fn kullanici_sil(
                 (path, state): (Path<(u32)>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    state
        .db
        .send(KullaniciSil {id: path.into_inner()})
        .from_err()
        .and_then(|res| match res {
            Ok(silmesonucu) => Ok(HttpResponse::Ok().json(SilSonuc{ sonuc : silmesonucu})),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let kullanici_sistem = actix::System::new("kullanici");

    let veritabani_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL bir değere sahip olmalıdır.");

    // DB Executor'ın konfigürasyon ve başlatma işlemlerini gerçekleştirir.
    let manager = ConnectionManager::<PgConnection>::new(veritabani_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Uygulama havuzu oluşuturulurken hata oluştu!");

    let addr = SyncArbiter::start(12, move || DbExecutor(pool.clone()));

    // Dış uygulamaların ulaşması için yeni bir servis ekler
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            .prefix("/app")
            .scope("/kullanici", |acc_scope| {
                acc_scope
                    .resource("", |r| {
                        r.method(http::Method::GET).with(kullanici_listele_async);
                        r.method(http::Method::POST).with(kullanici_olustur);
                        r.method(http::Method::PUT).with(kullanici_guncelle)
                    })
                    .resource("/{kullanici_id}", |r| {
                        r.method(http::Method::DELETE).with(kullanici_sil)
                    })        
            })
            
    })
        .bind("localhost:8000")
        .unwrap()
        .start();

    println!("Uygulama başladı: localhost:8000");
    let _ = kullanici_sistem.run();

}