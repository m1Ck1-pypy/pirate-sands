# [Pirate Sands]

[A lightweight desktop application built with Rust, featuring a modern UI, audio playback capabilities, and system notifications.]

![Screenshot APP](https://github.com/user-attachments/assets/d437f0f0-e67e-4689-ba09-3addf28eedb9)

## Особенности
- 🖥️ Modern GUI using egui
- 🔊 Audio playback through rodio with support for common formats (MP3, WAV, FLAC)
- 🔔 System notifications via notify-rust (supports Windows, macOS, Linux)
- 📦 Single-binary deployment
- 🔒 Memory-safe implementation thanks to Rust

## Технологии
- Rust 1.70+
- [egui](https://github.com/emilk/egui) 0.24+ (UI)
- [rodio](https://github.com/RustAudio/rodio) 0.17+ (sound notification)
- [notify-rust](https://github.com/hoodie/notify-rust) 4.8+ (window notification)

## Install
### Source code build
```bash
https://github.com/m1Ck1-pypy/pirate-sands.git
cd pirate-sands
cargo build --release
