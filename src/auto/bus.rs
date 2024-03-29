// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject;
use gobject_sys;
use ibus_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Component;
use Config;
use EngineDesc;
use InputContext;
use Object;

glib_wrapper! {
    pub struct Bus(Object<ibus_sys::IBusBus, ibus_sys::IBusBusClass, BusClass>) @extends Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_bus_get_type(),
    }
}

impl Bus {
    pub fn new() -> Bus {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_new())
        }
    }

    pub fn new_async() -> Bus {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_new_async())
        }
    }

    pub fn new_async_client() -> Bus {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_new_async_client())
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BUS: Option<&Bus> = None;

pub trait BusExt: 'static {
    fn add_match(&self, rule: &str) -> bool;

    //fn add_match_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, rule: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn create_input_context(&self, client_name: &str) -> Option<InputContext>;

    //fn create_input_context_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, client_name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn current_input_context(&self) -> Option<GString>;

    //fn current_input_context_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn exit(&self, restart: bool) -> bool;

    //fn exit_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, restart: bool, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn get_config(&self) -> Option<Config>;

    fn get_connection(&self) -> Option<gio::DBusConnection>;

    fn get_engines_by_names(&self, names: &[&str]) -> Vec<EngineDesc>;

    fn get_global_engine(&self) -> Option<EngineDesc>;

    //fn get_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    //fn get_ibus_property(&self, property_name: &str) -> /*Ignored*/Option<glib::Variant>;

    //fn get_ibus_property_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, property_name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn get_name_owner(&self, name: &str) -> Option<GString>;

    //fn get_name_owner_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn get_service_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v1_5_3", deprecated)]
    fn get_use_global_engine(&self) -> bool;

    //#[cfg_attr(feature = "v1_5_3", deprecated)]
    //fn get_use_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    #[cfg_attr(feature = "v1_5_3", deprecated)]
    fn get_use_sys_layout(&self) -> bool;

    //#[cfg_attr(feature = "v1_5_3", deprecated)]
    //fn get_use_sys_layout_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn hello(&self) -> Option<GString>;

    fn is_connected(&self) -> bool;

    #[cfg_attr(feature = "v1_5_3", deprecated)]
    fn is_global_engine_enabled(&self) -> bool;

    //#[cfg_attr(feature = "v1_5_3", deprecated)]
    //fn is_global_engine_enabled_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    #[cfg_attr(feature = "v1_5_3", deprecated)]
    fn list_active_engines(&self) -> Vec<EngineDesc>;

    //#[cfg_attr(feature = "v1_5_3", deprecated)]
    //fn list_active_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn list_engines(&self) -> Vec<EngineDesc>;

    //fn list_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn list_names(&self) -> Vec<GString>;

    fn list_queued_owners(&self, name: &str) -> Vec<GString>;

    fn name_has_owner(&self, name: &str) -> bool;

    //fn name_has_owner_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn preload_engines(&self, names: &[&str]) -> bool;

    //fn preload_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, names: &[&str], timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn register_component<P: IsA<Component>>(&self, component: &P) -> bool;

    //fn register_component_async<P: IsA<Component>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(), glib::Error>) + 'static>(&self, component: &P, timeout_msec: i32, cancellable: Option<&Q>, callback: R);

    fn release_name(&self, name: &str) -> u32;

    //fn release_name_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn remove_match(&self, rule: &str) -> bool;

    //fn remove_match_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, rule: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn request_name(&self, name: &str, flags: u32) -> u32;

    //fn request_name_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, flags: u32, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn set_global_engine(&self, global_engine: &str) -> bool;

    //fn set_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, global_engine: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    //fn set_ibus_property(&self, property_name: &str, value: /*Ignored*/&glib::Variant);

    //fn set_ibus_property_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, property_name: &str, value: /*Ignored*/&glib::Variant, timeout_msec: i32, cancellable: Option<&P>, callback: Q);

    fn set_watch_dbus_signal(&self, watch: bool);

    fn set_watch_ibus_signal(&self, watch: bool);

    fn get_property_client_only(&self) -> bool;

    fn get_property_connect_async(&self) -> bool;

    fn connect_connected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_global_engine_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_name_owner_changed<F: Fn(&Self, &str, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Bus>> BusExt for O {
    fn add_match(&self, rule: &str) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_add_match(self.as_ref().to_glib_none().0, rule.to_glib_none().0))
        }
    }

    //fn add_match_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, rule: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_add_match_async() }
    //}

    fn create_input_context(&self, client_name: &str) -> Option<InputContext> {
        unsafe {
            from_glib_full(ibus_sys::ibus_bus_create_input_context(self.as_ref().to_glib_none().0, client_name.to_glib_none().0))
        }
    }

    //fn create_input_context_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, client_name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_create_input_context_async() }
    //}

    fn current_input_context(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ibus_sys::ibus_bus_current_input_context(self.as_ref().to_glib_none().0))
        }
    }

    //fn current_input_context_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_current_input_context_async() }
    //}

    fn exit(&self, restart: bool) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_exit(self.as_ref().to_glib_none().0, restart.to_glib()))
        }
    }

    //fn exit_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, restart: bool, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_exit_async() }
    //}

    fn get_config(&self) -> Option<Config> {
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_get_config(self.as_ref().to_glib_none().0))
        }
    }

    fn get_connection(&self) -> Option<gio::DBusConnection> {
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_get_connection(self.as_ref().to_glib_none().0))
        }
    }

    fn get_engines_by_names(&self, names: &[&str]) -> Vec<EngineDesc> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_bus_get_engines_by_names(self.as_ref().to_glib_none().0, names.to_glib_none().0))
        }
    }

    fn get_global_engine(&self) -> Option<EngineDesc> {
        unsafe {
            from_glib_full(ibus_sys::ibus_bus_get_global_engine(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_global_engine_async() }
    //}

    //fn get_ibus_property(&self, property_name: &str) -> /*Ignored*/Option<glib::Variant> {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_ibus_property() }
    //}

    //fn get_ibus_property_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, property_name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_ibus_property_async() }
    //}

    fn get_name_owner(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_full(ibus_sys::ibus_bus_get_name_owner(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    //fn get_name_owner_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_name_owner_async() }
    //}

    fn get_service_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_get_service_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_global_engine(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_get_use_global_engine(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_use_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_use_global_engine_async() }
    //}

    fn get_use_sys_layout(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_get_use_sys_layout(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_use_sys_layout_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_get_use_sys_layout_async() }
    //}

    fn hello(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_bus_hello(self.as_ref().to_glib_none().0))
        }
    }

    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_is_connected(self.as_ref().to_glib_none().0))
        }
    }

    fn is_global_engine_enabled(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_is_global_engine_enabled(self.as_ref().to_glib_none().0))
        }
    }

    //fn is_global_engine_enabled_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_is_global_engine_enabled_async() }
    //}

    fn list_active_engines(&self) -> Vec<EngineDesc> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_bus_list_active_engines(self.as_ref().to_glib_none().0))
        }
    }

    //fn list_active_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_list_active_engines_async() }
    //}

    fn list_engines(&self) -> Vec<EngineDesc> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_bus_list_engines(self.as_ref().to_glib_none().0))
        }
    }

    //fn list_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_list_engines_async() }
    //}

    fn list_names(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_bus_list_names(self.as_ref().to_glib_none().0))
        }
    }

    fn list_queued_owners(&self, name: &str) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_bus_list_queued_owners(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn name_has_owner(&self, name: &str) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_name_has_owner(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    //fn name_has_owner_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_name_has_owner_async() }
    //}

    fn preload_engines(&self, names: &[&str]) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_preload_engines(self.as_ref().to_glib_none().0, names.to_glib_none().0))
        }
    }

    //fn preload_engines_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, names: &[&str], timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_preload_engines_async() }
    //}

    fn register_component<P: IsA<Component>>(&self, component: &P) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_register_component(self.as_ref().to_glib_none().0, component.as_ref().to_glib_none().0))
        }
    }

    //fn register_component_async<P: IsA<Component>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(), glib::Error>) + 'static>(&self, component: &P, timeout_msec: i32, cancellable: Option<&Q>, callback: R) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_register_component_async() }
    //}

    fn release_name(&self, name: &str) -> u32 {
        unsafe {
            ibus_sys::ibus_bus_release_name(self.as_ref().to_glib_none().0, name.to_glib_none().0)
        }
    }

    //fn release_name_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_release_name_async() }
    //}

    fn remove_match(&self, rule: &str) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_remove_match(self.as_ref().to_glib_none().0, rule.to_glib_none().0))
        }
    }

    //fn remove_match_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, rule: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_remove_match_async() }
    //}

    fn request_name(&self, name: &str, flags: u32) -> u32 {
        unsafe {
            ibus_sys::ibus_bus_request_name(self.as_ref().to_glib_none().0, name.to_glib_none().0, flags)
        }
    }

    //fn request_name_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, name: &str, flags: u32, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_request_name_async() }
    //}

    fn set_global_engine(&self, global_engine: &str) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_bus_set_global_engine(self.as_ref().to_glib_none().0, global_engine.to_glib_none().0))
        }
    }

    //fn set_global_engine_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, global_engine: &str, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_set_global_engine_async() }
    //}

    //fn set_ibus_property(&self, property_name: &str, value: /*Ignored*/&glib::Variant) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_set_ibus_property() }
    //}

    //fn set_ibus_property_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, property_name: &str, value: /*Ignored*/&glib::Variant, timeout_msec: i32, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ibus_sys:ibus_bus_set_ibus_property_async() }
    //}

    fn set_watch_dbus_signal(&self, watch: bool) {
        unsafe {
            ibus_sys::ibus_bus_set_watch_dbus_signal(self.as_ref().to_glib_none().0, watch.to_glib());
        }
    }

    fn set_watch_ibus_signal(&self, watch: bool) {
        unsafe {
            ibus_sys::ibus_bus_set_watch_ibus_signal(self.as_ref().to_glib_none().0, watch.to_glib());
        }
    }

    fn get_property_client_only(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"client-only\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `client-only` getter").unwrap()
        }
    }

    fn get_property_connect_async(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"connect-async\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `connect-async` getter").unwrap()
        }
    }

    fn connect_connected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn connected_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusBus, f: glib_sys::gpointer)
            where P: IsA<Bus>
        {
            let f: &F = &*(f as *const F);
            f(&Bus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"connected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(connected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusBus, f: glib_sys::gpointer)
            where P: IsA<Bus>
        {
            let f: &F = &*(f as *const F);
            f(&Bus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(disconnected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_global_engine_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn global_engine_changed_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut ibus_sys::IBusBus, name: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<Bus>
        {
            let f: &F = &*(f as *const F);
            f(&Bus::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(name))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"global-engine-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(global_engine_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_name_owner_changed<F: Fn(&Self, &str, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn name_owner_changed_trampoline<P, F: Fn(&P, &str, &str, &str) + 'static>(this: *mut ibus_sys::IBusBus, name: *mut libc::c_char, old_owner: *mut libc::c_char, new_owner: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<Bus>
        {
            let f: &F = &*(f as *const F);
            f(&Bus::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(name), &GString::from_glib_borrow(old_owner), &GString::from_glib_borrow(new_owner))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"name-owner-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(name_owner_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}
