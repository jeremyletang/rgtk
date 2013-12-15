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

//! A frame that constrains its child to a particular aspect ratio

use std::libc::{c_float, c_void};
use std::{ptr, cast};

use traits::{GtkWidget, GtkFrame, GtkContainer, Signal};
use utils::cast::GTK_ASPECTFRAME;
use ffi;

/// AspectFrame — A frame that constrains its child to a particular aspect ratio
pub struct AspectFrame {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl AspectFrame {
    pub fn new(label: Option<&str>,
               x_align: f32,
               y_align: f32,
               ratio: f32,
               obey_child: bool)
               -> Option<AspectFrame> {
        let c_obey_child = if obey_child { ffi::Gtrue } else { ffi::Gfalse };
        let tmp_pointer = match label {
            Some(l) => unsafe { l.with_c_str(|c_str| { ffi::gtk_aspect_frame_new(c_str, x_align as c_float, y_align as c_float, ratio as c_float, c_obey_child) }) },
            None    => unsafe { ffi::gtk_aspect_frame_new(ptr::null(), x_align as c_float, y_align as c_float, ratio as c_float, c_obey_child) }
        };
        check_pointer!(tmp_pointer, AspectFrame)
    }

    pub fn set(&mut self,
               x_align: f32,
               y_align: f32,
               ratio: f32,
               obey_child: bool) -> () {
        let c_obey_child = if obey_child { ffi::Gtrue } else { ffi::Gfalse };
        unsafe { 
            ffi::gtk_aspect_frame_set(GTK_ASPECTFRAME(self.get_widget()), x_align as c_float, y_align as c_float, ratio as c_float, c_obey_child);
        }
    }
}

impl_GtkWidget!(AspectFrame)

impl GtkFrame for AspectFrame {}
impl GtkContainer for AspectFrame {}
