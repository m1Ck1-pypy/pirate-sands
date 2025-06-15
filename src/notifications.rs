use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Source};

pub fn show_notification() {
    if let Err(e) = Notification::new()
        .summary("Таймер закончился. Сделайте перерыв")
        .body("Время вышло")
        .icon("clock")
        .show()
    {
        eprintln!("Ошибка уведомления! {}", e);
    }
}

pub fn notify_sound() {
    let sound_data = include_bytes!("../assets/notify.wav");

    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(source) = Decoder::new(std::io::Cursor::new(sound_data)) {
            let _ = stream_handle.play_raw(source.convert_samples().amplify(0.5));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
