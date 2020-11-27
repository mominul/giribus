// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject;
use ibus_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Object;
use Serializable;
use XEventType;

glib_wrapper! {
    pub struct XEvent(Object<ibus_sys::IBusXEvent, ibus_sys::IBusXEventClass, XEventClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_x_event_get_type(),
    }
}

impl XEvent {
    //pub fn new(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> XEvent {
    //    unsafe { TODO: call ibus_sys:ibus_x_event_new() }
    //}
}

pub const NONE_XEVENT: Option<&XEvent> = None;

pub trait XEventExt: 'static {
    fn get_event_type(&self) -> XEventType;

    fn get_group(&self) -> u8;

    fn get_hardware_keycode(&self) -> u16;

    fn get_is_modifier(&self) -> bool;

    fn get_keyval(&self) -> u32;

    fn get_length(&self) -> i32;

    fn get_purpose(&self) -> Option<GString>;

    fn get_root(&self) -> u32;

    fn get_same_screen(&self) -> bool;

    fn get_send_event(&self) -> i8;

    fn get_serial(&self) -> libc::c_ulong;

    fn get_state(&self) -> u32;

    fn get_string(&self) -> Option<GString>;

    fn get_subwindow(&self) -> u32;

    fn get_time(&self) -> u32;

    fn get_version(&self) -> u32;

    fn get_window(&self) -> u32;

    fn get_x(&self) -> i32;

    fn get_x_root(&self) -> i32;

    fn get_y(&self) -> i32;

    fn get_y_root(&self) -> i32;

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<XEvent>> XEventExt for O {
    fn get_event_type(&self) -> XEventType {
        unsafe {
            from_glib(ibus_sys::ibus_x_event_get_event_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_group(&self) -> u8 {
        unsafe {
            ibus_sys::ibus_x_event_get_group(self.as_ref().to_glib_none().0)
        }
    }

    fn get_hardware_keycode(&self) -> u16 {
        unsafe {
            ibus_sys::ibus_x_event_get_hardware_keycode(self.as_ref().to_glib_none().0)
        }
    }

    fn get_is_modifier(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_x_event_get_is_modifier(self.as_ref().to_glib_none().0))
        }
    }

    fn get_keyval(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_keyval(self.as_ref().to_glib_none().0)
        }
    }

    fn get_length(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_x_event_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn get_purpose(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_x_event_get_purpose(self.as_ref().to_glib_none().0))
        }
    }

    fn get_root(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_root(self.as_ref().to_glib_none().0)
        }
    }

    fn get_same_screen(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_x_event_get_same_screen(self.as_ref().to_glib_none().0))
        }
    }

    fn get_send_event(&self) -> i8 {
        unsafe {
            ibus_sys::ibus_x_event_get_send_event(self.as_ref().to_glib_none().0)
        }
    }

    fn get_serial(&self) -> libc::c_ulong {
        unsafe {
            ibus_sys::ibus_x_event_get_serial(self.as_ref().to_glib_none().0)
        }
    }

    fn get_state(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_state(self.as_ref().to_glib_none().0)
        }
    }

    fn get_string(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_x_event_get_string(self.as_ref().to_glib_none().0))
        }
    }

    fn get_subwindow(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_subwindow(self.as_ref().to_glib_none().0)
        }
    }

    fn get_time(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_time(self.as_ref().to_glib_none().0)
        }
    }

    fn get_version(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_version(self.as_ref().to_glib_none().0)
        }
    }

    fn get_window(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_x_event_get_window(self.as_ref().to_glib_none().0)
        }
    }

    fn get_x(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_x_event_get_x(self.as_ref().to_glib_none().0)
        }
    }

    fn get_x_root(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_x_event_get_x_root(self.as_ref().to_glib_none().0)
        }
    }

    fn get_y(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_x_event_get_y(self.as_ref().to_glib_none().0)
        }
    }

    fn get_y_root(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_x_event_get_y_root(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_version_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusXEvent, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<XEvent>
        {
            let f: &F = &*(f as *const F);
            f(&XEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::version\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_version_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for XEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XEvent")
    }
}