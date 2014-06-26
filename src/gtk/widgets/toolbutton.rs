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

//! A ToolItem subclass that displays buttons

use std::ptr;

use gtk::ffi;
use gtk::traits;

/// ToolButton — A ToolItem subclass that displays buttons
struct_Widget!(ToolButton)

impl ToolButton {
    pub fn new<T: traits::Widget>(icon_widget: Option<&T>, label: Option<&str>) -> ToolButton {
        let tmp_pointer: *ffi::C_GtkWidget = unsafe {
            match label {
                Some(l) => {
                    l.with_c_str(|c_str| {
                        match icon_widget {
                            Some(i) => ffi::gtk_tool_button_new(i.get_widget(), c_str),
                            None    => ffi::gtk_tool_button_new(ptr::null(), c_str)
                        }
                    })
                },
                None    => {
                    match icon_widget {
                        Some(i) => ffi::gtk_tool_button_new(i.get_widget(), ptr::null()),
                        None    => ffi::gtk_tool_button_new(ptr::null(), ptr::null())
                    }
                }
            }
        };
        check_pointer!(tmp_pointer, ToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> ToolButton {
        let tmp_pointer = stock_id.with_c_str(|c_str| {
            unsafe { ffi::gtk_tool_button_new_from_stock(c_str) }
        });
        check_pointer!(tmp_pointer, ToolButton)
    }
}

impl_drop!(ToolButton)
impl_TraitWidget!(ToolButton)

impl traits::Container for ToolButton {}
impl traits::Bin for ToolButton {}
impl traits::ToolItem for ToolButton {}
impl traits::ToolButton for ToolButton {}