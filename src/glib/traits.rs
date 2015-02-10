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

use glib::ffi;
use gtk::signals::Signal;
use std::ffi::CString;

pub trait FFIGObject {
    fn get_gobject(&self) -> *mut ffi::C_GObject;
    fn wrap_object(object: *mut ffi::C_GObject) -> Self;
}

pub use glib::ffi::{GType, C_GObject};

pub trait Cast<T> {
    fn cast(&self) -> *mut T;
}

pub trait Downcast<T> {
    fn try_downcast(&self) -> Option<*mut T>;
    unsafe fn force_downcast(&self) -> *mut T;

    fn downcast(&self) -> *mut T {
        self.try_downcast().unwrap()
    }
}

pub trait AsPtr {
    type Inner;
    fn as_ptr(&self) -> *mut Self::Inner;
}

pub trait FromPtr {
    type Inner;
    fn from_ptr(ptr: *mut Self::Inner) -> Self;
}

pub trait ObjectTrait: AsPtr {
    fn ref_(&self);
    fn unref(&self);
}

pub trait GetGType {
    fn get_gtype() -> GType;
}

impl GetGType for C_GObject {
    fn get_gtype() -> GType {
        unsafe {
            ffi::g_object_get_type()
        }
    }
}

pub trait Connect_ {
    fn connect<'a, S: Signal<'a>>(&self, signal: Box<S>);
}

impl <T> ObjectTrait for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: Cast<C_GObject> {
    fn ref_(&self) {
        unsafe {
            ffi::g_object_ref_sink(self.as_ptr().cast());
        }
    }

    fn unref(&self) {
        unsafe {
            ffi::g_object_unref(self.as_ptr().cast());
        }
    }
}

impl <T> Cast<T> for *mut T {
    fn cast(&self) -> *mut T {
        *self as *mut T
    }
}

impl <T> Connect_ for T
where T: AsPtr,
      <T as AsPtr>::Inner: GetGType,
      *mut <T as AsPtr>::Inner: Cast<C_GObject> {
    fn connect<'a, S: Signal<'a>>(&self, signal: Box<S>) {
        use std::mem::transmute;

        let signal = signal as Box<Signal<'a>>;

        unsafe {
            let signal_name     = signal.get_signal_name().to_string();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(Box::new(signal));

            let c_str = CString::from_slice(signal_name.replace("_", "-").as_bytes());

            ffi::g_signal_connect_data(
                self.as_ptr().cast(),
                c_str.as_ptr(),
                Some(trampoline),
                user_data_ptr,
                0 as *const _,
                ffi::GConnectFlags::None
            );
        }
    }
}

// pub trait Connect<T>: FFIGObject {
//     fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
//         use std::mem::transmute;

//         unsafe {
//             let signal_name     = signal.get_signal_name().to_string();
//             let trampoline      = signal.get_trampoline();

//             let user_data_ptr   = transmute(Box::new(signal));

//             signal_name.replace("_", "-").with_c_str(|signal_name| {
//                 ffi::glue_signal_connect(
//                     self.get_gobject(),
//                     signal_name,
//                     Some(trampoline),
//                     user_data_ptr
//                 )
//             });
//         }
//     }
// }

pub trait Connect<'a, T: Signal<'a>>: FFIGObject {
    fn connect(&self, signal: Box<T>) -> () {
        use std::mem::transmute;

        let signal = signal as Box<Signal<'a>>;

        unsafe {
            let signal_name     = signal.get_signal_name().to_string();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(Box::new(signal));

            let c_str = CString::from_slice(signal_name.replace("_", "-").as_bytes());
            
            ffi::glue_signal_connect(
                self.get_gobject(),
                c_str.as_ptr(),
                Some(trampoline),
                user_data_ptr
            );
        }
    }
}
