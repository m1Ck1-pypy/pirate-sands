use crate::gui::MyGuiApp;
use egui::Key;

pub fn inputs_gui(ctx: &egui::Context, my_gui: &mut MyGuiApp) {
    ctx.input(|inpt| {
        if inpt.key_pressed(Key::S) || inpt.key_pressed(Key::Space) {
            my_gui.toggle_timer();
        }
        if inpt.key_pressed(Key::R) {
            my_gui.reset_timer();
        }
        if inpt.key_pressed(Key::Q) {
            my_gui.allow_exit = true;
        }
        if inpt.key_pressed(Key::A) {
            my_gui.set_one_minute();
        }
    });
}
