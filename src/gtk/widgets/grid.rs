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

//! Pack widgets in a rows and columns

// FIXME: Missings methods

use libc::{c_int, c_uint};

use gtk::{PositionType};
use gtk::cast::GTK_GRID;
use gtk::{self, ffi};

/// Grid — Pack widgets in a rows and columns
struct_Widget!(Grid);

impl Grid {
    pub fn new() -> Option<Grid> {
        let tmp_pointer = unsafe { ffi::gtk_grid_new() };
        check_pointer!(tmp_pointer, Grid)
    }

    pub fn attach<T: gtk::WidgetTrait>(&mut self,
                                child: &T,
                                left: i32,
                                top: i32,
                                width: i32,
                                height: i32) -> () {
        unsafe {
            ffi::gtk_grid_attach(GTK_GRID(self.pointer),
                                 child.get_widget(),
                                 left as c_int,
                                 top as c_int,
                                 width as c_int,
                                 height as c_int);
        }
    }

    pub fn attach_next_to<T: gtk::WidgetTrait>(&mut self,
                                        child: &T,
                                        sibling: &T,
                                        side: PositionType,
                                        width: i32,
                                        height: i32) -> () {
        unsafe {
            ffi::gtk_grid_attach_next_to(GTK_GRID(self.pointer),
                                         child.get_widget(),
                                         sibling.get_widget(),
                                         side,
                                         width as c_int,
                                         height as c_int);
        }
    }

    pub fn insert_row(&mut self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_insert_row(GTK_GRID(self.pointer), position as c_int);
        }
    }

     pub fn insert_column(&mut self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_insert_column(GTK_GRID(self.pointer), position as c_int);
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
     pub fn remove_row(&mut self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_remove_row(GTK_GRID(self.pointer), position as c_int);
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
     pub fn remove_column(&mut self, position: i32) -> () {
        unsafe {
            ffi::gtk_grid_remove_column(GTK_GRID(self.pointer), position as c_int);
        }
    }

    pub fn insert_next_to<T: gtk::WidgetTrait>(&mut self, sibling: &T, side: PositionType) -> () {
        unsafe {
            ffi::gtk_grid_insert_next_to(GTK_GRID(self.pointer), sibling.get_widget(), side);
        }
    }

    pub fn set_row_homogeneous(&mut self, homogeneous: bool) -> () {
        match homogeneous {
            true    => unsafe { ffi::gtk_grid_set_row_homogeneous(GTK_GRID(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_grid_set_row_homogeneous(GTK_GRID(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_row_homogeneous(&self) -> bool {
        match unsafe { ffi::gtk_grid_get_row_homogeneous(GTK_GRID(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_row_spacing(&mut self, spacing: u32) -> () {
        unsafe {
            ffi::gtk_grid_set_row_spacing(GTK_GRID(self.pointer), spacing as c_uint);
        }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_row_spacing(GTK_GRID(self.pointer)) as u32
        }
    }

    pub fn set_column_homogeneous(&mut self, homogeneous: bool) -> () {
        match homogeneous {
            true    => unsafe { ffi::gtk_grid_set_column_homogeneous(GTK_GRID(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_grid_set_column_homogeneous(GTK_GRID(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_column_homogeneous(&self) -> bool {
        match unsafe { ffi::gtk_grid_get_column_homogeneous(GTK_GRID(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    pub fn set_column_spacing(&mut self, spacing: u32) -> () {
        unsafe {
            ffi::gtk_grid_set_column_spacing(GTK_GRID(self.pointer), spacing as c_uint);
        }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_column_spacing(GTK_GRID(self.pointer)) as u32
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn get_baseline_row(&self) -> i32 {
        unsafe {
            ffi::gtk_grid_get_baseline_row(GTK_GRID(self.pointer)) as i32
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    pub fn set_baseline_row(&mut self, row: i32) -> () {
        unsafe {
            ffi::gtk_grid_set_baseline_row(GTK_GRID(self.pointer), row as c_int);
        }
    }
}

impl_drop!(Grid);
impl_TraitWidget!(Grid);

impl gtk::ContainerTrait for Grid {}
impl gtk::OrientableTrait for Grid {}

impl_widget_events!(Grid);
