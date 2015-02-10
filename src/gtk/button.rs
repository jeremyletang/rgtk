use std::ffi::CString;
use glib::traits::{MutCast, Downcast, MutDowncast, AsPtr, FromPtr, ObjectTrait, GetGType};
use gtk::ffi;
use super::container::ContainerTrait;

pub trait ButtonTrait: ContainerTrait {
}

impl <T> ButtonTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkButton>  + MutCast<ffi::C_GtkContainer> + MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject> {

}

struct_skel!(Button, ffi::C_GtkButton);

impl Button {
    pub fn new_with_label(label: &str) -> Button {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            FromPtr::from_ptr(
                ffi::gtk_button_new_with_label(c_str.as_ptr()).force_downcast())
        }
    }
}

