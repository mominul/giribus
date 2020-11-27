// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject;
use gobject_sys;
use ibus_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Object;
use Serializable;

glib_wrapper! {
    pub struct UnicodeBlock(Object<ibus_sys::IBusUnicodeBlock, ibus_sys::IBusUnicodeBlockClass, UnicodeBlockClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_unicode_block_get_type(),
    }
}

impl UnicodeBlock {
    //pub fn new(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> UnicodeBlock {
    //    unsafe { TODO: call ibus_sys:ibus_unicode_block_new() }
    //}

    pub fn load(path: &str) -> Vec<UnicodeBlock> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_unicode_block_load(path.to_glib_none().0))
        }
    }

    pub fn save(path: &str, list: &[UnicodeBlock]) {
        assert_initialized_main_thread!();
        unsafe {
            ibus_sys::ibus_unicode_block_save(path.to_glib_none().0, list.to_glib_none().0);
        }
    }
}

pub const NONE_UNICODE_BLOCK: Option<&UnicodeBlock> = None;

pub trait UnicodeBlockExt: 'static {
    fn get_end(&self) -> char;

    fn get_name(&self) -> Option<GString>;

    fn get_start(&self) -> char;

    fn set_property_name(&self, name: Option<&str>);

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UnicodeBlock>> UnicodeBlockExt for O {
    fn get_end(&self) -> char {
        unsafe {
            from_glib(ibus_sys::ibus_unicode_block_get_end(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_unicode_block_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_start(&self) -> char {
        unsafe {
            from_glib(ibus_sys::ibus_unicode_block_get_start(self.as_ref().to_glib_none().0))
        }
    }

    fn set_property_name(&self, name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, Value::from(name).to_glib_none().0);
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusUnicodeBlock, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<UnicodeBlock>
        {
            let f: &F = &*(f as *const F);
            f(&UnicodeBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for UnicodeBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnicodeBlock")
    }
}