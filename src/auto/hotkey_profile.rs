// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gobject;
use ibus_sys;
use std::fmt;
use Object;
use Serializable;

glib_wrapper! {
    pub struct HotkeyProfile(Object<ibus_sys::IBusHotkeyProfile, ibus_sys::IBusHotkeyProfileClass, HotkeyProfileClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_hotkey_profile_get_type(),
    }
}

impl HotkeyProfile {
    pub fn new() -> HotkeyProfile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_hotkey_profile_new())
        }
    }
}

impl Default for HotkeyProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_HOTKEY_PROFILE: Option<&HotkeyProfile> = None;

pub trait HotkeyProfileExt: 'static {
    //fn add_hotkey(&self, keyval: u32, modifiers: u32, event: /*Ignored*/glib::Quark) -> bool;

    //fn add_hotkey_from_string(&self, str: &str, event: /*Ignored*/glib::Quark) -> bool;

    //fn filter_key_event(&self, keyval: u32, modifiers: u32, prev_keyval: u32, prev_modifiers: u32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/glib::Quark;

    //fn lookup_hotkey(&self, keyval: u32, modifiers: u32) -> /*Ignored*/glib::Quark;

    fn remove_hotkey(&self, keyval: u32, modifiers: u32) -> bool;

    //fn remove_hotkey_by_event(&self, event: /*Ignored*/glib::Quark) -> bool;

    //fn connect_trigger<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<HotkeyProfile>> HotkeyProfileExt for O {
    //fn add_hotkey(&self, keyval: u32, modifiers: u32, event: /*Ignored*/glib::Quark) -> bool {
    //    unsafe { TODO: call ibus_sys:ibus_hotkey_profile_add_hotkey() }
    //}

    //fn add_hotkey_from_string(&self, str: &str, event: /*Ignored*/glib::Quark) -> bool {
    //    unsafe { TODO: call ibus_sys:ibus_hotkey_profile_add_hotkey_from_string() }
    //}

    //fn filter_key_event(&self, keyval: u32, modifiers: u32, prev_keyval: u32, prev_modifiers: u32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/glib::Quark {
    //    unsafe { TODO: call ibus_sys:ibus_hotkey_profile_filter_key_event() }
    //}

    //fn lookup_hotkey(&self, keyval: u32, modifiers: u32) -> /*Ignored*/glib::Quark {
    //    unsafe { TODO: call ibus_sys:ibus_hotkey_profile_lookup_hotkey() }
    //}

    fn remove_hotkey(&self, keyval: u32, modifiers: u32) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_hotkey_profile_remove_hotkey(self.as_ref().to_glib_none().0, keyval, modifiers))
        }
    }

    //fn remove_hotkey_by_event(&self, event: /*Ignored*/glib::Quark) -> bool {
    //    unsafe { TODO: call ibus_sys:ibus_hotkey_profile_remove_hotkey_by_event() }
    //}

    //fn connect_trigger<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented user_data: *.Pointer
    //}
}

impl fmt::Display for HotkeyProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HotkeyProfile")
    }
}
