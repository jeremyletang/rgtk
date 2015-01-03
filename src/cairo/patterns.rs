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

use libc::{c_double, c_int, c_uint};
use std::mem::transmute;
use cairo::enums::{
    Extend,
    Filter,
    Status,
    PatternType
};
use cairo::ffi;
use cairo::ffi::{
    cairo_pattern_t
};
use cairo::{
    Path
};

//Quite some changes from the C api but all suggested by the cairo devs.
//See http://cairographics.org/manual/bindings-patterns.html for more info


//TODO Does anyone know a way to do this without dynamic dispatch -- @mthq
pub fn wrap_pattern<'a>(ptr: *mut cairo_pattern_t) -> Box<Pattern + 'a> {
    let pattern_type = unsafe{ ffi::cairo_pattern_get_type(ptr) };

    match pattern_type {
        PatternType::PatternTypeSolid            => box SolidPattern::wrap(ptr)   as Box<Pattern>,
        PatternType::PatternTypeSurface          => box SurfacePattern::wrap(ptr) as Box<Pattern>,
        PatternType::PatternTypeLinearGradient   => box LinearGradient::wrap(ptr) as Box<Pattern>,
        PatternType::PatternTypeRadialGradient   => box RadialGradient::wrap(ptr) as Box<Pattern>,
        PatternType::PatternTypeMesh             => box Mesh::wrap(ptr)           as Box<Pattern>,
        PatternType::PatternTypeRasterSource     => panic!("Not implemented")
    }
}

pub trait Pattern {
    fn get_ptr(&self) -> *mut cairo_pattern_t;

    fn ensure_status(&self) {
        self.status().ensure_valid();
    }

    fn status(&self) -> Status {
        unsafe {
            ffi::cairo_pattern_status(self.get_ptr())
        }
    }

    fn get_reference_count(&self) -> int {
        unsafe {
            ffi::cairo_pattern_get_reference_count(self.get_ptr()) as int
        }
    }

    fn set_extend(&self, extend: Extend) {
        unsafe {
            ffi::cairo_pattern_set_extend(self.get_ptr(), extend)
        }
    }

    fn get_extend(&self) -> Extend {
        unsafe {
            ffi::cairo_pattern_get_extend(self.get_ptr())
        }
    }

    fn set_filter(&self, filter: Filter) {
        unsafe {
            ffi::cairo_pattern_set_filter(self.get_ptr(), filter)
        }
    }

    fn get_filter(&self) -> Filter {
        unsafe {
            ffi::cairo_pattern_get_filter(self.get_ptr())
        }
    }

    //fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);

    //fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
}

macro_rules! pattern_type(
    //Signals without arguments
    ($pattern_type:ident) => (

        pub struct $pattern_type {
            pointer: *mut cairo_pattern_t
        }

        impl $pattern_type {
            pub fn wrap(pointer: *mut cairo_pattern_t) -> $pattern_type {
                $pattern_type{
                    pointer: pointer
                }
            }

            pub fn reference(&self) -> $pattern_type {
                $pattern_type{
                    pointer: unsafe {
                        ffi::cairo_pattern_reference(self.pointer)
                    }
                }
            }
        }

        impl Pattern for $pattern_type {
            fn get_ptr(&self) -> *mut cairo_pattern_t{
                self.pointer
            }
        }

        impl Drop for $pattern_type {
            fn drop(&mut self){
                unsafe {
                    ffi::cairo_pattern_destroy(self.pointer)
                }
            }
        }
    );
);

pattern_type!(SolidPattern);

impl SolidPattern {
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidPattern {
        SolidPattern::wrap(unsafe {
            ffi::cairo_pattern_create_rgb(red, green, blue)
        })
    }

    pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> SolidPattern {
        SolidPattern::wrap(unsafe {
            ffi::cairo_pattern_create_rgba(red, green, blue, alpha)
        })
    }

    pub fn get_rgba(&self) -> (f64,f64,f64,f64) {
        unsafe {
            let red  : *mut c_double = transmute(box 0.0f64);
            let green: *mut c_double = transmute(box 0.0f64);
            let blue : *mut c_double = transmute(box 0.0f64);
            let alpha: *mut c_double = transmute(box 0.0f64);

            ffi::cairo_pattern_get_rgba(self.pointer, red, green, blue, alpha).ensure_valid();

            (*red, *green, *blue, *alpha)
        }
    }
}



pub trait Gradient : Pattern {
    fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgb(self.get_ptr(), offset, red, green, blue)
        }
    }

    fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_pattern_add_color_stop_rgba(self.get_ptr(), offset, red, green, blue, alpha)
        }
    }

    fn get_color_stop_count(&self) -> int {
        unsafe {
            let count : *mut c_int = transmute(box 0i32);
            let result = ffi::cairo_pattern_get_color_stop_count(self.get_ptr(), count);

            result.ensure_valid(); // Not sure if these are needed

            count as int
        }
    }

    fn get_color_stop_rgba(&self, index: int) -> (f64,f64,f64,f64,f64) {
        unsafe {
            let offset: *mut c_double = transmute(box 0.0f64);
            let red   : *mut c_double = transmute(box 0.0f64);
            let green : *mut c_double = transmute(box 0.0f64);
            let blue  : *mut c_double = transmute(box 0.0f64);
            let alpha : *mut c_double = transmute(box 0.0f64);

            ffi::cairo_pattern_get_color_stop_rgba(self.get_ptr(), index as c_int, offset, red, green, blue, alpha).ensure_valid();

            (*offset, *red, *green, *blue, *alpha)
        }
    }
}

pattern_type!(LinearGradient);

impl LinearGradient {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> LinearGradient {
        LinearGradient::wrap(unsafe {
            ffi::cairo_pattern_create_linear(x0, y0, x1, y1)
        })
    }

    pub fn get_linear_points(&self) -> (f64,f64,f64,f64) {
        unsafe {
            let x0 : *mut c_double = transmute(box 0.0f64);
            let y0 : *mut c_double = transmute(box 0.0f64);
            let x1 : *mut c_double = transmute(box 0.0f64);
            let y1 : *mut c_double = transmute(box 0.0f64);

            ffi::cairo_pattern_get_linear_points(self.pointer, x0, y0, x1, y1).ensure_valid();


            (*x0, *y0, *x1, *y1)
        }
    }
}

impl Gradient for LinearGradient{}


pattern_type!(RadialGradient);

impl RadialGradient {
    pub fn new(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> RadialGradient {
        RadialGradient::wrap(unsafe{
            ffi::cairo_pattern_create_radial(x0, y0, r0, x1, y1, r1)
        })
    }

    pub fn get_radial_circles(&self) -> (f64,f64,f64,f64) {
        unsafe{
            let x0 : *mut c_double = transmute(box 0.0f64);
            let y0 : *mut c_double = transmute(box 0.0f64);
            let r0 : *mut c_double = transmute(box 0.0f64);
            let x1 : *mut c_double = transmute(box 0.0f64);
            let y1 : *mut c_double = transmute(box 0.0f64);
            let r1 : *mut c_double = transmute(box 0.0f64);

            ffi::cairo_pattern_get_radial_circles(self.pointer, x0, y0, r0, x1, y1, r1).ensure_valid();

            (*x0, *y0, *x1, *y1)
        }
    }
}


pattern_type!(SurfacePattern);

impl SurfacePattern {
    //pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;

    //pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: **mut cairo_surface_t) -> Status;
}

#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub enum MeshCorner {
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3
}

pattern_type!(Mesh);

impl Mesh {
    pub fn new() -> Mesh {
        Mesh::wrap(unsafe {
            ffi::cairo_pattern_create_mesh()
        })
    }

    pub fn begin_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_begin_patch(self.pointer)
        }
        self.ensure_status();
    }

    pub fn end_patch(&self) {
        unsafe {
            ffi::cairo_mesh_pattern_end_patch(self.pointer)
        }
        self.ensure_status();
    }

    pub fn move_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_move_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_line_to(self.pointer, x, y)
        }
        self.ensure_status();
    }

    pub fn curve_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_curve_to(self.pointer, x1, y1, x2, y2, x3, y3)
        }
        self.ensure_status();
    }

    pub fn set_control_point(&self, corner: MeshCorner, x: f64, y: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_control_point(self.pointer, corner as c_uint, x, y)
        }
        self.ensure_status();
    }

    pub fn get_control_point(&self, patch_num: uint, corner: MeshCorner) -> (f64, f64) {
        let mut x: c_double = 0.0;
        let mut y: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_control_point(self.pointer, patch_num as c_uint, corner as c_uint, &mut x, &mut y)
        };
        status.ensure_valid();

        (x, y)
    }

    pub fn set_corner_color_rgb(&self, corner: MeshCorner, red: f64, green: f64, blue: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgb(self.pointer, corner as c_uint, red, green, blue)
        }
        self.ensure_status();
    }

    pub fn set_corner_color_rgba(&self, corner: MeshCorner, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe {
            ffi::cairo_mesh_pattern_set_corner_color_rgba(self.pointer, corner as c_uint, red, green, blue, alpha)
        }
        self.ensure_status();
    }

    pub fn get_corner_color_rgba(&self, patch_num: uint, corner: MeshCorner) -> (f64, f64, f64, f64) {
        let mut red: c_double = 0.0;
        let mut green: c_double = 0.0;
        let mut blue: c_double = 0.0;
        let mut alpha: c_double = 0.0;

        let status = unsafe {
            ffi::cairo_mesh_pattern_get_corner_color_rgba(self.pointer, patch_num as c_uint, corner as c_uint, &mut red, &mut green, &mut blue, &mut alpha)
        };
        status.ensure_valid();

        (red, green, blue, alpha)
    }

    pub fn get_patch_count(&self) -> uint {
        let mut count: c_uint = 0;
        unsafe {
            ffi::cairo_mesh_pattern_get_patch_count(self.pointer, &mut count).ensure_valid();
        }
        count as uint
    }

    pub fn get_path(&self, patch_num: uint) -> Path {
        let path: Path = Path::wrap(unsafe {
            ffi::cairo_mesh_pattern_get_path(self.pointer, patch_num as c_uint)
        });
        path.ensure_status();
        path
    }
}