use adzan_reminder_lib::{AppConfig, PrayerService};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use skim::prelude::*;
use skim::Skim;
use std::io::Cursor;

const BANNER: &str = r#"
▄▖ ▌        ▄▖     ▘   ▌      ▄▖▖ ▄▖
▌▌▛▌▀▌▀▌▛▌  ▙▘█▌▛▛▌▌▛▌▛▌█▌▛▘  ▌ ▌ ▐
▛▌▙▌▙▖█▌▌▌  ▌▌▙▖▌▌▌▌▌▌▙▌▙▖▌   ▙▖▙▖▟▖
"#;

#[tokio::main]
async fn main() {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();

    loop {
        term.clear_screen().unwrap();
        println!("{}", console::style(BANNER).cyan().bold());
        println!("{}", console::style("Adzan Reminder CLI").green().bold());
        println!();

        let items = vec![
            "Tampilkan jadwal hari ini",
            "Pilih kota",
            "Lihat kota terpilih",
            "Keluar",
        ];

        let selection = Select::with_theme(&theme)
            .items(&items)
            .default(0)
            .interact_on_opt(&term)
            .unwrap();

        match selection {
            Some(0) => show_today_schedule().await,
            Some(1) => set_city_interactive().await,
            Some(2) => show_current_city().await,
            Some(3) => {
                println!("Keluar dari aplikasi. Semoga bermanfaat! 🕌");
                break;
            }
            None => break,
            _ => unreachable!(),
        }

        println!();
        println!("Tekan Enter untuk kembali ke menu...");
        let _ = term.read_line();
    }
}

async fn show_today_schedule() {
    let config = AppConfig::load().unwrap_or_default();

    match config.selected_city_id {
        Some(id) => {
            let service = PrayerService::new();
            let city_name = config
                .selected_city_name
                .as_deref()
                .unwrap_or("Kota tidak diketahui");

            let mut sp = spinners::Spinner::new(
                spinners::Spinners::Dots9,
                format!("Mengambil jadwal untuk {}...", city_name),
            );
            match service.get_today_schedule(id.as_str()).await {
                Ok(schedule) => {
                    let lokasi = &schedule.data.kabko;
                    sp.stop_with_message("✅ Jadwal berhasil dimuat!\n".to_string());

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
                Err(e) => {
                    sp.stop_with_message(format!("❌ Gagal fetch jadwal: {}\n", e));
                }
            }
        }
        None => {
            println!("Belum ada kota yang dipilih.");
            return;
        }
    }
}

async fn show_current_city() {
    let config = AppConfig::load().unwrap_or_default();
    match config.selected_city_id {
        Some(id) => {
            let name = config
                .selected_city_name
                .as_deref()
                .unwrap_or("Tidak diketahui");
            println!("Kota terpilih: {} ({})", name, id);
        }
        None => println!("Belum ada kota yang dipilih."),
    }
}

async fn set_city_interactive() {
    let mut sp = spinners::Spinner::new(
        spinners::Spinners::Dots,
        "Mengambil list kota dari API...".to_string(),
    );

    let service = PrayerService::new();
    let cities = match service.get_cities().await {
        Ok(c) => {
            sp.stop_with_message("✅ List kota berhasil dimuat!\n".to_string());
            c
        }
        Err(e) => {
            sp.stop_with_message(format!("❌ Gagal fetch list kota: {}\n", e));
            return;
        }
    };

    // Format items sebagai string sederhana (nama kota)
    let input_bytes: Vec<u8> = cities
        .iter()
        .map(|c| format!("{}\n", c.lokasi))
        .collect::<String>()
        .into_bytes();

    let options = SkimOptionsBuilder::default()
        .height("70%".into())
        .multi(false)
        .prompt("Cari kota: ".into())
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input_bytes));

    let selected = Skim::run_with(&options, Some(items));

    if let Some(output) = selected {
        if output.is_abort {
            println!("Pemilihan dibatalkan.");
            return;
        }

        if let Some(selected_line) = output.selected_items.first() {
            let selected_name = selected_line.output().to_string();

            // Cari kota berdasarkan nama (karena output hanya nama)
            if let Some(chosen) = cities.iter().find(|c| c.lokasi == selected_name) {
                let mut config = AppConfig::load().unwrap_or_default();
                config.selected_city_id = Some(chosen.id.clone());
                config.selected_city_name = Some(chosen.lokasi.clone());

                if let Err(e) = config.save() {
                    eprintln!("Gagal simpan config: {}", e);
                } else {
                    println!("\n✅ Kota berhasil disimpan: {}", chosen.lokasi);
                }
            }
        }
    }
}
