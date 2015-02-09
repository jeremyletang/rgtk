use libc::c_uint;
use glib::traits::{AsPtr, MutCast, Cast, GetGType};
use gtk::ffi;
use super::widget::WidgetTrait;

pub trait ContainerTrait: WidgetTrait {
    fn add<W>(&mut self, widget: &mut W)
        where W: AsPtr, *mut <W as AsPtr>::Inner: MutCast<ffi::C_GtkWidget>;
    fn set_border_width(&mut self, border_width: u32);
}

impl <T> ContainerTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkContainer> + MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject>,
      *const <T as AsPtr>::Inner: Cast<ffi::C_GtkContainer> + Cast<ffi::C_GtkWidget>  + Cast<::glib::ffi::C_GObject> {

    fn add<W>(&mut self, widget: &mut W)
    where W: AsPtr, *mut <W as AsPtr>::Inner: MutCast<ffi::C_GtkWidget> {
        unsafe {
            ffi::gtk_container_add(self.as_mut_ptr().cast(), widget.as_mut_ptr().cast());
        }
    }

    fn set_border_width(&mut self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.as_mut_ptr().cast(), border_width as c_uint);
        }
    }
}

