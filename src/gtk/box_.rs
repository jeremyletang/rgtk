use libc::c_int;
use glib::traits::{Cast, MutCast, Downcast, MutDowncast, AsPtr, FromPtr, ObjectTrait, GetGType};
use gtk::ffi;
use gtk::Orientation;
use super::container::ContainerTrait;

pub trait BoxTrait: ContainerTrait {
}

impl <T> BoxTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkBox>  + MutCast<ffi::C_GtkContainer> + MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject>,
      *const <T as AsPtr>::Inner: Cast<ffi::C_GtkBox>  + Cast<ffi::C_GtkContainer> + Cast<ffi::C_GtkWidget>  + Cast<::glib::ffi::C_GObject> {

}

struct_skel!(Box_, ffi::C_GtkBox);

impl Box_ {
    pub fn new(orientation: Orientation, spacing: i32) -> Box_ {
        unsafe {
            FromPtr::from_ptr(
                ffi::gtk_box_new(orientation, spacing as c_int).force_downcast())
        }
    }
}

