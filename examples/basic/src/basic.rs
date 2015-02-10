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

    let window = Window::new(gtk::WindowType::TopLevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    Connect_::connect(&window, DeleteEvent::new(&mut |_| {
        gtk::main_quit();
        true
    }));

    let bx = Box_::new(gtk::Orientation::Vertical, 10);
    let cb = CheckButton::new_with_label("Exit");
    let button = Button::new_with_label("Click me!");

    bx.add(&cb);
    bx.add(&button);

    window.add(&bx);

    Connect_::connect(&button, Clicked::new(&mut || {
        if cb.get_active() {
            gtk::main_quit();
        }
    }));

    window.show_all();
    gtk::main();
}
