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

//! A container which overlays widgets on top of each other

use gtk::cast::{GTK_OVERLAY};
use gtk::{self, ffi};

/// GtkOverlay — A container which overlays widgets on top of each other
struct_Widget!(Overlay);

impl Overlay {
    pub fn new() -> Option<Overlay> {
        let tmp_pointer = unsafe { ffi::gtk_overlay_new() };
        check_pointer!(tmp_pointer, Overlay)
    }

    pub fn add_overlay<T: gtk::WidgetTrait>(&mut self, widget: &T) {
        unsafe {
            ffi::gtk_overlay_add_overlay(GTK_OVERLAY(self.pointer), widget.get_widget())
        }
    }
}

impl_drop!(Overlay);
impl_TraitWidget!(Overlay);

impl gtk::ContainerTrait for Overlay {}
impl gtk::BinTrait for Overlay {}

impl_widget_events!(Overlay);
