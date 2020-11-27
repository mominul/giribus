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
    pub struct Attribute(Object<ibus_sys::IBusAttribute, ibus_sys::IBusAttributeClass, AttributeClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_attribute_get_type(),
    }
}

impl Attribute {
    pub fn new(type_: u32, value: u32, start_index: u32, end_index: u32) -> Attribute {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_attribute_new(type_, value, start_index, end_index))
        }
    }
}

pub const NONE_ATTRIBUTE: Option<&Attribute> = None;

pub trait AttributeExt: 'static {
    fn get_attr_type(&self) -> u32;

    fn get_end_index(&self) -> u32;

    fn get_start_index(&self) -> u32;

    fn get_value(&self) -> u32;
}

impl<O: IsA<Attribute>> AttributeExt for O {
    fn get_attr_type(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_attribute_get_attr_type(self.as_ref().to_glib_none().0)
        }
    }

    fn get_end_index(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_attribute_get_end_index(self.as_ref().to_glib_none().0)
        }
    }

    fn get_start_index(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_attribute_get_start_index(self.as_ref().to_glib_none().0)
        }
    }

    fn get_value(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_attribute_get_value(self.as_ref().to_glib_none().0)
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute")
    }
}
