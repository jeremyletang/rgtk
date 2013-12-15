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

macro_rules! extern_drop_handler(
    ($extern_name:ident, $handler_type:ident) => (
        extern "C" fn $extern_name(data: ffi::gpointer, _closure: *ffi::C_GClosure) {
            unsafe {
                let handler = cast::transmute::<ffi::gpointer, ~~$handler_type>(data);
                drop(handler);
            }
        }
    )
)

macro_rules! extern_default_callback(
    ($extern_name:ident, $handler_name:ident) => (
        extern "C" fn $extern_name(object: *ffi::C_GtkWidget, user_data: ffi::gpointer) {
            let mut handler = unsafe { cast::transmute::<ffi::gpointer, ~~$handler_name>(user_data) };

            let window = GtkWidget::wrap_widget(object);
            handler.callback(&window);

            unsafe {
                cast::forget(handler);
            }
        }
    )
)

macro_rules! impl_connect_signal(
    ($type_name:ident, $handler_drop_fn:ident, $handler_callback_fn:ident, $handler_name:ident, $method_name:ident, $signal_name:expr) => (
        impl $type_name {
            pub fn $method_name(&self, handler: ~$handler_name) {
                let data = unsafe { cast::transmute::<~~$handler_name, ffi::gpointer>(~handler) };
                $signal_name.with_c_str(|cstr| {
                    unsafe {
                        ffi::g_signal_connect_data(self.pointer as ffi::gpointer,
                            cstr,
                            Some(cast::transmute($handler_callback_fn)),
                            data,
                            Some($handler_drop_fn),
                            0);
                    }
                });
            }
        }
    )
)
