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

use std::c_str::ToCStr;
use gtk::{self, ffi};
use gtk::cast::GTK_COMBO_BOX;

pub trait ComboBoxTrait: gtk::WidgetTrait + gtk::ContainerTrait + gtk::BinTrait {
    fn get_wrap_width(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_wrap_width(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe { ffi::gtk_combo_box_set_wrap_width(GTK_COMBO_BOX(self.get_widget()), width) }
    }

    fn get_row_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_row_span_column(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_row_span_column(&self, row_span: i32) {
        unsafe { ffi::gtk_combo_box_set_row_span_column(GTK_COMBO_BOX(self.get_widget()), row_span) }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_column_span_column(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe { ffi::gtk_combo_box_set_column_span_column(GTK_COMBO_BOX(self.get_widget()), column_span) }
    }

    fn get_active(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_active(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_active(&self, active: i32) {
        unsafe { ffi::gtk_combo_box_set_active(GTK_COMBO_BOX(self.get_widget()), active) }
    }

    fn get_active_iter(&self) -> Option<gtk::TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_get_active_iter(GTK_COMBO_BOX(self.get_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::TreeIter::wrap_pointer(tmp_pointer))
        }
    }

    fn set_active_iter(&self, iter: &gtk::TreeIter) {
        unsafe { ffi::gtk_combo_box_set_active_iter(GTK_COMBO_BOX(self.get_widget()), iter.get_pointer()) }
    }

    fn get_id_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_id_column(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe { ffi::gtk_combo_box_set_id_column(GTK_COMBO_BOX(self.get_widget()), id_column) }
    }

    fn get_active_id(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_combo_box_get_active_id(GTK_COMBO_BOX(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(String::from_raw_buf(tmp as *const u8)) }
        }
    }

    fn set_active_id(&self, active_id: &str) -> bool {
        match unsafe {
            active_id.with_c_str(|c_str|{
                ffi::gtk_combo_box_set_active_id(GTK_COMBO_BOX(self.get_widget()), c_str)
            })
        } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn get_model(&self) -> Option<gtk::TreeModel> {
        let tmp = unsafe { ffi::gtk_combo_box_get_model(GTK_COMBO_BOX(self.get_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(gtk::TreeModel::wrap_pointer(tmp))
        }
    }

    fn set_model(&self, model: gtk::TreeModel) {
        unsafe { ffi::gtk_combo_box_set_model(GTK_COMBO_BOX(self.get_widget()), model.get_pointer()) }
    }

    fn popup(&self) {
        unsafe { ffi::gtk_combo_box_popup(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn popdown(&self) {
        unsafe { ffi::gtk_combo_box_popdown(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn get_focus_on_click(&self) -> bool {
        match unsafe { ffi::gtk_combo_box_get_focus_on_click(GTK_COMBO_BOX(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe { ffi::gtk_combo_box_set_focus_on_click(GTK_COMBO_BOX(self.get_widget()), if focus_on_click {ffi::GTRUE} else {ffi::GFALSE}) }
    }

    fn get_button_sensitivity(&self) -> gtk::SensitivityType {
        unsafe { ffi::gtk_combo_box_get_button_sensitivity(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_button_sensitivity(&self, sensitivity: gtk::SensitivityType) {
        unsafe { ffi::gtk_combo_box_set_button_sensitivity(GTK_COMBO_BOX(self.get_widget()), sensitivity) }
    }

    fn get_has_entry(&self) -> bool {
        match unsafe { ffi::gtk_combo_box_get_has_entry(GTK_COMBO_BOX(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe { ffi::gtk_combo_box_set_entry_text_column(GTK_COMBO_BOX(self.get_widget()), text_column) }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_entry_text_column(GTK_COMBO_BOX(self.get_widget())) }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe { ffi::gtk_combo_box_set_popup_fixed_width(GTK_COMBO_BOX(self.get_widget()), if fixed {ffi::GTRUE} else {ffi::GFALSE}) }
    }

    fn get_popup_fixed_width(&self) -> bool {
        match unsafe { ffi::gtk_combo_box_get_popup_fixed_width(GTK_COMBO_BOX(self.get_widget())) } {
            ffi::GFALSE => false,
            _ => true
        }
    }
}