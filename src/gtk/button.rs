use std::ffi::CString;
use glib::traits::{Cast, Downcast, AsPtr, FromPtr, Envelope};
use gtk::ffi::{self};

pub trait ButtonTrait { }

impl <T> ButtonTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkButton>
{ }

pub struct Button;

impl Button {
    pub fn new_with_label(label: &str) -> Envelope<ffi::C_GtkButton> {
        unsafe {
            let c_str = CString::from_slice(label.as_bytes());
            FromPtr::from_floating_ptr(
                ffi::gtk_button_new_with_label(c_str.as_ptr()).unchecked_downcast())
        }
    }

}
