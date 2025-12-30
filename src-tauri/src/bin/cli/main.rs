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
            let j = &schedule.data.jadwal;
            let lokasi = &schedule.data.lokasi;

            println!("Jadwal Sholat Hari Ini - {}", lokasi);
            println!("──────────────────────────────");
            println!("Subuh   : {}", j.subuh);
            println!("Dzuhur  : {}", j.dzuhur);
            println!("Ashar   : {}", j.ashar);
            println!("Maghrib : {}", j.maghrib);
            println!("Isya    : {}", j.isya);
        }
        Err(e) => eprintln!("Error fetch jadwal: {}", e),
    }
}
