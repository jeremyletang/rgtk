use libc::c_uint;
use glib::traits::{AsPtr, Cast};
use gtk::ffi;

pub trait ContainerTrait {
    fn add<W>(&self, widget: &W)
        where W: AsPtr, *mut <W as AsPtr>::Inner: Cast<ffi::C_GtkWidget>;
    fn set_border_width(&self, border_width: u32);
}

impl <T> ContainerTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkContainer> {
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

