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

//! Create bars of buttons and other widgets

use std::libc::{c_void, c_int};
use std::{ptr, cast};

use traits::{GtkWidget, GtkToolShell, GtkOrientable, GtkContainer, GtkToolItem, Signal};
use ffi;
use utils::cast::{GTK_TOOLBAR, GTK_TOOLITEM};
use gtk;
use gtk::enums::{GtkIconSize, GtkReliefStyle, GtkToolbarStyle};

/** 
* Toolbar — Create bars of buttons and other widgets
*
* # Availables signals :
* * `focus-home-or-end` : Action
* * `orientation-changed` : Run First
* * `popup-context-menu` : Run Last
* * `style-changed` : Run First
*/
pub struct Toolbar {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl Toolbar {
    pub fn new() -> Option<Toolbar> {
        let tmp_pointer = unsafe { ffi::gtk_toolbar_new() };
        check_pointer!(tmp_pointer, Toolbar)
    }

    pub fn insert<T: GtkToolItem>(&mut self, 
                                  item: &T,
                                  pos: i32) -> () {
        unsafe {
            ffi::gtk_toolbar_insert(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.get_widget()), pos as c_int)
        }
    }

    pub fn item_index<T: GtkToolItem>(&mut self, item: &T) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.get_widget())) as i32
        }
    }

    pub fn n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(GTK_TOOLBAR(self.pointer)) as i32
        }
    }

    pub fn nth_item(&self, n: i32) -> Option<gtk::ToolItem> {
        unsafe {
            let tmp_pointer = ffi::gtk_toolbar_get_nth_item(GTK_TOOLBAR(self.pointer), n as c_int) as *ffi::C_GtkWidget;
            if tmp_pointer.is_null() {
                None
            } else {
                Some(GtkWidget::wrap_widget(tmp_pointer))
            }
        }
    }

    pub fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(GTK_TOOLBAR(self.pointer), x as c_int, y as c_int) as i32
        }
    }

    pub fn set_drop_highlight_item<T: GtkToolItem>(&mut self, item: &T, index: i32) -> () {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(GTK_TOOLBAR(self.pointer), GTK_TOOLITEM(item.get_widget()), index as c_int);
        }
    }

    pub fn set_show_arrow(&mut self, show_arrow: bool) -> () {
        match show_arrow {
            true    => unsafe { ffi::gtk_toolbar_set_show_arrow(GTK_TOOLBAR(self.pointer), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_toolbar_set_show_arrow(GTK_TOOLBAR(self.pointer), ffi::Gfalse) }
        }
    }

    pub fn unset_icon_size(&mut self) -> () {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_show_arrow(&self) -> bool {
        match unsafe { ffi::gtk_toolbar_get_show_arrow(GTK_TOOLBAR(self.pointer)) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    pub fn get_style(&self) -> GtkToolbarStyle {
        unsafe {
            ffi::gtk_toolbar_get_style(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_icon_size(&self) -> GtkIconSize {
        unsafe {
            ffi::gtk_toolbar_get_icon_size(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn get_relief_style(&self) -> GtkReliefStyle {
        unsafe {
            ffi::gtk_toolbar_get_relief_style(GTK_TOOLBAR(self.pointer))
        }
    }

    pub fn set_style(&mut self, style: GtkToolbarStyle) -> () {
        unsafe {
            ffi::gtk_toolbar_set_style(GTK_TOOLBAR(self.pointer), style);
        }
    }

    pub fn set_icon_size(&mut self, icon_size: GtkIconSize) -> () {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(GTK_TOOLBAR(self.pointer), icon_size);
        }
    }

    pub fn unset_style(&mut self) -> () {
        unsafe {
            ffi::gtk_toolbar_unset_style(GTK_TOOLBAR(self.pointer));
        }
    }
}

impl_GtkWidget!(Toolbar)

impl GtkContainer for Toolbar {}
impl GtkToolShell for Toolbar {}
impl GtkOrientable for Toolbar {}
