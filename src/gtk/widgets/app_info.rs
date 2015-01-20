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

#![allow(unused_imports)]

use gtk::ffi;
use gtk::cast::{GTK_APP_INFO, GTK_APP_LAUNCH_CONTEXT};
use gtk;
use glib;
use glib::GlibContainer;
use std::str;
use std::string;

struct_Widget!(AppInfo);

impl AppInfo {/*
    pub fn create_from_commandline(commande_line: &str, application_name: &str, flag: gtk::AppInfoCreateFlags, error: &mut glib::Error) -> Option<AppInfo> {
        let tmp_pointer = unsafe {
            commande_line.with_c_str(|c_command_line| {
                application_name.with_c_str(|c_application_name| {
                    ffi::g_app_info_create_from_commandline(c_command_line, c_application_name, flag, &mut error.unwrap())
                })
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut gtk::ffi::C_GtkWidget))
        }
    }

    pub fn equals(&self, other: &AppInfo) -> bool {
        match unsafe { ffi::g_app_info_equal(GTK_APP_INFO(self.get_widget()), GTK_APP_INFO(other.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn get_id(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_id(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe {String::from_utf8(tmp_pointer) })
        }
    }

    pub fn get_name(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_name(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp_pointer) })
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_display_name(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp_pointer) })
        }
    }

    pub fn get_description(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_description(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp_pointer) })
        }
    }

    pub fn get_executable(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_executable(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp_pointer) })
        }
    }

    pub fn get_commandline(&self) -> Option<String> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_commandline(GTK_APP_INFO(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8(tmp_pointer) })
        }
    }

    pub fn launch(&self, files: &mut glib::List, launch_context: &mut gtk::AppLaunchContext, error: &mut glib::Error) -> bool {
        match unsafe { ffi::g_app_info_launch(GTK_APP_INFO(self.get_widget()),
            files.unwrap(),
            GTK_APP_LAUNCH_CONTEXT(launch_context.get_widget()),
            &mut error.unwrap()) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn supports_files(&self) -> bool {
        match unsafe { ffi::g_app_info_supports_files(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn supports_uris(&self) -> bool {
        match unsafe { ffi::g_app_info_supports_uris(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn launch_uris(&self, uris: &mut glib::List, launch_context: &mut gtk::AppLaunchContext, error: &mut glib::Error) -> bool {
        match unsafe { ffi::g_app_info_launch_uris(GTK_APP_INFO(self.get_widget()),
            uris.unwrap(),
            GTK_APP_LAUNCH_CONTEXT(launch_context.get_widget()),
            &mut error.unwrap()) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn should_show(&self) -> bool {
        match unsafe { ffi::g_app_info_should_show(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn can_delete(&self) -> bool {
        match unsafe { ffi::g_app_info_can_delete(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn delete(&self) -> bool {
        match unsafe { ffi::g_app_info_delete(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn reset_type_associations(&self, content_type: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(content_type.as_bytes());
            ffi::g_app_info_reset_type_associations(c_str)
            })
        }
    }

    pub fn set_as_default_for_type(&self, content_type: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_set_as_default_for_type(GTK_APP_INFO(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn set_as_default_for_extension(&self, extension: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            extension.with_c_str(|c_str| {
                ffi::g_app_info_set_as_default_for_extension(GTK_APP_INFO(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn set_as_last_used_for_type(&self, content_type: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_set_as_last_used_for_type(GTK_APP_INFO(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn add_supports_type(&self, content_type: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_add_supports_type(GTK_APP_INFO(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn can_remove_supports_type(&self) -> bool {
        match unsafe { ffi::g_app_info_can_remove_supports_type(GTK_APP_INFO(self.get_widget())) } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn remove_supports_type(&self, content_type: &str, error: &mut glib::Error) -> bool {
        match unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_remove_supports_type(GTK_APP_INFO(self.get_widget()), c_str, &mut error.unwrap())
            })
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }

    pub fn get_supported_types(&self) -> Vec<String> {
        let types = unsafe { ffi::g_app_info_get_supported_types(GTK_APP_INFO(self.get_widget())) };
        let mut ret = Vec::new();

        if types.is_not_null() {
            let mut it = 0;

            unsafe {
                loop {
                    let tmp = types.offset(it);

                    if tmp.is_null() {
                        break;
                    }
                    ret.push(String::from_utf8(*tmp));
                    it += 1;
                }
            }
        }
        ret
    }

    pub fn get_all() -> Option<glib::List> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_all() };

        if tmp_pointer.is_null() {
            None
        } else {
            glib::GlibContainer::wrap(tmp_pointer)
        }
    }

    pub fn get_all_for_type() -> Option<glib::List> {
        let tmp_pointer = unsafe { ffi::g_app_info_get_all_for_type() };

        if tmp_pointer.is_null() {
            None
        } else {
            glib::GlibContainer::wrap(tmp_pointer)
        }
    }

    pub fn get_default_for_type(content_type: &str, must_support_uris: bool) -> Option<AppInfo> {
        let tmp_pointer = unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_get_default_for_type(c_str, match must_support_uris {
                    true => ffi::GTRUE,
                    false => ffi::GFALSE
                }) })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut gtk::ffi::C_GtkWidget))
        }
    }

    pub fn get_default_for_uri_scheme(uri_scheme: &str) -> Option<AppInfo> {
        let tmp_pointer = unsafe {
            uri_scheme.with_c_str(|c_str| {
                ffi::g_app_info_get_default_for_uri_scheme(c_str)
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp_pointer as *mut gtk::ffi::C_GtkWidget))
        }
    }

    pub fn get_fallback_for_type(content_type: &str) -> Option<glib::List> {
        let tmp_pointer = unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_get_fallback_for_type(c_str)
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            glib::GlibContainer::wrap(tmp_pointer)
        }
    }

    pub fn get_recommended_for_type(content_type: &str) -> Option<glib::List> {
        let tmp_pointer = unsafe {
            content_type.with_c_str(|c_str| {
                ffi::g_app_info_get_recommended_for_type(c_str)
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            glib::GlibContainer::wrap(tmp_pointer)
        }
    }

    pub fn launch_default_for_uri(uri: &str, launch_context: &gtk::AppLaunchContext, error: &glib::Error) -> bool {
        match unsafe {
            uri.with_c_str(|c_str| {
                ffi::g_app_info_launch_default_for_uri(c_str, GTK_APP_LAUNCH_CONTEXT(launch_context.get_widget()), &mut error.unwrap())
        } {
            ffi::GTRUE => true,
            _ => false
        }
    }*/
}

/*impl Clone for AppInfo {
    fn clone(&self) -> AppInfo {
        let tmp_pointer = unsafe { ffi::g_app_info_dup(GTK_APP_INFO(self.get_widget())) };

        ffi::FFIWidget::wrap(tmp_pointer as *mut gtk::ffi::C_GtkWidget)
    }
}

impl PartialEq for AppInfo {
    fn eq(&self, other: &AppInfo) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &AppInfo) -> bool {
        !self.equals(other)
    }
}*/

impl_drop!(AppInfo);
impl_TraitWidget!(AppInfo);
