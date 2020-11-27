// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::IsA;
use glib::translate::*;
use gobject;
use ibus_sys;
use std::fmt;
use Object;
use Service;

glib_wrapper! {
    pub struct ConfigService(Object<ibus_sys::IBusConfigService, ibus_sys::IBusConfigServiceClass, ConfigServiceClass>) @extends Service, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_config_service_get_type(),
    }
}

impl ConfigService {
    pub fn new(connection: &gio::DBusConnection) -> ConfigService {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_config_service_new(connection.to_glib_none().0))
        }
    }
}

pub const NONE_CONFIG_SERVICE: Option<&ConfigService> = None;

pub trait ConfigServiceExt: 'static {
    //fn value_changed(&self, section: &str, name: &str, value: /*Ignored*/&glib::Variant);
}

impl<O: IsA<ConfigService>> ConfigServiceExt for O {
    //fn value_changed(&self, section: &str, name: &str, value: /*Ignored*/&glib::Variant) {
    //    unsafe { TODO: call ibus_sys:ibus_config_service_value_changed() }
    //}
}

impl fmt::Display for ConfigService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigService")
    }
}