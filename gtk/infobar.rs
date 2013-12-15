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

//! Report important messages to the user

// use std::{ptr, cast};
use std::libc::{c_void, c_int};

use gtk::enums::GtkMessageType;
use traits::{GtkContainer, GtkWidget, GtkBox, GtkOrientable};
use utils::cast::GTK_INFOBAR;
use gtk;
use ffi;

/// InfoBar — Report important messages to the user
pub struct InfoBar {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl InfoBar {
    pub fn new() -> Option<InfoBar> {
        let tmp_pointer = unsafe { ffi::gtk_info_bar_new() };
        check_pointer!(tmp_pointer, InfoBar)
    }

    pub fn add_action_widget<T: GtkWidget>(&mut self, child: &T, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(GTK_INFOBAR(self.pointer), child.get_widget(), response_id as c_int)
        }
    }

    pub fn add_button(&mut self, button_text: &str, response_id: i32) -> gtk::Button {
        let button = unsafe {
            button_text.with_c_str(|c_str| {
                ffi::gtk_info_bar_add_button(GTK_INFOBAR(self.pointer), c_str, response_id as c_int)
            })
        };
        GtkWidget::wrap_widget(button)
    }

    pub fn set_response_sensitive(&mut self, response_id: i32, setting: bool) -> () {
        match setting {
            true    => unsafe { ffi::gtk_info_bar_set_response_sensitive(GTK_INFOBAR(self.pointer), response_id as c_int, ffi::Gtrue) },
            false   => unsafe { ffi::gtk_info_bar_set_response_sensitive(GTK_INFOBAR(self.pointer), response_id as c_int, ffi::Gfalse) }
        }
    }

    pub fn set_default_response(&mut self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_set_default_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn response(&mut self, response_id: i32) -> () {
        unsafe {
            ffi::gtk_info_bar_response(GTK_INFOBAR(self.pointer), response_id as c_int)
        }
    }

    pub fn set_message_type(&mut self, message_type: GtkMessageType) -> () {
        unsafe {
            ffi::gtk_info_bar_set_message_type(GTK_INFOBAR(self.pointer), message_type);
        }
    }

    pub fn get_message_type(&mut self) -> GtkMessageType {
        unsafe {
            ffi::gtk_info_bar_get_message_type(GTK_INFOBAR(self.pointer))
        }
    }

    pub fn show_close_button(&mut self, show: bool) -> () {
         match show {
            true    => unsafe { ffi::gtk_info_bar_set_show_close_button(GTK_INFOBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_info_bar_set_show_close_button(GTK_INFOBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn get_show_close_button(&self) -> bool {
        match unsafe { ffi::gtk_info_bar_get_show_close_button(GTK_INFOBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }
}

impl_GtkWidget!(InfoBar)

impl GtkContainer for InfoBar {}
impl GtkBox for InfoBar {}
impl GtkOrientable for InfoBar {}
