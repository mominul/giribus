// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gobject;
use ibus_sys;
use std::fmt;
use Object;

glib_wrapper! {
    pub struct Service(Object<ibus_sys::IBusService, ibus_sys::IBusServiceClass, ServiceClass>) @extends Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_service_get_type(),
    }
}

impl Service {
    pub fn new(connection: &gio::DBusConnection, path: &str) -> Service {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_service_new(connection.to_glib_none().0, path.to_glib_none().0))
        }
    }
}

pub const NONE_SERVICE: Option<&Service> = None;

pub trait ServiceExt: 'static {
    //fn emit_signal(&self, dest_bus_name: &str, interface_name: &str, signal_name: &str, parameters: /*Ignored*/&glib::Variant, error: /*Ignored*/Option<glib::Error>) -> bool;

    fn get_connection(&self) -> Option<gio::DBusConnection>;

    fn get_object_path(&self) -> Option<GString>;

    //fn register(&self, connection: &gio::DBusConnection, error: /*Ignored*/Option<glib::Error>) -> bool;

    fn unregister(&self, connection: &gio::DBusConnection);
}

impl<O: IsA<Service>> ServiceExt for O {
    //fn emit_signal(&self, dest_bus_name: &str, interface_name: &str, signal_name: &str, parameters: /*Ignored*/&glib::Variant, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ibus_sys:ibus_service_emit_signal() }
    //}

    fn get_connection(&self) -> Option<gio::DBusConnection> {
        unsafe {
            from_glib_none(ibus_sys::ibus_service_get_connection(self.as_ref().to_glib_none().0))
        }
    }

    fn get_object_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_service_get_object_path(self.as_ref().to_glib_none().0))
        }
    }

    //fn register(&self, connection: &gio::DBusConnection, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ibus_sys:ibus_service_register() }
    //}

    fn unregister(&self, connection: &gio::DBusConnection) {
        unsafe {
            ibus_sys::ibus_service_unregister(self.as_ref().to_glib_none().0, connection.to_glib_none().0);
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Service")
    }
}