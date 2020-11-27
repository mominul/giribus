// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gobject;
use ibus_sys;
use std::fmt;
use Object;
use Serializable;
use XML;

glib_wrapper! {
    pub struct EngineDesc(Object<ibus_sys::IBusEngineDesc, ibus_sys::IBusEngineDescClass, EngineDescClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_engine_desc_get_type(),
    }
}

impl EngineDesc {
    pub fn new(name: &str, longname: &str, description: &str, language: &str, license: &str, author: &str, icon: &str, layout: &str) -> EngineDesc {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_new(name.to_glib_none().0, longname.to_glib_none().0, description.to_glib_none().0, language.to_glib_none().0, license.to_glib_none().0, author.to_glib_none().0, icon.to_glib_none().0, layout.to_glib_none().0))
        }
    }

    pub fn from_xml_node(node: &mut XML) -> EngineDesc {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_new_from_xml_node(node.to_glib_none_mut().0))
        }
    }

    //pub fn new_varargs(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> EngineDesc {
    //    unsafe { TODO: call ibus_sys:ibus_engine_desc_new_varargs() }
    //}
}

pub const NONE_ENGINE_DESC: Option<&EngineDesc> = None;

pub trait EngineDescExt: 'static {
    fn get_author(&self) -> Option<GString>;

    fn get_description(&self) -> Option<GString>;

    fn get_hotkeys(&self) -> Option<GString>;

    fn get_icon(&self) -> Option<GString>;

    fn get_icon_prop_key(&self) -> Option<GString>;

    fn get_language(&self) -> Option<GString>;

    fn get_layout(&self) -> Option<GString>;

    fn get_layout_option(&self) -> Option<GString>;

    fn get_layout_variant(&self) -> Option<GString>;

    fn get_license(&self) -> Option<GString>;

    fn get_longname(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_rank(&self) -> u32;

    fn get_setup(&self) -> Option<GString>;

    fn get_symbol(&self) -> Option<GString>;

    fn get_textdomain(&self) -> Option<GString>;

    fn get_version(&self) -> Option<GString>;

    //fn output(&self, output: /*Ignored*/&mut glib::String, indent: i32);
}

impl<O: IsA<EngineDesc>> EngineDescExt for O {
    fn get_author(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_author(self.as_ref().to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hotkeys(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_hotkeys(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon_prop_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_icon_prop_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_language(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_language(self.as_ref().to_glib_none().0))
        }
    }

    fn get_layout(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_layout(self.as_ref().to_glib_none().0))
        }
    }

    fn get_layout_option(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_layout_option(self.as_ref().to_glib_none().0))
        }
    }

    fn get_layout_variant(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_layout_variant(self.as_ref().to_glib_none().0))
        }
    }

    fn get_license(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_license(self.as_ref().to_glib_none().0))
        }
    }

    fn get_longname(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_longname(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rank(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_engine_desc_get_rank(self.as_ref().to_glib_none().0)
        }
    }

    fn get_setup(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_setup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_symbol(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_symbol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_textdomain(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_textdomain(self.as_ref().to_glib_none().0))
        }
    }

    fn get_version(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_desc_get_version(self.as_ref().to_glib_none().0))
        }
    }

    //fn output(&self, output: /*Ignored*/&mut glib::String, indent: i32) {
    //    unsafe { TODO: call ibus_sys:ibus_engine_desc_output() }
    //}
}

impl fmt::Display for EngineDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EngineDesc")
    }
}
