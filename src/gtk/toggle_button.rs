use std::ffi::CString;
use glib::traits::{Cast, MutCast, Downcast, MutDowncast, AsPtr, FromPtr, ObjectTrait, GetGType};
use gtk::ffi::{self, to_bool};
use super::button::ButtonTrait;

pub trait ToggleButtonTrait: ButtonTrait {
    fn get_active(&self) -> bool;
}

impl <T> ToggleButtonTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkToggleButton>  + MutCast<ffi::C_GtkButton>  + MutCast<ffi::C_GtkContainer> + MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject> {

    fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_button_get_active(self.as_ptr().cast())) }
    }
}

struct_skel!(ToggleButton, ffi::C_GtkToggleButton);

impl ToggleButton {
    pub fn new_with_label(label: &str) -> ToggleButton {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            FromPtr::from_ptr(
                ffi::gtk_toggle_button_new_with_label(c_str.as_ptr()).force_downcast())
        }
    }
}

