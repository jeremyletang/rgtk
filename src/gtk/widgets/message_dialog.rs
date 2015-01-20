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
use gtk::cast::{GTK_MESSAGE_DIALOG, GTK_WINDOW};
use std::ffi::CString;

struct_Widget!(MessageDialog);

impl MessageDialog {
    pub fn new(parent: Option<gtk::Window>, flags: gtk::DialogFlags, _type: gtk::MessageType, buttons: gtk::ButtonsType) -> Option<MessageDialog> {
        let tmp_pointer = unsafe { ffi::gtk_message_dialog_new(match parent {
                Some(ref p) => GTK_WINDOW(p.get_widget()),
                None => ::std::ptr::null_mut()
            }, flags, _type, buttons, ::std::ptr::null())
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }

    pub fn new_with_markup(parent: Option<gtk::Window>, flags: gtk::DialogFlags, _type: gtk::MessageType, buttons: gtk::ButtonsType, markup: &str) -> Option<MessageDialog> {
        //gtk_message_dialog_new_with_markup
        match MessageDialog::new(parent, flags, _type, buttons) {
            Some(m) => {
                m.set_markup(markup);
                Some(m)
            }
            None => None
        }
    }

    pub fn set_markup(&self, markup: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(markup.as_bytes());
            ffi::gtk_message_dialog_set_markup(GTK_MESSAGE_DIALOG(self.get_widget()), c_str)
        }
    }

    pub fn get_message_area<T: gtk::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_message_dialog_get_message_area(GTK_MESSAGE_DIALOG(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(MessageDialog);
impl_TraitWidget!(MessageDialog);

impl gtk::ContainerTrait for MessageDialog {}
impl gtk::BinTrait for MessageDialog {}
impl gtk::WindowTrait for MessageDialog {}
impl gtk::DialogTrait for MessageDialog {}

impl_widget_events!(MessageDialog);
