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

//! A ToolItem containing a toggle button

use gtk::ffi;
use gtk::traits;

/// ToggleToolButton — A ToolItem containing a toggle button
struct_Widget!(ToggleToolButton)

impl ToggleToolButton {
    pub fn new() -> ToggleToolButton {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new() };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> ToggleToolButton {
        let tmp_pointer = stock_id.with_c_str(|c_str| {
            unsafe { ffi::gtk_toggle_tool_button_new_from_stock(c_str) }
        });
        check_pointer!(tmp_pointer, ToggleToolButton)
    }
}

impl_drop!(ToggleToolButton)
impl_TraitWidget!(ToggleToolButton)

impl traits::Container for ToggleToolButton {}
impl traits::Bin for ToggleToolButton {}
impl traits::ToolItem for ToggleToolButton {}
impl traits::ToolButton for ToggleToolButton {}
impl traits::ToggleToolButton for ToggleToolButton {}