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

use gtk::{self, ffi};
use gtk::ffi::FFIWidget;
use gtk::cast::{GTK_PAPER_SIZE};
use glib;
use std::ffi::CString;

// FIXME: PaperSize is not a widget nor a GObject -> GBoxed
struct_Widget!(PaperSize);

impl PaperSize {
    pub fn new(name: &str) -> Option<PaperSize> {
        let c_str = CString::from_slice(name.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new(c_str.as_ptr())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn new_from_ppd(ppd_name: &str, ppd_display_name: &str, width: f64, height: f64) -> Option<PaperSize> {
        let c_str = CString::from_slice(ppd_name.as_bytes());
        let c_str2 = CString::from_slice(ppd_display_name.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new_from_ppd(c_str.as_ptr(), c_str2.as_ptr(), width, height)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn new_custom(name: &str, display_name: &str, width: f64, height: f64, unit: gtk::Unit) -> Option<PaperSize> {
        let c_str = CString::from_slice(name.as_bytes());
        let c_str2 = CString::from_slice(display_name.as_bytes());
        let tmp_pointer = unsafe {
            ffi::gtk_paper_size_new_custom(c_str.as_ptr(), c_str2.as_ptr(), width, height, unit)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn copy(&self) -> Option<PaperSize> {
        let tmp_pointer = unsafe { ffi::gtk_paper_size_copy(GTK_PAPER_SIZE(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    pub fn is_equal(&self, other: &PaperSize) -> bool {
        match unsafe { ffi::gtk_paper_size_is_equal(GTK_PAPER_SIZE(self.get_widget()), GTK_PAPER_SIZE(other.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn get_paper_sizes(include_custom: bool) -> glib::List<Box<PaperSize>> {
        let tmp = unsafe { ffi::gtk_paper_size_get_paper_sizes(match include_custom {
            true => ffi::GTRUE,
            false => ffi::GFALSE });
        };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<PaperSize>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(ffi::FFIWidget::wrap)(*it));
            }
            tmp_vec
        }
    }

    pub fn get_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_paper_size_get_name(GTK_PAPER_SIZE(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp as *const u8) })
        }
    }

    pub fn get_display_name_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_paper_size_get_display_name(GTK_PAPER_SIZE(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp as *const u8) })
        }
    }

    pub fn get_ppd_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_paper_size_get_ppd_name(GTK_PAPER_SIZE(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp as *const u8) })
        }
    }

    pub fn get_width(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_width(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn get_height(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_height(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn is_custom(&self) -> bool {
        match unsafe { ffi::gtk_paper_size_is_custom(GTK_PAPER_SIZE(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_size(&self, width: f64, height: f64, unit: gtk::Unit) {
        unsafe { ffi::gtk_paper_size_set_size(GTK_PAPER_SIZE(self.get_widget()), width, height, unit) }
    }

    pub fn get_default_top_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_top_margin(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn get_default_bottom_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_bottom_margin(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn get_default_left_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn get_default_right_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(GTK_PAPER_SIZE(self.get_widget()), unit) }
    }

    pub fn get_default() -> Option<String> {
        let tmp = unsafe { ffi::gtk_paper_size_get_default() };

        if tmp.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp as *const u8) })
        }
    }
}

impl Drop for PaperSize {
    fn drop(&mut self) {
        unsafe {
            ffi::gtk_paper_size_free(GTK_PAPER_SIZE(self.get_widget()));
            ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
        }
    }
}

impl Clone for PaperSize {
    fn clone(&self) -> PaperSize {
        let pointer = unsafe {
            ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
        };

        PaperSize {
            pointer: pointer as *mut ffi::C_GtkWidget
        }
    }
}

impl_TraitWidget!(PaperSize);

impl_widget_events!(PaperSize);
