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
                signal_handlers:    ~[]
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
                    signal_handlers: ~[]
                }
            }
        }
    );
)

macro_rules! redirect_callback(
    ($gtk_struct:ident) => (
        extern fn redirect_callback(widget: *ffi::C_GtkWidget, user_data: *c_void) -> () {
            let mut button = $gtk_struct { pointer: widget, can_drop: false, signal_handlers: ~[]};
            let sighandler = unsafe {(user_data as *SignalHandler).to_option().unwrap()};
            let func = sighandler.function.unwrap();
            func(&mut button, sighandler.user_data);
        }
    );
)

macro_rules! redirect_callback_widget(
    ($gtk_struct:ident) => (
        extern fn redirect_callback_widget(widget: *ffi::C_GtkWidget, user_data: *c_void) -> () {
            let mut button = $gtk_struct { pointer: widget, can_drop: false, signal_handlers: ~[]};
            let sighandler = unsafe {(user_data as *SignalHandler).to_option().unwrap()};
            let user_data = if !user_data.is_null() {
                Some(unsafe { cast::transmute_mut((sighandler.user_data  as *$gtk_struct).to_option().unwrap())  as &mut GtkWidget})
            }  else {
                None
            };
            let func = sighandler.function_widget.unwrap();
            func(&mut button, user_data);
        }
    );
)

macro_rules! struct_signal(
    ($gtk_struct:ident) => (
        #[doc(hidden)]
        pub struct SignalHandler {
            function: Option<fn(&mut $gtk_struct, *c_void)>,
            function_widget: Option<fn(&mut $gtk_struct, Option<&mut GtkWidget>)>,
            user_data: *c_void
        }
    );
)


macro_rules! impl_signals(
    ($gtk_struct:ident) => (
        impl Signal for $gtk_struct {
            fn connect(&mut self, signal: &str, function: fn()) -> () {
                unsafe {
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect(self.pointer, c_str, Some(function))
                    })
                }
            }

            fn connect_2p<B>(&mut self,
                             signal: &str,
                             function: fn(&mut $gtk_struct, *c_void),
                             user_data: Option<&B>) -> () {
                let tmp_sighandler = ~SignalHandler {
                    function: Some(function),
                    function_widget: None,
                    user_data: user_data.unwrap() as *B as *c_void
                };
                unsafe{
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer,
                                                    c_str,
                                                    Some(redirect_callback),
                                                    &*tmp_sighandler as *SignalHandler as *c_void)
                    });
                }
                self.signal_handlers.push(tmp_sighandler);
            }

            fn connect_2p_widget<B: GtkWidget>(&mut self,
                                               signal: &str,
                                               function: fn(&mut $gtk_struct, Option<&mut GtkWidget>),
                                               user_data: Option<&B>) -> () {
                let tmp_sighandler = ~SignalHandler {
                    function: None,
                    function_widget: Some(function),
                    user_data: if user_data.is_some() {user_data.unwrap() as *B as *c_void } else { ptr::null() }
                };
                unsafe{
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer,
                                                    c_str,
                                                    Some(redirect_callback_widget),
                                                    &*tmp_sighandler as *SignalHandler as *c_void)
                    });
                }
                self.signal_handlers.push(tmp_sighandler);
            }

        }
    );
)
