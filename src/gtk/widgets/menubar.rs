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

//! A widget that shows a menu when clicked on



use gtk::ffi;
use gtk::traits;
use gtk::cast::GTK_MENU_BAR;
use gtk::PackDirection;

/// MenuButton — A widget that shows a menu when clicked on
struct_Widget!(MenuBar)

impl MenuBar {
    pub fn new() -> Option<MenuBar> {
        let tmp_pointer = unsafe { ffi::gtk_menu_bar_new () };
        check_pointer!(tmp_pointer, MenuBar)
    }

    //pub fn new_from_model(&self, model:GMenuModel) -> Option<MenuBar> 

    pub fn set_pack_direction(&self, direction: PackDirection){
        unsafe {
            ffi::gtk_menu_bar_set_pack_direction(GTK_MENU_BAR(self.pointer), direction)
        }    
    }

    pub fn get_pack_direction(&self) -> PackDirection {
        unsafe {
            ffi::gtk_menu_bar_get_pack_direction(GTK_MENU_BAR(self.pointer))
        }      
    }

    pub fn set_child_pack_direction(&self, direction: PackDirection){
        unsafe {
            ffi::gtk_menu_bar_set_child_pack_direction(GTK_MENU_BAR(self.pointer), direction)
        }    
    }

    pub fn get_child_pack_direction(&self) -> PackDirection {
        unsafe {
            ffi::gtk_menu_bar_get_child_pack_direction(GTK_MENU_BAR(self.pointer))
        }    
    }
    
}


impl_drop!(MenuBar)
impl_TraitWidget!(MenuBar)

impl traits::Container for MenuBar {}
impl traits::MenuShell for MenuBar {}

