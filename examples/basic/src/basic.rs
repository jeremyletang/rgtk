//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate rgtk;

use rgtk::glib::traits::Connect_;
use rgtk::gtk;
use rgtk::gtk::signals::{DeleteEvent, Clicked};
use rgtk::gtk::widget::WidgetTrait;
use rgtk::gtk::container::ContainerTrait;
use rgtk::gtk::box_::{Box_};
use rgtk::gtk::button::{Button};
use rgtk::gtk::toggle_button::{ToggleButtonTrait};
use rgtk::gtk::check_button::{CheckButton};
use rgtk::gtk::window::{WindowTrait, Window};

fn main() {
    gtk::init();

    let mut window = Window::new(gtk::WindowType::TopLevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    Connect_::connect(&mut window, DeleteEvent::new(&mut |_| {
        gtk::main_quit();
        true
    }));

    let mut bx = Box_::new(gtk::Orientation::Vertical, 10);
    let mut cb = CheckButton::new_with_label("Exit");
    let mut button = Button::new_with_label("Click me!");

    bx.add(&mut cb);
    bx.add(&mut button);

    window.add(&mut bx);

    Connect_::connect(&mut button, Clicked::new(&mut || {
        if cb.get_active() {
            gtk::main_quit();
        }
    }));

    window.show_all();
    gtk::main();
}
