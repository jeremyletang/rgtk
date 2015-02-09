use glib::traits::{GetGType, ObjectTrait, AsPtr, MutCast, Cast};
use gtk::ffi;

pub trait WidgetTrait: ObjectTrait {
    fn show_all(&mut self);
}

impl <T> WidgetTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject>,
      *const <T as AsPtr>::Inner: Cast<ffi::C_GtkWidget>  + Cast<::glib::ffi::C_GObject> {

    fn show_all(&mut self) {
        unsafe {
            ffi::gtk_widget_show_all(self.as_mut_ptr().cast());
        }
    }
}

