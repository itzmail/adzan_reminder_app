use notify_rust::Notification;

pub fn send_prayer_notification(prayer_name: &str, message: &str) {
    let result = Notification::new()
        .summary(&format!("Waktu Sholat {}", prayer_name))
        .body(message)
        .icon("appointment-soon")
        .timeout(notify_rust::Timeout::Milliseconds(12000))
        .show();

    if let Err(e) = result {
        eprintln!("Gagal kirim notifikiasi: {}", e);
    }
}
