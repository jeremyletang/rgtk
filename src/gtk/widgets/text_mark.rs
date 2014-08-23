// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A position in the buffer preserved across buffer modifications

use gtk::ffi;

pub struct TextMark {
    ptr: *mut ffi::C_GtkTextMark
}

impl TextMark {
    pub fn new(name: &str, left_gravity: bool) -> TextMark {
        let ptr = name.with_c_str(|c_str| {
            unsafe {
                ffi::gtk_text_mark_new(c_str, ffi::to_gboolean(left_gravity))
            }
        });
        TextMark {
            ptr: ptr
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *mut ffi::C_GtkTextMark {
        self.ptr
    }

    #[doc(hidden)]
    pub fn wrap(ptr: *mut ffi::C_GtkTextMark) -> TextMark{
        TextMark {
            ptr: ptr
        }
    }

    pub fn set_visible(&mut self, setting: bool) {
        unsafe {
            ffi:: gtk_text_mark_set_visible(self.ptr, ffi::to_gboolean(setting))
        }
    }

    pub fn is_visible(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_text_mark_get_visible(self.ptr))
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let name = ffi::gtk_text_mark_get_name(self.ptr);
            ::std::string::raw::from_buf(name as *const u8)
        }
    }

    pub fn is_deleted(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_text_mark_get_deleted(self.ptr))
        }
    }

    pub fn left_gravity(&self) -> bool {
        unsafe {
            ffi::to_bool(ffi::gtk_text_mark_get_left_gravity(self.ptr))
        }
    }
}
