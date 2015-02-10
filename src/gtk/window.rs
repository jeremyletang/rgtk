use std::ffi::CString;
use glib::traits::{Cast, MutCast, Downcast, MutDowncast, AsPtr, FromPtr, ObjectTrait, GetGType};
use gtk::ffi;
use super::container::ContainerTrait;
use gtk::{WindowPosition, WindowType};

pub trait WindowTrait: ContainerTrait {
    fn set_default_size(&mut self, width: i32, height: i32);
    fn set_title(&mut self, title: &str);
    fn set_window_position(&mut self, window_position: WindowPosition);
    fn get_title(&self) -> Option<String>;
}

impl <T> WindowTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: MutCast<ffi::C_GtkWindow>  + MutCast<ffi::C_GtkContainer> + MutCast<ffi::C_GtkWidget>  + MutCast<::glib::ffi::C_GObject> {

    fn set_default_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.as_mut_ptr().cast(), width, height)
        }
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let c_str = CString::from_slice(title.as_bytes());
            ffi::gtk_window_set_title(self.as_mut_ptr().cast(), c_str.as_ptr());
        }
    }

    fn set_window_position(&mut self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.as_mut_ptr().cast(), window_position);
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

struct_skel!(Window, ffi::C_GtkWindow);

impl Window {
    pub fn new(window_type: WindowType) -> Window {
        unsafe {
            FromPtr::from_ptr(
                ffi::gtk_window_new(window_type).force_downcast())
        }
    }
}
