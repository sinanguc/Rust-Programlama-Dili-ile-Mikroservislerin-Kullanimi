-- SQL Kodları buraya yazılacak

CREATE TABLE kullanici (
  id SERIAL PRIMARY KEY,
  adi VARCHAR NOT NULL,
  soyadi VARCHAR NOT NULL,
  kullanici_adi VARCHAR NOT NULL,
  sifre VARCHAR NOT NULL,
  eposta VARCHAR NOT NULL
);
