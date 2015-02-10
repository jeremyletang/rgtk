use glib::traits::{GetGType, ObjectTrait, AsPtr, Cast};
use gtk::ffi;

pub trait WidgetTrait: ObjectTrait {
    fn show_all(&self);
}

impl <T> WidgetTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkWidget>  + Cast<::glib::ffi::C_GObject> {

    fn show_all(&self) {
        unsafe {
            ffi::gtk_widget_show_all(self.as_ptr().cast());
        }
    }
}

