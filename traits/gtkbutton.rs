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

use std::str;
use std::libc::c_float;

use traits::{GtkWidget, GtkContainer};
use gtk::enums::{GtkReliefStyle, GtkPositionType};
use utils::cast::GTK_BUTTON;
use ffi;

pub trait GtkButton: GtkWidget + GtkContainer {
    fn pressed(&self) -> () {
        unsafe {
            ffi::gtk_button_pressed(GTK_BUTTON(self.get_widget()));
        }
    }

    fn released(&self) -> () {
        unsafe {
            ffi::gtk_button_released(GTK_BUTTON(self.get_widget()));
        }
    }

    fn clicked(&self) -> () {
        unsafe {
            ffi::gtk_button_clicked(GTK_BUTTON(self.get_widget()));
        }
    }

    fn enter(&self) -> () {
        unsafe {
            ffi::gtk_button_enter(GTK_BUTTON(self.get_widget()));
        }
    }

    fn leave(&self) -> () {
        unsafe {
            ffi::gtk_button_leave(GTK_BUTTON(self.get_widget()));
        }
    }

    fn set_relief(&mut self, new_style: GtkReliefStyle) -> () {
        unsafe {
            ffi::gtk_button_set_relief(GTK_BUTTON(self.get_widget()), new_style);
        }
    }

    fn get_relief(&self) -> GtkReliefStyle {
        unsafe {
            ffi::gtk_button_get_relief(GTK_BUTTON(self.get_widget()))
        }
    }

    fn get_label(&self) -> Option<~str> {
        let c_str = unsafe { ffi::gtk_button_get_label(GTK_BUTTON(self.get_widget())) };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe { str::raw::from_c_str(c_str) })
        }
    }

    fn set_label(&mut self, label: &str) -> () {
        unsafe {
            label.with_c_str(|c_str| {
                ffi::gtk_button_set_label(GTK_BUTTON(self.get_widget()), c_str)
            });
        }
    }

    fn get_use_stock(&self) -> bool {
        match unsafe { ffi::gtk_button_get_use_stock(GTK_BUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_stock(&mut self, use_stock: bool) -> () {
        match use_stock {
            true    => unsafe { ffi::gtk_button_set_use_stock(GTK_BUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_button_set_use_stock(GTK_BUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_use_underline(&self) -> bool {
        match unsafe { ffi::gtk_button_get_use_underline(GTK_BUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_use_underline(&mut self, use_underline: bool) -> () {
        match use_underline {
            true    => unsafe { ffi::gtk_button_set_use_underline(GTK_BUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_button_set_use_underline(GTK_BUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn set_focus_on_click(&mut self, focus_on_click: bool) -> () {
        match focus_on_click {
            true    => unsafe { ffi::gtk_button_set_focus_on_click(GTK_BUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_button_set_focus_on_click(GTK_BUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_focus_on_click(&self) -> bool {
        match unsafe { ffi::gtk_button_get_focus_on_click(GTK_BUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }

    fn set_alignment(&mut self, x_align: f32, y_align: f32) -> () {
        unsafe {
            ffi::gtk_button_set_alignment(GTK_BUTTON(self.get_widget()), x_align as c_float, y_align as c_float)
        }
    }

    fn get_alignment(&self) -> (f32, f32) {
        let x_align = 0.1;
        let y_align = 0.1;
        unsafe {
            ffi::gtk_button_get_alignment(GTK_BUTTON(self.get_widget()), &x_align, &y_align);
        }
        (x_align as f32, y_align as f32)
    }

    fn set_image<T: GtkWidget>(&mut self, image: &T) -> () {
        unsafe {
            ffi::gtk_button_set_image(GTK_BUTTON(self.get_widget()), image.get_widget());
        }
    }

    fn set_image_position(&mut self, position: GtkPositionType) -> () {
        unsafe {
            ffi::gtk_button_set_image_position(GTK_BUTTON(self.get_widget()), position);
        }
    }

    fn get_image_position(&self) -> GtkPositionType {
        unsafe {
            ffi::gtk_button_get_image_position(GTK_BUTTON(self.get_widget()))
        }
    }

    fn set_always_show_image(&mut self, always_show: bool) -> () {
        match always_show {
            true    => unsafe { ffi::gtk_button_set_always_show_image(GTK_BUTTON(self.get_widget()), ffi::Gtrue) },
            false   => unsafe { ffi::gtk_button_set_always_show_image(GTK_BUTTON(self.get_widget()), ffi::Gfalse) }
        }
    }

    fn get_always_show_image(&self) -> bool {
        match unsafe { ffi::gtk_button_get_always_show_image(GTK_BUTTON(self.get_widget())) } {
            ffi::Gfalse     => false,
            _               => true
        }
    }
}
