use std::ffi::CString;
use glib::traits::{Cast, Downcast, AsPtr, FromPtr, Envelope};
use gtk::ffi;
use gtk::{WindowPosition, WindowType};

pub trait WindowTrait {
    fn set_default_size(&self, width: i32, height: i32);
    fn set_title(&self, title: &str);
    fn set_window_position(&self, window_position: WindowPosition);
    fn get_title(&self) -> Option<String>;
}

impl <T> WindowTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkWindow> {
    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.as_ptr().cast(), width, height)
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            let c_str = CString::from_slice(title.as_bytes());
            ffi::gtk_window_set_title(self.as_ptr().cast(), c_str.as_ptr());
        }
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.as_ptr().cast(), window_position);
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            let c_title = ffi::gtk_window_get_title(self.as_ptr().cast());

            if c_title.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_title)).to_string())
            }
        }
    }
}

pub struct Window;

impl Window {
    pub fn new(window_type: WindowType) -> Envelope<ffi::C_GtkWindow> {
        unsafe {
            FromPtr::from_floating_ptr(
                ffi::gtk_window_new(window_type).unchecked_downcast())
        }
    }
}
