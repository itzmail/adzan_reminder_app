use adzan_reminder_lib::PrayerService;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
      match args[1].as_str() {
        "today" => show_today_schedule().await,
        "cities" => show_city_count().await,
        _ => print_help(),
      }
    } else {
      print_help();
    }
}

fn print_help() {
  println!("Adzan Reminder CLI");
  println!("Perintah yang tersedia:");
  println!("   cities   - Lihat jumlah kota yang tersedia");
  println!("   today    - Tampilkan jadwal sholat hari ini (contoh Jakarta)");
  println!("Gunakan: adzan <perintah>");
}

async fn show_city_count() {
    let service = PrayerService::new();
    match service.get_cities().await {
        Ok(cities) => println!("Total kota tersedia: {} kota", cities.len()),
        Err(e) => eprintln!("Error fetch kota: {}", e),
    }
}

async fn show_today_schedule() {
    let service = PrayerService::new();

    let jakarta_id = "eda80a3d5b344bc40f3bc04f65b7a357";

    match service.get_today_schedule(jakarta_id).await {
        Ok(schedule) => {
            let lokasi = &schedule.data.kabko;

            println!("Jadwal Sholat Hari Ini - {}", lokasi);
            println!("──────────────────────────────");
            
            // Ambil jadwal untuk hari ini (ambil yang pertama dari HashMap)
            if let Some((_, jadwal_hari)) = schedule.data.jadwal.iter().next() {
                println!("Tanggal : {}", jadwal_hari.tanggal);
                println!("Imsak   : {}", jadwal_hari.imsak);
                println!("Subuh   : {}", jadwal_hari.subuh);
                println!("Terbit  : {}", jadwal_hari.terbit);
                println!("Dhuha   : {}", jadwal_hari.dhuha);
                println!("Dzuhur  : {}", jadwal_hari.dzuhur);
                println!("Ashar   : {}", jadwal_hari.ashar);
                println!("Maghrib : {}", jadwal_hari.maghrib);
                println!("Isya    : {}", jadwal_hari.isya);
            } else {
                println!("Tidak ada data jadwal tersedia");
            }
        }
        Err(e) => eprintln!("Error fetch jadwal: {}", e),
    }
}
