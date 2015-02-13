use std::ffi::CString;
use glib::traits::{Cast, Downcast, AsPtr, FromPtr, Envelope};
use gtk::ffi::{self, to_bool};

pub trait ToggleButtonTrait {
    fn get_active(&self) -> bool;
}

impl <T> ToggleButtonTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkToggleButton> {
    fn get_active(&self) -> bool {
        unsafe { to_bool(ffi::gtk_toggle_button_get_active(self.as_ptr().cast())) }
    }
}

pub struct ToggleButton;

impl ToggleButton {
    pub fn new_with_label(label: &str) -> Envelope<ffi::C_GtkToggleButton> {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            FromPtr::from_floating_ptr(
                ffi::gtk_toggle_button_new_with_label(c_str.as_ptr()).unchecked_downcast())
        }
    }
}

