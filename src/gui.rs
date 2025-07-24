#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use crate::app::{Timer, TimerState, TimerStateType};
use crate::inputs::inputs_gui;
use crate::text_lib::{Titles, Translator};
use eframe::egui::Color32;
use egui::epaint::{
    CornerRadius,
    text::{FontInsert, InsertFontFamily},
};
use egui::{Vec2, ViewportCommand};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub fn app_gui(timer: TimerStateType) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 430.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Pirate sands Timer",
        options,
        Box::new(|cc| Ok(Box::new(MyGuiApp::new(cc, timer)))),
    )
}

struct ButtonProps<'a> {
    text: &'a str,
    size: Option<Vec2>,
    color: Option<Color32>,
    radius: Option<CornerRadius>,
}

pub struct MyGuiApp {
    pub minutes: u64,
    pub last_tick: Instant,
    pub next_repaint: Instant,
    pub timer: Arc<Mutex<Timer>>,
    pub allow_exit: bool,
    pub is_sound_notify: bool,
    pub system_lang: Translator,
}

impl MyGuiApp {
    fn new(cc: &eframe::CreationContext<'_>, timer: TimerStateType) -> Self {
        let (minutes, _seconds) = {
            let locked = timer.lock().unwrap();
            (
                locked.remaining.as_secs() / 60,
                locked.remaining.as_secs() % 60,
            )
        };

        let now = Instant::now();
        let system_lang = Translator::default();

        cc.egui_ctx.set_zoom_factor(0.9);
        add_font(&cc.egui_ctx);
        Self {
            minutes,
            timer,
            last_tick: now,
            next_repaint: now + Duration::from_secs(1),
            allow_exit: false,
            is_sound_notify: true,
            system_lang,
        }
    }

    pub fn format_time(&self) -> String {
        let timer = self.timer.lock().unwrap();
        let total_secs = timer.remaining.as_secs();
        format!("{:02}:{:02}", total_secs / 60, total_secs % 60)
    }

    pub fn update_minutes(&mut self) {
        let mut timer = self.timer.lock().unwrap();
        timer.update_minutes(self.minutes);
    }

    pub fn current_state(&self) -> TimerState {
        self.timer.lock().unwrap().state
    }

    pub fn toggle_timer(&mut self) {
        let mut timer = self.timer.lock().unwrap();
        timer.sound_notify = self.is_sound_notify;
        timer.toggle();
    }

    pub fn reset_timer(&mut self) {
        let mut timer = self.timer.lock().unwrap();
        timer.reset(self.minutes);
    }

    pub fn set_one_minute(&mut self) {
        let mut timer = self.timer.lock().unwrap();

        if timer.state == TimerState::Stopped {
            self.minutes += 1;
        }
        timer.set_one_minute();
    }

    pub fn get_translator_str(&self, title: Titles) -> &'static str {
        self.system_lang.translate(title)
    }
}

impl eframe::App for MyGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if now - self.last_tick >= Duration::from_secs(1) {
            if self.current_state() == TimerState::Running {
                let mut timer = self.timer.lock().unwrap();
                timer.tick();
            }
            self.last_tick = now;
        }

        self.next_repaint = now + Duration::from_millis(100);

        let no_active_timer = self.current_state() == TimerState::Stopped;
        let state_text = current_state_text(self);
        let state_color = color_from_state(self);

        inputs_gui(ctx, self);

        if self.allow_exit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        let button = |props: ButtonProps| {
            let size = props.size.unwrap_or(Vec2::new(100.0, 50.0));
            let fill = props.color.unwrap_or(Color32::LIGHT_GREEN);
            let radius = props.radius.unwrap_or(CornerRadius::same(12));

            egui::Button::new(
                egui::RichText::new(props.text)
                    .color(Color32::BLACK)
                    .size(20.0),
            )
            .fill(fill)
            .corner_radius(radius)
            .min_size(size)
        };

        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(15.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(self.get_translator_str(Titles::Header))
                                .heading()
                                .size(42.0),
                        )
                        .wrap(),
                    );
                    ui.add_space(15.0);

                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(self.get_translator_str(Titles::Subheader))
                                .heading()
                                .size(20.0)
                                .color(Color32::GOLD),
                        )
                        .wrap(),
                    );
                    ui.add_enabled_ui(no_active_timer, |ui| {
                        ui.vertical_centered(|ui_slider| {
                            if ui_slider
                                .add_sized(
                                    [150., 50.],
                                    egui::Slider::new(&mut self.minutes, 0..=120)
                                        .text(self.system_lang.translate(Titles::Slider))
                                        .show_value(true),
                                )
                                .changed()
                            {
                                self.update_minutes();
                            };
                        });
                    });

                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(self.format_time()).heading().size(96.0),
                        )
                        .wrap(),
                    );

                    ui.add(egui::Checkbox::new(
                        &mut self.is_sound_notify,
                        self.system_lang.translate(Titles::SoundNotification),
                    ));

                    ui.add_space(10.0);
                    if no_active_timer {
                        if ui
                            .add(button(ButtonProps {
                                text: self.get_translator_str(Titles::StartButton),
                                size: Some(Vec2::new(230.0, 50.0)),
                                color: None,
                                radius: None,
                            }))
                            .clicked()
                        {
                            self.toggle_timer();
                        };
                    } else if ui
                        .add(button(ButtonProps {
                            text: self.get_translator_str(Titles::AddMinutes),
                            size: Some(Vec2::new(230.0, 50.0)),
                            color: Some(Color32::ORANGE),
                            radius: None,
                        }))
                        .clicked()
                    {
                        self.set_one_minute();
                    };

                    ui.style_mut().spacing.button_padding = egui::vec2(20.0, 15.0);
                    ui.add_space(15.0);

                    ui.label(
                        egui::RichText::new(state_text)
                            .color(state_color)
                            .size(18.0),
                    );

                    ui.add_space(20.0);
                    ui.add(
                        egui::Label::new(egui::RichText::new(
                            self.get_translator_str(Titles::Help),
                        ))
                        .wrap(),
                    );
                })
            });
        });
    }
}

fn current_state_text(state: &mut MyGuiApp) -> String {
    match state.current_state() {
        TimerState::Running => state
            .system_lang
            .translate(Titles::TimerRunning)
            .to_string(),
        TimerState::Paused => state.system_lang.translate(Titles::TimerPaused).to_string(),
        TimerState::Stopped => state
            .system_lang
            .translate(Titles::TimerStopped)
            .to_string(),
    }
}

fn color_from_state(state: &mut MyGuiApp) -> Color32 {
    match state.current_state() {
        TimerState::Running => Color32::GREEN,
        TimerState::Paused => Color32::YELLOW,
        TimerState::Stopped => Color32::RED,
    }
}

fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("../assets/emoji-icon-font.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}
