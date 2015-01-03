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

use gtk::cast::GTK_TOGGLEBUTTON;
use gtk::{self, ffi};

pub trait ToggleButtonTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::ButtonTrait {
    fn set_mode(&mut self, draw_indicate: bool) {
        match draw_indicate {
            true    => unsafe { ffi::gtk_toggle_button_set_mode(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_toggle_button_set_mode(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn get_mode(&self) -> bool {
        match unsafe { ffi::gtk_toggle_button_get_mode(GTK_TOGGLEBUTTON(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn toggled(&mut self) -> () {
        unsafe {
            ffi::gtk_toggle_button_toggled(GTK_TOGGLEBUTTON(self.get_widget()))
        }
    }

    fn set_active(&mut self, is_active: bool) {
        match is_active {
            true    => unsafe { ffi::gtk_toggle_button_set_active(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_toggle_button_set_active(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn get_active(&self) -> bool {
        match unsafe { ffi::gtk_toggle_button_get_active(GTK_TOGGLEBUTTON(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn set_inconsistent(&mut self, setting: bool) {
        match setting {
            true    => unsafe { ffi::gtk_toggle_button_set_inconsistent(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_toggle_button_set_inconsistent(GTK_TOGGLEBUTTON(self.get_widget()), ffi::GFALSE) }
        }
    }

    fn get_inconsistent(&self) -> bool {
        match unsafe { ffi::gtk_toggle_button_get_inconsistent(GTK_TOGGLEBUTTON(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }
}

