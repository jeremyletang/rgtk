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

//! A button which pops up a volume control

use gtk::{self, ffi};

/// VolumeButton — A button which pops up a volume control
struct_Widget!(VolumeButton);

impl VolumeButton {
    pub fn new() -> Option<VolumeButton> {
        let tmp_pointer = unsafe { ffi::gtk_volume_button_new() };
        check_pointer!(tmp_pointer, VolumeButton)
    }
}

impl_drop!(VolumeButton);
impl_TraitWidget!(VolumeButton);

impl gtk::ContainerTrait for VolumeButton {}
impl gtk::ButtonTrait for VolumeButton {}
impl gtk::ScaleButtonTrait for VolumeButton {}
impl gtk::OrientableTrait for VolumeButton {}

impl_widget_events!(VolumeButton);
