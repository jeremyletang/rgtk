
#![crate_id = "test"]
#![feature(globs)]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let mut window = gtk::Window::new(gtk::window_type::TopLevel);
    let mut frame = gtk::Frame::new(Some("Yep a frame"));
    let mut _box = gtk::_Box::new(gtk::orientation::Horizontal, 10);
    let mut v_box = gtk::_Box::new(gtk::orientation::Horizontal, 10);
    let mut button_box = gtk::ButtonBox::new(gtk::orientation::Horizontal);
    let mut label = gtk::Label::new("Yeah a wonderful label too !");
    let mut button = gtk::Button::new_with_label("Whattttt a button !");
    let font_button = gtk::FontButton::new();
    let toggle_button = gtk::ToggleButton::new_with_label("Toggle Me !");
    let check_button = gtk::CheckButton::new_with_label("Labeled check button");
    let color_button = gtk::ColorButton::new();
    let menu_button = gtk::MenuButton::new();
    let link_button = gtk::LinkButton::new("www.rust-lang.org");
    let mut volume_button = gtk::VolumeButton::new();
    let mut entry = gtk::Entry::new();
    let search_entry = gtk::SearchEntry::new();
    let separator = gtk::Separator::new(gtk::orientation::Horizontal);
    let separator2 = gtk::Separator::new(gtk::orientation::Horizontal);
    let switch = gtk::Switch::new();
    let mut switch2 = gtk::Switch::new();
    let scale = gtk::Scale::new_with_range(gtk::orientation::Horizontal, 0., 100., 1.);
    let mut level_bar = gtk::LevelBar::new_for_interval(0., 100.);
    let spin_button = gtk::SpinButton::new_with_range(0., 100., 1.);
    let mut spinner = gtk::Spinner::new();
    let image = gtk::Image::new_from_file("./test/resources/gtk.jpg");
    let mut progress_bar = gtk::ProgressBar::new();
    let arrow = gtk::Arrow::new(gtk::arrow_type::Right, gtk::shadow_type::EtchedOut);
    let calendar = gtk::Calendar::new();
    let mut info_bar = gtk::InfoBar::new();

    println!("test");

    info_bar.show_close_button(true);

    /*info_bar.connect(signals::Response::new(|response_id| {
        info_bar.hide()
    }));*/ //TODO: Why does this not work?

    progress_bar.set_fraction(0.7);
    spinner.start();
    level_bar.set_value(37.);
    switch2.set_active(true);
    frame.set_border_width(10);
    _box.set_border_width(5);
    entry.set_placeholder("An Entry with a placeholder !");
    volume_button.set_orientation(gtk::orientation::Horizontal);
    label.set_justify(gtk::justification::Left);
    window.set_title("Yeah a beautiful window with rgtk !");
    window.add(&frame);

    button.connect(signals::Clicked::new(||{
        //entry.set_text("Clicked!".to_string());
        let dialog = gtk::MessageDialog::new_with_markup(None, gtk::dialog_flags::Modal, gtk::message_type::Info,
            gtk::buttons_type::OkCancel, "This is a trap !");

        dialog.run();
    }));

    window.connect(signals::DeleteEvent::new(|_|{
        gtk::main_quit();
        true
    }));

    frame.add(&_box);
    button_box.add(&button);
    button_box.add(&font_button);
    button_box.add(&toggle_button);
    button_box.add(&color_button);
    button_box.add(&volume_button);
    v_box.add(&switch);
    v_box.add(&menu_button);
    v_box.add(&switch2);
    v_box.add(&check_button);
    v_box.add(&link_button);
    v_box.add(&spin_button);
    _box.add(&info_bar);
    _box.add(&v_box);
    _box.add(&scale);
    _box.add(&level_bar);
    _box.add(&button_box);
    _box.add(&progress_bar);
    _box.add(&separator);
    _box.add(&label);
    _box.add(&entry);
    _box.add(&separator2);
    _box.add(&search_entry);
    _box.add(&spinner);
    _box.add(&image);
    _box.add(&arrow);
    _box.add(&calendar);
    _box.set_orientation(gtk::orientation::Vertical);

    window.show_all();
    gtk::main();
}
