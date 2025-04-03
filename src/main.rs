use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::thread;
use std::sync::{Arc, Mutex};

struct GameFeatures {
    aimbot: bool,
    speed_hack: bool,
    god_mode: bool,
}

impl GameFeatures {
    fn new() -> Self {
        GameFeatures {
            aimbot: false,
            speed_hack: false,
            god_mode: false,
        }
    }

    fn toggle_aimbot(&mut self) {
        self.aimbot = !self.aimbot;
    }

    fn toggle_speed_hack(&mut self) {
        self.speed_hack = !self.speed_hack;
    }

    fn toggle_god_mode(&mut self) {
        self.god_mode = !self.god_mode;
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let features = Arc::new(Mutex::new(GameFeatures::new()));
    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTA 5 Multi");
    window.set_default_size(300, 200);

    let label = Label::new(Some("GTA 5 Multi Features"));
    let aimbot_button = Button::with_label("Toggle Aimbot");
    let speed_hack_button = Button::with_label("Toggle Speed Hack");
    let god_mode_button = Button::with_label("Toggle God Mode");

    let features_clone = Arc::clone(&features);
    aimbot_button.connect_clicked(move |_| {
        let mut features = features_clone.lock().unwrap();
        features.toggle_aimbot();
    });

    let features_clone = Arc::clone(&features);
    speed_hack_button.connect_clicked(move |_| {
        let mut features = features_clone.lock().unwrap();
        features.toggle_speed_hack();
    });

    let features_clone = Arc::clone(&features);
    god_mode_button.connect_clicked(move |_| {
        let mut features = features_clone.lock().unwrap();
        features.toggle_god_mode();
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&aimbot_button, true, true, 0);
    vbox.pack_start(&speed_hack_button, true, true, 0);
    vbox.pack_start(&god_mode_button, true, true, 0);
    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}