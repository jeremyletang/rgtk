#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(350, 70);
    //window.destroy.connect (Gtk.main_quit);

    window.connect(signals::DeleteEvent::new(|_|{
        gtk::main_quit();
        true
    }));

    let button = gtk::Button::new_with_label("Click me!").unwrap();
    // button.clicked.connect (() => {
    //     button.label = "Thank you";
    // });

	window.add(&button);

    window.show_all();
    gtk::main();
}