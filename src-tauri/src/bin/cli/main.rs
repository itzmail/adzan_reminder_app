use adzan_reminder_lib::{AppConfig, PrayerService};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

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
    let service = PrayerService::new();
    let cities = match service.get_cities().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Gagal fetch list kota: {}", e);
            return;
        }
    };

    let city_names: Vec<String> = cities.iter().map(|c| c.lokasi.clone()).collect();

    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let selection = Select::with_theme(&theme)
        .with_prompt("Pilih kota:")
        .items(&city_names)
        .default(0)
        .interact_on_opt(&term)
        .unwrap();

    if let Some(index) = selection {
        let chosen = &cities[index];

        let mut config = AppConfig::load().unwrap_or_default();
        config.selected_city_id = Some(chosen.id.clone());
        config.selected_city_name = Some(chosen.lokasi.clone());

        if let Err(e) = config.save() {
            eprintln!("Gagal simpan config: {}", e);
        } else {
            println!("Kota berhasil disimpan: {}", chosen.lokasi);
        }
    }
}
