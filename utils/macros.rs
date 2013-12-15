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

#[macro_escape];

macro_rules! check_pointer(
    ($tmp_pointer:ident, $gtk_struct:ident) => (
        if $tmp_pointer.is_null() {
            None
        } else {
            Some($gtk_struct {
                pointer:            $tmp_pointer,
                can_drop:           true,
            })
        }
    );
)

macro_rules! impl_GtkWidget(
    ($gtk_struct:ident) => (
        impl GtkWidget for $gtk_struct {
            fn get_widget(&self) -> *ffi::C_GtkWidget {
                self.pointer
            }

            fn wrap_widget(widget: *ffi::C_GtkWidget) -> $gtk_struct {
                $gtk_struct {
                    pointer:         widget,
                    can_drop:        false,
                }
            }
        }
    );
)
