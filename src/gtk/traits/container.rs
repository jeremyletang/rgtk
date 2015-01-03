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

use libc::c_uint;

use gtk::cast::GTK_CONTAINER;
use gtk::ResizeMode;
use gtk::{self, ffi};

pub trait ContainerTrait: gtk::WidgetTrait {
    fn add<'r, T: gtk::WidgetTrait>(&'r mut self, widget: &'r T) {
        unsafe {
            ffi::gtk_container_add(GTK_CONTAINER(self.get_widget()), widget.get_widget());
        }
    }

    fn remove<'r, T: gtk::WidgetTrait>(&'r mut self, widget: &'r T) {
        unsafe {
            ffi::gtk_container_remove(GTK_CONTAINER(self.get_widget()), widget.get_widget());
        }
    }

    fn get_resize_mode(&self) -> ResizeMode {
        unsafe {
            ffi::gtk_container_get_resize_mode(GTK_CONTAINER(self.get_widget()))
        }
    }

    fn set_resize_mode(&mut self, resize_mode: ResizeMode) -> () {
        unsafe {
            ffi::gtk_container_set_resize_mode(GTK_CONTAINER(self.get_widget()), resize_mode);
        }
    }

    fn get_border_width(&self) -> u32 {
        unsafe {
            ffi::gtk_container_get_border_width(GTK_CONTAINER(self.get_widget())) as u32
        }
    }

    fn set_border_width(&self, border_width: u32) -> () {
        unsafe {
            ffi::gtk_container_set_border_width(GTK_CONTAINER(self.get_widget()), border_width as c_uint);
        }
    }
}
