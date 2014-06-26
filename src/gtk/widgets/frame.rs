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

//! A bin with a decorative frame and optional label


use std::ptr;

use gtk::ffi;
use gtk::traits;

/// Frame — A bin with a decorative frame and optional label
struct_Widget!(Frame)

impl Frame {
    pub fn new(label: Option<&str>) -> Frame {
        let tmp_pointer = match label {
            Some(l) => unsafe { l.with_c_str(|c_str| { ffi::gtk_frame_new(c_str) }) },
            None    => unsafe { ffi::gtk_frame_new(ptr::null()) }
        };
        check_pointer!(tmp_pointer, Frame)
    }
}

impl_drop!(Frame)
impl_TraitWidget!(Frame)

impl traits::Frame for Frame {}
impl traits::Container for Frame {}
impl traits::Bin for Frame {}
