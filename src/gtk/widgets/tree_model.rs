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

use glib::ffi::GType;
use gtk::{self, ffi, GValue, TreeIter, TreePath};
use std::ffi::CString;
use libc;

pub struct TreeModel {
    pointer: *mut ffi::C_GtkTreeModel
}

impl TreeModel {
    pub fn get_flags(&self) -> gtk::TreeModelFlags {
        unsafe { ffi::gtk_tree_model_get_flags(self.pointer) }
    }

    pub fn get_n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.pointer) }
    }

    pub fn get_column_type(&self, index_: i32) -> GType {
        unsafe { ffi::gtk_tree_model_get_column_type(self.pointer, index_) }
    }

    pub fn get_iter(&self, iter: &mut TreeIter, path: &TreePath) -> bool {
        match unsafe { ffi::gtk_tree_model_get_iter(self.pointer, iter.get_pointer(), path.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_iter_from_string(&self, iter: &mut TreeIter, path_string: &str) -> bool {
        let c_str = CString::from_slice(path_string.as_bytes());
        match unsafe { ffi::gtk_tree_model_get_iter_from_string(self.pointer, iter.get_pointer(), c_str.as_ptr()) } {
                0 => false,
                _ => true
            }
    }

    pub fn get_iter_first(&self, iter: &mut TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_get_iter_first(self.pointer, iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_path(&self, iter: &TreeIter) -> Option<TreePath> {
        let tmp_pointer = unsafe { ffi::gtk_tree_model_get_path(self.pointer, iter.get_pointer()) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreePath::wrap_pointer(tmp_pointer))
        }
    }

    pub fn get_value(&self, iter: &TreeIter, column: i32) -> GValue {
        let value = GValue::new().unwrap();
        unsafe { ffi::gtk_tree_model_get_value(self.pointer, iter.get_pointer(), column, value.unwrap_pointer()) };
        value
    }

    pub fn iter_next(&self, iter: &mut TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_next(self.pointer, iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_previous(self.pointer, iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_children(&self, iter: &mut TreeIter, parent: Option<&TreeIter>) -> bool {
        match unsafe {
            ffi::gtk_tree_model_iter_children(self.pointer,
                                              iter.get_pointer(),
                                              if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().get_pointer() })
        } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_has_child(&self, iter: &TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_has_child(self.pointer, iter.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_n_children(&self, iter: &TreeIter) -> i32 {
        unsafe { ffi::gtk_tree_model_iter_n_children(self.pointer, iter.get_pointer()) }
    }

    pub fn iter_nth_child(&self, iter: &mut TreeIter, parent: Option<&TreeIter>, n: i32) -> bool {
        match unsafe {
            ffi::gtk_tree_model_iter_nth_child(self.pointer,
                                               iter.get_pointer(),
                                               if parent.is_none() { ::std::ptr::null_mut() } else { parent.unwrap().get_pointer() },
                                               n)
        } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_parent(&self, iter: &mut TreeIter, child: &TreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_parent(self.pointer, iter.get_pointer(), child.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    #[allow(unused_variables)]
    pub fn get_string_from_iter(&self, iter: &TreeIter) -> String {
        let string = unsafe { ffi::gtk_tree_model_get_string_from_iter(self.pointer, iter.get_pointer()) };

        if string.is_null() {
            String::new()
        } else {
            unsafe {
                let res = String::from_utf8(string as *const u8);
                libc::free(string);
                res
            }
        }
    }

    pub fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_changed(self.pointer, path.get_pointer(), iter.get_pointer()) }
    }

    pub fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_inserted(self.pointer, path.get_pointer(), iter.get_pointer()) }
    }

    pub fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_row_has_child_toggled(self.pointer, path.get_pointer(), iter.get_pointer()) }
    }

    pub fn row_deleted(&self, path: &TreePath) {
        unsafe { ffi::gtk_tree_model_row_deleted(self.pointer, path.get_pointer()) }
    }

    pub fn rows_reordered(&self, path: &TreePath, iter: Option<&TreeIter>, new_order: &mut [i32]) {
        unsafe {
            ffi::gtk_tree_model_rows_reordered(self.pointer,
                                               path.get_pointer(),
                                               if iter.is_none() { ::std::ptr::null_mut() } else { iter.unwrap().get_pointer() },
                                               new_order.as_mut_ptr())
        }
    }

    pub fn ref_node(&self, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_ref_node(self.pointer, iter.get_pointer()) }
    }

    pub fn unref_node(&self, iter: &TreeIter) {
        unsafe { ffi::gtk_tree_model_unref_node(self.pointer, iter.get_pointer()) }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkTreeModel {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treemodel: *mut ffi::C_GtkTreeModel) -> TreeModel {
        TreeModel {
            pointer: c_treemodel
        }
    }
}

impl_drop!(TreeModel, GTK_TREE_MODEL);
