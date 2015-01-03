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

use std::mem::transmute;
use std::iter::Iterator;
use std::c_vec::CVec;
use cairo::enums::PathDataType;
use cairo::ffi::{
    cairo_path_t,
    cairo_path_data_header
};
use cairo::ffi;

pub struct Path(*mut cairo_path_t);

impl Path {
    pub fn get_ptr(&self) -> *mut cairo_path_t {
        let Path(ptr) = *self;

        ptr
    }

    pub fn ensure_status(&self) {
        unsafe {
            let ptr: *mut cairo_path_t = self.get_ptr();
            (*ptr).status.ensure_valid()
        }
    }

    pub fn wrap(pointer: *mut cairo_path_t) -> Path {
        Path(pointer)
    }

    pub fn iter(&self) -> PathSegments {
        unsafe {
            let ptr: *mut cairo_path_t = self.get_ptr();
            let length = (*ptr).num_data as uint;
            let data_ptr = (*ptr).data;

            PathSegments {
                data: CVec::new(data_ptr, length),
                i: 0,
                num_data: length
            }
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe{
            ffi::cairo_path_destroy(self.get_ptr());
        }
    }
}

#[derive(Show, Copy)]
pub enum PathSegment {
    MoveTo((f64,f64)),
    LineTo((f64,f64)),
    CurveTo((f64, f64),(f64, f64),(f64, f64)),
    ClosePath
}

pub struct PathSegments<'a> {
    data: CVec<(f64, f64)>,
    i: uint,
    num_data: uint
}

impl<'a> Iterator<PathSegment> for PathSegments<'a> {
    fn next(&mut self) -> Option<PathSegment> {
        let i = self.i;

        if i >= self.num_data{
            return None;
        }

        let (data_type, length) = unsafe {
            let data_header: &cairo_path_data_header = transmute(self.data.get(i));
            (data_header.data_type, data_header.length)
        };

        self.i += length as uint;

        let ref data = self.data;

        Some(match data_type {
            PathDataType::PathMoveTo => PathSegment::MoveTo(*data.get(i+1).unwrap()),
            PathDataType::PathLineTo => PathSegment::LineTo(*data.get(i+1).unwrap()),
            PathDataType::PathCurveTo => PathSegment::CurveTo(*data.get(i+1).unwrap(), *data.get(i+2).unwrap(), *data.get(i+3).unwrap()),
            PathDataType::PathClosePath => PathSegment::ClosePath
        })
    }
}
