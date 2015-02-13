use std::ffi::CString;
use glib::traits::{Cast, Downcast, AsPtr, FromPtr, Envelope};
use gtk::ffi;

pub trait CheckButtonTrait { }

impl <T> CheckButtonTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkCheckButton>
{ }

pub struct CheckButton;

impl CheckButton {
    pub fn new_with_label(label: &str) -> Envelope<ffi::C_GtkCheckButton> {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            FromPtr::from_floating_ptr(
                ffi::gtk_check_button_new_with_label(c_str.as_ptr()).unchecked_downcast())
        }
    }
}

