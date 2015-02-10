use libc::c_uint;
use glib::traits::{AsPtr, Cast, GetGType};
use gtk::ffi;
use super::widget::WidgetTrait;

pub trait ContainerTrait: WidgetTrait {
    fn add<W>(&self, widget: &W)
        where W: AsPtr, *mut <W as AsPtr>::Inner: Cast<ffi::C_GtkWidget>;
    fn set_border_width(&self, border_width: u32);
}

impl <T> ContainerTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkContainer> + Cast<ffi::C_GtkWidget>  + Cast<::glib::ffi::C_GObject> {

    fn add<W>(&self, widget: &W)
    where W: AsPtr, *mut <W as AsPtr>::Inner: Cast<ffi::C_GtkWidget> {
        unsafe {
            ffi::gtk_container_add(self.as_ptr().cast(), widget.as_ptr().cast());
        }
    }

    fn set_border_width(&self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.as_ptr().cast(), border_width as c_uint);
        }
    }
}

