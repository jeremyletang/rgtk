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

/*!
Bindings and wrappers for __GLib__
*/

#![feature(core)]
#![feature(unsafe_destructor)]
#![feature(std_misc)]

extern crate libc;
extern crate "glib-sys" as glib_ffi;

pub use glib_ffi as ffi;

use libc::c_char;

pub use self::list::{List, Elem, RevElem};
pub use self::slist::{SList, SElem};
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::permission::Permission;
pub use self::traits::{FFIGObject, Connect};

mod list;
mod slist;
pub mod glib_container;
mod error;
mod permission;
pub mod traits;
pub mod translate;

pub fn to_gboolean(b: bool) -> ffi::Gboolean {
    match b {
        true => ffi::GTRUE,
        false => ffi::GFALSE
    }
}

pub fn to_bool(b: ffi::Gboolean) -> bool {
    b != ffi::GFALSE
}

// An opaque structure used as the base of all interface types.
#[derive(Copy)]
pub struct TypeInterface;

// An opaque structure used as the base of all type instances.
#[derive(Copy)]
pub struct TypeInstance;

// An opaque structure used as the base of all classes.
#[derive(Copy)]
pub struct TypeClass;

//FIXME: Check if this is actually correct (maybe not since ParamFlags is deprecated)
#[derive(Copy)]
pub enum ParamFlags{
    Readable,
    Writable,
    ReadWrite,
    Construct,
    ConstructOnly,
    LaxValidation,
    StaticName,
    Private,
    StaticNick,
    StaticBlurb,
    Deprecated
}

#[derive(Copy)]
#[repr(C)]
pub struct ParamSpec {
    g_type_instance: TypeInstance,
    name: *mut c_char,
    flags: ParamFlags,
    value_type: ffi::GType,
    owner_type: ffi::GType
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, FromPrimitive, Copy)]
pub enum GType {
    /// An invalid GType used as error return value in some functions which return a GType.
    Invalid = 0 << 2,
    /// A fundamental type which is used as a replacement for the C void return type.
    None = 1 << 2,
    /// The fundamental type from which all interfaces are derived.
    Interface = 2 << 2,
    /// The fundamental type corresponding to gchar. The type designated by G_TYPE_CHAR is unconditionally
    /// an 8-bit signed integer. This may or may not be the same type a the C type "gchar".
    Char = 3 << 2,
    /// The fundamental type corresponding to guchar.
    UChar = 4 << 2,
    /// The fundamental type corresponding to gboolean.
    Boolean = 5 << 2,
    /// The fundamental type corresponding to gint.
    Int = 6 << 2,
    /// The fundamental type corresponding to guint.
    UInt = 7 << 2,
    /// The fundamental type corresponding to glong.
    Long = 8 << 2,
    /// The fundamental type corresponding to gulong.
    ULong = 9 << 2,
    /// The fundamental type corresponding to gint64.
    Int64 = 10 << 2,
    /// The fundamental type corresponding to guint64.
    UInt64 = 11 << 2,
    /// The fundamental type from which all enumeration types are derived.
    Enum = 12 << 2,
    /// The fundamental type from which all flags types are derived.
    Flags = 13 << 2,
    /// The fundamental type corresponding to gfloat.
    Float = 14 << 2,
    /// The fundamental type corresponding to gdouble.
    Double = 15 << 2,
    /// The fundamental type corresponding to nul-terminated C strings.
    String = 16 << 2,
    /// The fundamental type corresponding to gpointer.
    Pointer = 17 << 2,
    /// The fundamental type from which all boxed types are derived.
    Boxed = 18 << 2,
    /// The fundamental type from which all GParamSpec types are derived.
    Param = 19 << 2,
    /// The fundamental type for GObject.
    Object = 20 << 2,
    /// The fundamental type corresponding to GVariant.
    /// All floating GVariant instances passed through the GType system are consumed.
    /// Note that callbacks in closures, and signal handlers for signals of return type G_TYPE_VARIANT, must never return floating variants.
    /// Note: GLib 2.24 did include a boxed type with this name. It was replaced with this fundamental type in 2.26.
    Variant = 21 << 2,
/*
    /// First fundamental type number to create a new fundamental type id with G_TYPE_MAKE_FUNDAMENTAL() reserved for GLib.
    ReservedGLibFirst = 22,
    /// Last fundamental type number reserved for GLib.
    ReservedGLibLast = 31,
    /// First fundamental type number to create a new fundamental type id with G_TYPE_MAKE_FUNDAMENTAL() reserved for BSE.
    ReservedGLibBSEFirst = 32,
    /// Last fundamental type number reserved for BSE.
    ReservedGLibBSELast = 48,
    /// First available fundamental type number to create new fundamental type id with G_TYPE_MAKE_FUNDAMENTAL().
    ReservedUserFirst = 49
*/
}
