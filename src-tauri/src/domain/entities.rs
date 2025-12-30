use crate::helpers::serde_helpers::string_or_null;
use serde::{Deserialize, Serialize};

/// Entity untuk satu kota/kabupaten
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kota {
    #[serde(deserialize_with = "string_or_null")]
    pub id: String,

    #[serde(deserialize_with = "string_or_null")]
    pub lokasi: String,
}

/// Response list kota
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaftarKotaResponse {
    pub status: bool,
    pub data: Vec<Kota>,
}

/// Koordinat lokasi (optional, kalau nanti mau pakai)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Koordinat {
    pub lat: f64,
    pub lon: f64,
    pub lintang: String,
    pub bujur: String,
}

/// Jadwal sholat satu hari
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JadwalSholat {
    pub tanggal: String,
    pub imsak: String,
    pub subuh: String,
    pub terbit: String,
    pub dhuha: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
    pub date: String,
}

/// Data utama jadwal
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JadwalData {
    pub id: String,
    pub lokasi: String,
    pub daerah: String,
    pub koordinat: Koordinat,
    pub jadwal: JadwalSholat,
}

/// Response jadwal harian
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JadwalResponse {
    pub status: bool,
    pub data: JadwalData,
}
