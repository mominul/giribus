// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gobject;
use ibus_sys;
use std::fmt;
use Object;

glib_wrapper! {
    pub struct Serializable(Object<ibus_sys::IBusSerializable, ibus_sys::IBusSerializableClass, SerializableClass>) @extends Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_serializable_get_type(),
    }
}

impl Serializable {
    pub fn new() -> Serializable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_serializable_new())
        }
    }

    //pub fn deserialize_object(variant: /*Ignored*/&glib::Variant) -> Option<Serializable> {
    //    unsafe { TODO: call ibus_sys:ibus_serializable_deserialize_object() }
    //}
}

impl Default for Serializable {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SERIALIZABLE: Option<&Serializable> = None;

pub trait SerializableExt: 'static {
    fn copy(&self) -> Option<Serializable>;

    //fn get_qattachment(&self, key: /*Ignored*/glib::Quark) -> /*Ignored*/Option<glib::Variant>;

    //fn remove_qattachment(&self, key: /*Ignored*/glib::Quark);

    //fn serialize_object(&self) -> /*Ignored*/Option<glib::Variant>;

    //fn set_qattachment(&self, key: /*Ignored*/glib::Quark, value: /*Ignored*/&glib::Variant);
}

impl<O: IsA<Serializable>> SerializableExt for O {
    fn copy(&self) -> Option<Serializable> {
        unsafe {
            from_glib_none(ibus_sys::ibus_serializable_copy(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_qattachment(&self, key: /*Ignored*/glib::Quark) -> /*Ignored*/Option<glib::Variant> {
    //    unsafe { TODO: call ibus_sys:ibus_serializable_get_qattachment() }
    //}

    //fn remove_qattachment(&self, key: /*Ignored*/glib::Quark) {
    //    unsafe { TODO: call ibus_sys:ibus_serializable_remove_qattachment() }
    //}

    //fn serialize_object(&self) -> /*Ignored*/Option<glib::Variant> {
    //    unsafe { TODO: call ibus_sys:ibus_serializable_serialize_object() }
    //}

    //fn set_qattachment(&self, key: /*Ignored*/glib::Quark, value: /*Ignored*/&glib::Variant) {
    //    unsafe { TODO: call ibus_sys:ibus_serializable_set_qattachment() }
    //}
}

impl fmt::Display for Serializable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Serializable")
    }
}