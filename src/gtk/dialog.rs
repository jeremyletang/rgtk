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

use std::ptr;
use std::iter::IntoIterator;
use libc::c_char;
use glib::translate::*;
use gtk::cast::*;
use gtk::{self, ffi};
use gtk::{
    FFIWidget,
    WidgetTrait,
    ContainerTrait,
    BinTrait,
    WindowTrait,
    AppChooserTrait,
    ColorChooserTrait,
    FileChooserTrait,
    FontChooserTrait,
    RecentChooserTrait,
    Window,
};

/// Dialog response
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Response {
    /// Returned if an action widget has no response id, or if the dialog gets programmatically hidden or destroyed
    None,
    /// Generic response id, not used by GTK+ dialogs
    Reject,
    /// Generic response id, not used by GTK+ dialogs
    Accept,
    /// Returned if the dialog is deleted
    DeleteEvent,
    /// Returned by Ok buttons in GTK+ dialogs
    Ok,
    /// Returned by Cancel buttons in GTK+ dialogs
    Cancel,
    /// Returned by Close buttons in GTK+ dialogs
    Close,
    /// Returned by Yes buttons in GTK+ dialogs
    Yes,
    /// Returned by No buttons in GTK+ dialogs
    No,
    /// Returned by Apply buttons in GTK+ dialogs
    Apply,
    /// Returned by Help buttons in GTK+ dialogs
    Help,
    /// User-defined variants
    User(u16),
}

impl FromGlib for Response {
    type GlibType = ffi::c_enum;

    fn from_glib(val: ffi::c_enum) -> Response {
        match val {
            ffi::GTK_RESPONSE_NONE => Response::None,
            ffi::GTK_RESPONSE_REJECT => Response::Reject,
            ffi::GTK_RESPONSE_ACCEPT => Response::Accept,
            ffi::GTK_RESPONSE_DELETE_EVENT => Response::DeleteEvent,
            ffi::GTK_RESPONSE_OK => Response::Ok,
            ffi::GTK_RESPONSE_CANCEL => Response::Cancel,
            ffi::GTK_RESPONSE_CLOSE => Response::Close,
            ffi::GTK_RESPONSE_YES => Response::Yes,
            ffi::GTK_RESPONSE_NO => Response::No,
            ffi::GTK_RESPONSE_APPLY => Response::Apply,
            ffi::GTK_RESPONSE_HELP => Response::Help,
            x @ 0...65535 => Response::User(x as u16),
            _ => Response::None,
        }
    }
}

impl ToGlib for Response {
    type GlibType = ffi::c_enum;

    fn to_glib(&self) -> ffi::c_enum {
        match *self {
            Response::None => ffi::GTK_RESPONSE_NONE,
            Response::Reject => ffi::GTK_RESPONSE_REJECT,
            Response::Accept => ffi::GTK_RESPONSE_ACCEPT,
            Response::DeleteEvent => ffi::GTK_RESPONSE_DELETE_EVENT,
            Response::Ok => ffi::GTK_RESPONSE_OK,
            Response::Cancel => ffi::GTK_RESPONSE_CANCEL,
            Response::Close => ffi::GTK_RESPONSE_CLOSE,
            Response::Yes => ffi::GTK_RESPONSE_YES,
            Response::No => ffi::GTK_RESPONSE_NO,
            Response::Apply => ffi::GTK_RESPONSE_APPLY,
            Response::Help => ffi::GTK_RESPONSE_HELP,
            Response::User(x) => x as ffi::c_enum,
        }
    }
}

pub trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

impl AsRef<str> for str {
    fn as_ref(&self) -> &str {
        self
    }
}

impl <T> AsRef<[T]> for [T] {
    fn as_ref(&self) -> &[T] {
        self
    }
}

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            impl<T> AsRef<[T]> for [T; $N] {
                #[inline]
                fn as_ref(&self) -> &[T] {
                    &self[..]
                }
            }
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

/// Predefined popular button combinations
///
/// The GNOME Human Interface Guidelines recommend[1] putting the cancel button
/// before the affirmative one and using a descriptive verb instead of generic "Ok"
/// for the latter.
///
/// [1]: https://developer.gnome.org/hig/stable/dialogs.html.en#primary-buttons
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Buttons {
    /// An Ok button
    Ok,
    /// A Close button
    Close,
    /// A Cancel button
    Cancel,
    /// No and Yes buttons
    YesNo,
    /// Cancel and Ok buttons
    OkCancel,
}

const OK: &'static [(&'static str, Response); 1] = &[("Ok", Response::Ok)];
const CLOSE: &'static [(&'static str, Response); 1] = &[("Close", Response::Close)];
const CANCEL: &'static [(&'static str, Response); 1] = &[("Cancel", Response::Cancel)];
const CANCEL_OK: &'static [(&'static str, Response); 2] =
    &[("Cancel", Response::Cancel), ("Ok", Response::Ok)];
const NO_YES: &'static [(&'static str, Response); 2] =
    &[("No", Response::No), ("Yes", Response::Yes)];

impl <'a> AsRef<[(&'a str, Response)]> for Buttons {
    fn as_ref(&self) -> &[(&'a str, Response)] {
        match *self {
            Buttons::Ok => OK,
            Buttons::Close => CLOSE,
            Buttons::Cancel => CANCEL,
            Buttons::YesNo => NO_YES,
            Buttons::OkCancel => CANCEL_OK,
        }
    }
}

pub trait DialogTrait: WidgetTrait + ContainerTrait + BinTrait + WindowTrait {
    fn run(&self) -> Response {
        unsafe {
            from_glib(
                ffi::gtk_dialog_run(GTK_DIALOG(self.unwrap_widget())))
        }
    }

    fn response(&self, response: Response) {
        unsafe {
            ffi::gtk_dialog_response(GTK_DIALOG(self.unwrap_widget()), response.to_glib())
        }
    }

    fn add_button(&self, button_text: &str, response: Response) -> gtk::Button {
        unsafe {
            FFIWidget::wrap_widget(
                ffi::gtk_dialog_add_button(
                    GTK_DIALOG(self.unwrap_widget()),
                    button_text.borrow_to_glib().0,
                    response.to_glib()))
        }
    }

    fn add_buttons<'a, B: AsRef<[(&'a str, Response)]>>(&self, buttons: B) {
        for item in buttons.as_ref() {
            self.add_button(item.0, item.1);
        }
    }

    fn add_action_widget<T: WidgetTrait>(&self, child: &T, response: Response) {
        unsafe {
            ffi::gtk_dialog_add_action_widget(
                GTK_DIALOG(self.unwrap_widget()),
                child.unwrap_widget(),
                response.to_glib())
        }
    }

    fn set_default_response(&self, response: Response) -> () {
        unsafe {
            ffi::gtk_dialog_set_default_response(
                GTK_DIALOG(self.unwrap_widget()),
                response.to_glib())
        }
    }

    fn set_response_sensitive(&self, response: Response, setting: bool) -> () {
        unsafe {
            ffi::gtk_dialog_set_response_sensitive(
                GTK_DIALOG(self.unwrap_widget()),
                response.to_glib(),
                setting.to_glib())
        }
    }

    fn get_response_for_widget<T: WidgetTrait>(&self, widget: &T) -> Response {
        unsafe {
            from_glib(
                ffi::gtk_dialog_get_response_for_widget(
                    GTK_DIALOG(self.unwrap_widget()),
                    widget.unwrap_widget()))
        }
    }

    fn get_widget_for_reponse<T: WidgetTrait>(&self, response: Response) -> Option<T> {
        unsafe {
            FFIWidget::wrap_widget(
                ffi::gtk_dialog_get_widget_for_response(
                    GTK_DIALOG(self.unwrap_widget()),
                    response.to_glib()))
        }
    }

    fn get_action_area<T: WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_action_area(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    fn get_content_area<T: WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_content_area(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    #[cfg(any(feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn get_header_bar<T: WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_dialog_get_header_bar(GTK_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// GtkDialog
//////////////////////////////////////////////////////////////////////////////

/// A generic dialog
struct_Widget!(Dialog);

impl Dialog {
    pub fn new() -> Dialog {
        unsafe {
            FFIWidget::wrap_widget(
                ffi::gtk_dialog_new())
        }
    }

    pub fn with_buttons<'a, B>(title: &str, parent: Option<&Window>,
                               buttons: B) -> Dialog
        where B: AsRef<[(&'a str, Response)]> {
        unsafe {
            let mut flags = 0;
            if parent.is_some() {
                flags |= ffi::GTK_DIALOG_MODAL | ffi::GTK_DIALOG_DESTROY_WITH_PARENT;
            }
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            let res: Dialog = FFIWidget::wrap_widget(
                ffi::gtk_dialog_new_with_buttons(
                    title.borrow_to_glib().0,
                    parent,
                    flags,
                    ptr::null()));
            res.add_buttons(buttons);
            res
        }
    }
}

impl_drop!(Dialog);
impl_TraitWidget!(Dialog);

impl ContainerTrait for Dialog {}
impl BinTrait for Dialog {}
impl WindowTrait for Dialog {}
impl DialogTrait for Dialog {}

impl_widget_events!(Dialog);

//////////////////////////////////////////////////////////////////////////////
// GtkMessageDialog
//////////////////////////////////////////////////////////////////////////////

/// A type of message dialog
pub use gtk::ffi::enums::MessageType;

/// A message dialog

struct_Widget!(Message);

impl Message {
    /// Creates a message dialog
    pub fn new<'a, B>(parent: Option<&Window>, message_type: MessageType,
                      buttons: B, message: &str) -> Message
        where B: AsRef<[(&'a str, Response)]> {
        unsafe {
            let mut flags = 0;
            if parent.is_some() {
                flags |= ffi::GTK_DIALOG_MODAL | ffi::GTK_DIALOG_DESTROY_WITH_PARENT;
            }
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            let res: Message = FFIWidget::wrap_widget(
                ffi::gtk_message_dialog_new(
                    parent,
                    flags, message_type, ffi::GTK_BUTTONS_NONE,
                    "%s".borrow_to_glib().0, message.borrow_to_glib().0,
                    ptr::null::<c_char>()));
            res.add_buttons(buttons);
            res
        }
    }

    pub fn with_markup<'a, B>(parent: Option<&Window>, message_type: MessageType,
                              buttons: B, markup: &str) -> Message
        where B: AsRef<[(&'a str, Response)]> {
        let res = Message::new(parent, message_type, buttons, "");
        res.set_markup(markup);
        res
    }

    pub fn set_markup(&self, markup: &str) -> () {
        unsafe {
            ffi::gtk_message_dialog_set_markup(GTK_MESSAGE_DIALOG(self.unwrap_widget()), markup.borrow_to_glib().0)
        }
    }

    pub fn get_message_area<T: WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_message_dialog_get_message_area(GTK_MESSAGE_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(Message);
impl_TraitWidget!(Message);

impl ContainerTrait for Message {}
impl BinTrait for Message {}
impl WindowTrait for Message {}
impl DialogTrait for Message {}

impl_widget_events!(Message);

//////////////////////////////////////////////////////////////////////////////
// GtkAboutDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(About);

impl About {
    pub fn new() -> About {
        unsafe {
            FFIWidget::wrap_widget(
                ffi::gtk_about_dialog_new())
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_program_name(&self, name: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(self.unwrap_widget()), name.borrow_to_glib().0)
        };
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_version(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_version(&self, version: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(self.unwrap_widget()), version.borrow_to_glib().0)
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_copyright(&self, copyright: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(self.unwrap_widget()), copyright.borrow_to_glib().0)
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_comments(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_comments(&self, comments: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(self.unwrap_widget()), comments.borrow_to_glib().0)
        };
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_license(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_license(&self, license: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_license(GTK_ABOUT_DIALOG(self.unwrap_widget()), license.borrow_to_glib().0)
        };
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe { from_glib(ffi::gtk_about_dialog_get_wrap_license(GTK_ABOUT_DIALOG(self.unwrap_widget()))) }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                wrap_license.to_glib())
        }
    }

    pub fn get_license_type(&self) -> gtk::License {
        unsafe { ffi::gtk_about_dialog_get_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget())) }
    }

    pub fn set_license_type(&self, license_type: gtk::License) -> () {
        unsafe { ffi::gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(self.unwrap_widget()), license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_website(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website(&self, website: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(self.unwrap_widget()), website.borrow_to_glib().0)
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_website_label(&self, website_label: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(self.unwrap_widget()), website_label.borrow_to_glib().0)
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::borrow(
                ffi::gtk_about_dialog_get_authors(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_authors<'a, S, I: ?Sized>(&self, authors: &'a I)
    where S: Str, &'a I: IntoIterator<Item = &'a S> {
        unsafe {
            ffi::gtk_about_dialog_set_authors(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                authors.borrow_to_glib().0);
        }
    }

    pub fn get_artists(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::borrow(
                ffi::gtk_about_dialog_get_artists(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_artists<'a, S, I: ?Sized>(&self, artists: &'a I)
    where S: Str, &'a I: IntoIterator<Item = &'a S> {
        unsafe {
            ffi::gtk_about_dialog_set_artists(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                artists.borrow_to_glib().0);
        }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::borrow(
                ffi::gtk_about_dialog_get_documenters(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_documenters<'a, S, I: ?Sized>(&self, documenters: &'a I)
    where S: Str, &'a I: IntoIterator<Item = &'a S> {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                documenters.borrow_to_glib().0);
        }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_translator_credits(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                translator_credits.borrow_to_glib().0)
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { FFIWidget::wrap_widget(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) -> () {
        unsafe { ffi::gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(self.unwrap_widget()), GDK_PIXBUF(logo.unwrap_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_about_dialog_get_logo_icon_name(GTK_ABOUT_DIALOG(self.unwrap_widget())))
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) -> () {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                logo_icon_name.borrow_to_glib().0)
        };
    }

    pub fn add_credit_section<'a, S, I: ?Sized>(&self, section_name: &str, people: &'a I)
    where S: Str, &'a I: IntoIterator<Item = &'a S> {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(
                GTK_ABOUT_DIALOG(self.unwrap_widget()),
                section_name.borrow_to_glib().0,
                people.borrow_to_glib().0)
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) -> () {
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl_drop!(About);
impl_TraitWidget!(About);

impl ContainerTrait for About {}
impl BinTrait for About {}
impl WindowTrait for About {}
impl DialogTrait for About {}

impl_widget_events!(About);

//////////////////////////////////////////////////////////////////////////////
// GtkAppChooserDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(AppChooser);

impl AppChooser {
    pub fn new_for_content_type(parent: Option<&Window>, content_type: &str) -> AppChooser {
        unsafe {
            let mut flags = 0;
            if parent.is_some() {
                flags |= ffi::GTK_DIALOG_MODAL | ffi::GTK_DIALOG_DESTROY_WITH_PARENT;
            }
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            FFIWidget::wrap_widget(
                ffi::gtk_app_chooser_dialog_new_for_content_type(
                    parent,
                    flags,
                    content_type.borrow_to_glib().0))
        }
    }

    pub fn widget<T: WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_dialog_get_widget(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_heading(&self, heading: &str) -> () {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget()), heading.borrow_to_glib().0)
        }
    }

    pub fn get_heading(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_app_chooser_dialog_get_heading(GTK_APP_CHOOSER_DIALOG(self.unwrap_widget())))
        }
    }
}

impl_drop!(AppChooser);
impl_TraitWidget!(AppChooser);

impl ContainerTrait for AppChooser {}
impl BinTrait for AppChooser {}
impl WindowTrait for AppChooser {}
impl DialogTrait for AppChooser {}
impl AppChooserTrait for AppChooser {}

impl_widget_events!(AppChooser);

//////////////////////////////////////////////////////////////////////////////
// GtkColorChooserDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(ColorChooser);

impl ColorChooser {
    pub fn new(title: &str, parent: Option<&Window>) -> ColorChooser {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            FFIWidget::wrap_widget(
                ffi::gtk_color_chooser_dialog_new(title.borrow_to_glib().0, parent))
        }
    }
}

impl_drop!(ColorChooser);
impl_TraitWidget!(ColorChooser);

impl ContainerTrait for ColorChooser {}
impl BinTrait for ColorChooser {}
impl WindowTrait for ColorChooser {}
impl DialogTrait for ColorChooser {}
impl ColorChooserTrait for ColorChooser {}

impl_widget_events!(ColorChooser);

//////////////////////////////////////////////////////////////////////////////
// GtkFileChooserDialog
//////////////////////////////////////////////////////////////////////////////

/// File chooser dialog

struct_Widget!(FileChooser);

impl FileChooser {
    pub fn new<'a, B>(title: &str, parent: Option<&Window>,
                      action: gtk::FileChooserAction, buttons: B) -> FileChooser
        where B: AsRef<[(&'a str, Response)]> {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            let res: FileChooser = FFIWidget::wrap_widget(
                ffi::gtk_file_chooser_dialog_new(
                    title.borrow_to_glib().0,
                    parent,
                    action,
                    ptr::null()));
            res.add_buttons(buttons);
            res
        }
    }
}

impl_drop!(FileChooser);
impl_TraitWidget!(FileChooser);

impl ContainerTrait for FileChooser {}
impl BinTrait for FileChooser {}
impl WindowTrait for FileChooser {}
impl DialogTrait for FileChooser {}
impl FileChooserTrait for FileChooser {}

impl_widget_events!(FileChooser);

//////////////////////////////////////////////////////////////////////////////
// GtkFontChooserDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(FontChooser);

impl FontChooser {
    pub fn new(title: &str, parent: Option<&Window>) -> FontChooser {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            FFIWidget::wrap_widget(
                ffi::gtk_font_chooser_dialog_new(title.borrow_to_glib().0, parent))
        }
    }
}

impl_drop!(FontChooser);
impl_TraitWidget!(FontChooser);

impl ContainerTrait for FontChooser {}
impl BinTrait for FontChooser {}
impl WindowTrait for FontChooser {}
impl DialogTrait for FontChooser {}
impl FontChooserTrait for FontChooser {}

impl_widget_events!(FontChooser);

/*
//////////////////////////////////////////////////////////////////////////////
// GtkPageSetupUnixDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(PageSetupUnix);

impl PageSetupUnix {
    pub fn new(title: &str, parent: Option<&Window>) -> PageSetupUnix {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            FFIWidget::wrap_widget(
                ffi::gtk_page_setup_unix_dialog_new(title.borrow_to_glib().0, parent))
        }
    }

    pub fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PAGE_SETUP(page_setup.unwrap_widget())) }
    }

    pub fn get_page_setup(&self) -> Option<PageSetup> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PRINT_SETTINGS(print_settings.unwrap_widget())) }
    }

    pub fn get_print_settings(&self) -> Option<PrintSettings> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(PageSetupUnix);
impl_TraitWidget!(PageSetupUnix);

impl ContainerTrait for PageSetupUnix {}
impl BinTrait for PageSetupUnix {}
impl WindowTrait for PageSetupUnix {}
impl DialogTrait for PageSetupUnix {}

impl_widget_events!(PageSetupUnix);
*/

//////////////////////////////////////////////////////////////////////////////
// GtkRecentChooserDialog
//////////////////////////////////////////////////////////////////////////////

struct_Widget!(RecentChooser);

impl RecentChooser {
    pub fn new<'a, B>(title: &str, parent: Option<&Window>, buttons: B) -> RecentChooser
        where B: AsRef<[(&'a str, Response)]> {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            let res: RecentChooser = FFIWidget::wrap_widget(
                ffi::gtk_recent_chooser_dialog_new(
                    title.borrow_to_glib().0,
                    parent,
                    ptr::null()));
            res.add_buttons(buttons);
            res
        }
    }

    pub fn new_for_manager<'a, B>(title: &str, parent: Option<&Window>,
                                  manager: &gtk::RecentManager, buttons: B) -> RecentChooser
        where B: AsRef<[(&'a str, Response)]> {
        unsafe {
            let parent = parent.map_or(ptr::null_mut(), |w| GTK_WINDOW(w.unwrap_widget()));
            let res: RecentChooser = FFIWidget::wrap_widget(
                ffi::gtk_recent_chooser_dialog_new_for_manager(
                    title.borrow_to_glib().0,
                    parent,
                    GTK_RECENT_MANAGER(manager.unwrap_widget()),
                    ptr::null()));
            res.add_buttons(buttons);
            res
        }
    }
}

impl_drop!(RecentChooser);
impl_TraitWidget!(RecentChooser);

impl ContainerTrait for RecentChooser {}
impl BinTrait for RecentChooser {}
impl WindowTrait for RecentChooser {}
impl DialogTrait for RecentChooser {}
impl RecentChooserTrait for RecentChooser {}

impl_widget_events!(RecentChooser);
