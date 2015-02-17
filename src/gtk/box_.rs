use libc::c_int;
use glib::traits::{Cast, Downcast, AsPtr, FromPtr, Envelope};
use gtk::ffi;
use gtk::Orientation;

pub trait BoxTrait { }

impl <T> BoxTrait for T
where T: AsPtr, *mut <T as AsPtr>::Inner: Cast<ffi::C_GtkBox>
{ }

pub struct Box_;

impl Box_ {
    pub fn new(orientation: Orientation, spacing: i32) -> Envelope<ffi::C_GtkBox> {
        unsafe {
            FromPtr::from_floating_ptr(
                ffi::gtk_box_new(orientation, spacing as c_int).unchecked_downcast())
        }
    }
}
