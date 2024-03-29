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
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Object;
use Serializable;

glib_wrapper! {
    pub struct EmojiData(Object<ibus_sys::IBusEmojiData, ibus_sys::IBusEmojiDataClass, EmojiDataClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_emoji_data_get_type(),
    }
}

impl EmojiData {
    //pub fn new(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> EmojiData {
    //    unsafe { TODO: call ibus_sys:ibus_emoji_data_new() }
    //}

    pub fn load(path: &str) -> Vec<EmojiData> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ibus_sys::ibus_emoji_data_load(path.to_glib_none().0))
        }
    }

    pub fn save(path: &str, list: &[EmojiData]) {
        assert_initialized_main_thread!();
        unsafe {
            ibus_sys::ibus_emoji_data_save(path.to_glib_none().0, list.to_glib_none().0);
        }
    }
}

pub const NONE_EMOJI_DATA: Option<&EmojiData> = None;

pub trait EmojiDataExt: 'static {
    fn get_annotations(&self) -> Vec<GString>;

    fn get_category(&self) -> Option<GString>;

    fn get_description(&self) -> Option<GString>;

    fn get_emoji(&self) -> Option<GString>;

    fn set_annotations(&self, annotations: &[&str]);

    fn set_description(&self, description: &str);

    fn connect_property_annotations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EmojiData>> EmojiDataExt for O {
    fn get_annotations(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ibus_sys::ibus_emoji_data_get_annotations(self.as_ref().to_glib_none().0))
        }
    }

    fn get_category(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_emoji_data_get_category(self.as_ref().to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_emoji_data_get_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_emoji(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_emoji_data_get_emoji(self.as_ref().to_glib_none().0))
        }
    }

    fn set_annotations(&self, annotations: &[&str]) {
        unsafe {
            ibus_sys::ibus_emoji_data_set_annotations(self.as_ref().to_glib_none().0, annotations.to_glib_full());
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            ibus_sys::ibus_emoji_data_set_description(self.as_ref().to_glib_none().0, description.to_glib_none().0);
        }
    }

    fn connect_property_annotations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_annotations_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEmojiData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EmojiData>
        {
            let f: &F = &*(f as *const F);
            f(&EmojiData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::annotations\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_annotations_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEmojiData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EmojiData>
        {
            let f: &F = &*(f as *const F);
            f(&EmojiData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_description_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for EmojiData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmojiData")
    }
}
