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

//! GtkFontChooserWidget — A widget for selecting fonts

use gtk::{self, ffi};
use gtk::ffi::FFIWidget;

struct_Widget!(FontChooserWidget);

impl FontChooserWidget {
    pub fn new() -> Option<FontChooserWidget> {
        let tmp_pointer = unsafe { ffi::gtk_color_chooser_widget_new() };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer))
        }
    }
}

impl_drop!(FontChooserWidget);
impl_TraitWidget!(FontChooserWidget);

impl gtk::ContainerTrait for FontChooserWidget {}
impl gtk::BoxTrait for FontChooserWidget {}

impl_widget_events!(FontChooserWidget);