extern crate gtk;

use gtk::prelude::*;
use gtk::{Entry, WindowPosition, Orientation, Button, Window, WindowType};

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);

    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    let input = Entry::new();
    let button = Button::new();

    vbox.pack_start(&input, true, false, 0);
    vbox.pack_start(&button, true, true, 0);

    window.set_position(WindowPosition::Center);
    window.set_default_size(1024, 600);

    window.add(&vbox);


    window.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
