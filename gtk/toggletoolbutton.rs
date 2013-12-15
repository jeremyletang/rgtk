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

//! A GtkToolItem containing a toggle button

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkContainer, GtkWidget, GtkBin, GtkToolItem, GtkToolButton, GtkToggleToolButton, Signal};
use ffi;

/// ToggleToolButton — A GtkToolItem containing a toggle button
pub struct ToggleToolButton {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl ToggleToolButton {
    pub fn new() -> Option<ToggleToolButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new() };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<ToggleToolButton> {
        let tmp_pointer = stock_id.with_c_str(|c_str| {
            unsafe { ffi::gtk_toggle_tool_button_new_from_stock(c_str) }
        });
        check_pointer!(tmp_pointer, ToggleToolButton)
    }
}

impl_GtkWidget!(ToggleToolButton)

impl GtkContainer for ToggleToolButton {}
impl GtkBin for ToggleToolButton {}
impl GtkToolItem for ToggleToolButton {}
impl GtkToolButton for ToggleToolButton {}
impl GtkToggleToolButton for ToggleToolButton {}
