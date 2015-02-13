use glib::traits::{AsPtr, Cast};
use gtk::ffi;

pub trait WidgetTrait {
    fn show_all(&self);
}

impl <T> WidgetTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkWidget> {
    fn show_all(&self) {
        unsafe {
            ffi::gtk_widget_show_all(self.as_ptr().cast());
        }
    }
}
