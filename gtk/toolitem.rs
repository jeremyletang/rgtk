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

//! The base class of widgets that can be added to GtkToolShe

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkContainer, GtkWidget, GtkBin, GtkToolItem, Signal};
use ffi;

/// ToolItem — The base class of widgets that can be added to GtkToolShe
pub struct ToolItem {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl ToolItem {
    pub fn new() -> Option<ToolItem> {
        let tmp_pointer = unsafe { ffi::gtk_tool_item_new() };
        check_pointer!(tmp_pointer, ToolItem)
    }
}

impl_GtkWidget!(ToolItem)

impl GtkContainer for ToolItem {}
impl GtkBin for ToolItem {}
impl GtkToolItem for ToolItem {}
