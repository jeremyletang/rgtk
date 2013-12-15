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

//! Toplevel which can contain other widgets

use std::{ptr, cast};
use std::libc::c_void;

use traits::{GtkWidget, GtkWindow, GtkContainer, GtkBin, Signal};
use ffi;
use gtk::enums::GtkWindowType;
use gtk::Widget;
use gdk::event::Event;

/**
* Window — Toplevel which can contain other widgets
*
* # Available signals:
* * `activate-default` : Action
* * `activate-focus` : Action
* * `keys-changed` : Run First
* * `set-focus` : Run Last
*/
pub struct Window {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
}

impl Window {
    pub fn new(window_type: GtkWindowType) -> Option<Window> {
        let tmp_pointer = unsafe { ffi::gtk_window_new(window_type) };
        check_pointer!(tmp_pointer, Window)
    }
}

impl_GtkWidget!(Window)

impl GtkContainer for Window {}
impl GtkWindow for Window {}
impl GtkBin for Window {}

///// All the default signals

pub trait WindowDefaultHandler {
    fn callback(&mut self, window: &Window);
}

extern_drop_handler!(drop_window_default_handler, WindowDefaultHandler)
extern_default_callback!(window_default_callback, WindowDefaultHandler)

// GtkWindow
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_active_default_signal, "activate-default")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_active_focus_signal, "activate-focus")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_keys_changed_signal, "activate-focus")

// GtkContainer
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_check_resize_signal, "check-resize")

// GtkWidget
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_accel_closure_changed_signal, "accel-closure-changed")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_composited_changed_signal, "composited-changed")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_destroy_signal, "destroy")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_grab_focus_signal, "grab-focus")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_hide_signal, "hide")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_map_signal, "map")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_realize_signal, "realize")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_show_signal, "show")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_style_updated_signal, "style-updated")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_unmap_signal, "unmap")
impl_connect_signal!(Window, drop_window_default_handler, window_default_callback, WindowDefaultHandler, connect_unrealize_signal, "unrealize")

///// Custom signals

pub trait WindowWidgetHandler {
    fn callback(&mut self, window: &Window, widget: &Widget);
}

extern_drop_handler!(drop_window_widget_handler, WindowWidgetHandler)

extern "C" fn window_widget_callback(object: *ffi::C_GtkWidget, widget: *ffi::C_GtkWidget, user_data: ffi::gpointer) {
    let mut handler = unsafe { cast::transmute::<ffi::gpointer, ~~WindowWidgetHandler>(user_data) };

    let window = GtkWidget::wrap_widget(object);
    let widget = GtkWidget::wrap_widget(widget);
    handler.callback(&window, &widget);

    unsafe {
        cast::forget(handler);
    }
}

pub trait WindowBoolEventHandler {
    fn callback(&mut self, window: &Window, event: &Event) -> bool;
}

extern_drop_handler!(drop_window_bool_event_handler, WindowBoolEventHandler)

extern "C" fn window_bool_event_callback(object: *ffi::C_GtkWidget, event: *ffi::C_GdkEvent, user_data: ffi::gpointer) -> ffi::Gboolean {
    let mut handler = unsafe { cast::transmute::<ffi::gpointer, ~~WindowBoolEventHandler>(user_data) };

    let window = GtkWidget::wrap_widget(object);
    let event = Event::wrap(event);
    let res = handler.callback(&window, &event);

    unsafe {
        cast::forget(handler);
    }

    if res { ffi::Gtrue } else { ffi::Gfalse }
}

pub trait WindowBoolUintHandler {
    fn callback(&mut self, window: &Window, x: u32) -> bool;
}

extern_drop_handler!(drop_window_bool_uint_handler, WindowBoolUintHandler)

extern "C" fn window_bool_uint_callback(object: *ffi::C_GtkWidget, x: u32, user_data: ffi::gpointer) -> ffi::Gboolean {
    let mut handler = unsafe { cast::transmute::<ffi::gpointer, ~~WindowBoolUintHandler>(user_data) };

    let window = GtkWidget::wrap_widget(object);
    let res = handler.callback(&window, x);

    unsafe {
        cast::forget(handler);
    }

    if res { ffi::Gtrue } else { ffi::Gfalse }
}

pub trait WindowBoolHandler {
    fn callback(&mut self, window: &Window, x: bool);
}

extern_drop_handler!(drop_window_bool_handler, WindowBoolHandler)

extern "C" fn window_bool_callback(object: *ffi::C_GtkWidget, x: ffi::Gboolean, user_data: ffi::gpointer) {
    let mut handler = unsafe { cast::transmute::<ffi::gpointer, ~~WindowBoolHandler>(user_data) };

    let window = GtkWidget::wrap_widget(object);
    let b = x != ffi::Gfalse;
    handler.callback(&window, b);

    unsafe {
        cast::forget(handler);
    }
}

// GtkWindow
impl_connect_signal!(Window, drop_window_widget_handler, window_widget_callback, WindowWidgetHandler, connect_window_set_focus_signal, "set-focus")

// GtkContainer
impl_connect_signal!(Window, drop_window_widget_handler, window_widget_callback, WindowWidgetHandler, connect_window_add_signal, "add")
impl_connect_signal!(Window, drop_window_widget_handler, window_widget_callback, WindowWidgetHandler, connect_window_remove_signal, "remove")
impl_connect_signal!(Window, drop_window_widget_handler, window_widget_callback, WindowWidgetHandler, connect_window_set_focus_child_signal, "set-focus-child")

// GtkWidget
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_button_press_event_signal, "button-press-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_button_release_event_signal, "button-release-event")
impl_connect_signal!(Window, drop_window_bool_uint_handler, window_bool_uint_callback, WindowBoolUintHandler, connect_can_activate_accel_signal, "can-activate-accel")
// TODO child-notify
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_configure_event_signal, "configure-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_damage_event_signal, "damage-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_delete_event_signal, "delete-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_destroy_event_signal, "destroy-event")
// TODO direction-chaned
// TODO drag-begin
// TODO drag-data-delete
// TODO drag-data-get
// TODO drag-data-received
// TODO drag-drop
// TODO drag-end
// TODO drag-failed
// TODO drag-leave
// TODO drag-motion
// TODO draw
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_enter_notify_event_signal, "enter-notify-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_event_signal, "event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_event_after_signal, "event-after")
// TODO focus
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_focus_in_event_signal, "focus-in-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_focus_out_event_signal, "focus-out-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_grab_broken_event_signal, "grab-broken-event")
impl_connect_signal!(Window, drop_window_bool_handler, window_bool_callback, WindowBoolHandler, connect_window_grab_notify_signal, "grab-notify")
// TODO hierarchy-changed
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_key_press_event_signal, "key-press-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_key_release_event_signal, "key-release-event")
// TODO keynav-failed
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_leave_notify_event_signal, "leave-notify-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_map_event_signal, "map-event")
// TODO mnemonic-activate
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_motion_notify_event_signal, "motion-notify-event")
// TODO move-focus
impl_connect_signal!(Window, drop_window_widget_handler, window_widget_callback, WindowWidgetHandler, connect_parent_set_signal, "parent-set")
// TODO popup-menu
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_property_notify_event_signal, "property-notify-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_proximity_in_event_signal, "proximity-in-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_proximity_out_event_signal, "proximity-out-event")
// TODO query-tooltip
// TODO screen-changed
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_scroll_event_signal, "scroll-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_selection_clear_event_signal, "selection-clear-event")
// TODO selection-get
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_selection_notify_event_signal, "selection-notify-event")
// TODO selection-received
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_selection_request_event_signal, "selection-request-event")
// TODO show-help
// TODO size-allocate
// TODO state-flags-changed
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_touch_event_signal, "touch-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_unmap_event_signal, "unmap-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_visibility_notify_event_signal, "visibility-notify-event")
impl_connect_signal!(Window, drop_window_bool_event_handler, window_bool_event_callback, WindowBoolEventHandler, connect_window_window_state_event_signal, "window-state-event")
