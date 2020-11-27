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
use Text;

glib_wrapper! {
    pub struct LookupTable(Object<ibus_sys::IBusLookupTable, ibus_sys::IBusLookupTableClass, LookupTableClass>) @extends Serializable, Object, gobject::InitiallyUnowned;

    match fn {
        get_type => || ibus_sys::ibus_lookup_table_get_type(),
    }
}

impl LookupTable {
    pub fn new(page_size: u32, cursor_pos: u32, cursor_visible: bool, round: bool) -> LookupTable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ibus_sys::ibus_lookup_table_new(page_size, cursor_pos, cursor_visible.to_glib(), round.to_glib()))
        }
    }
}

pub const NONE_LOOKUP_TABLE: Option<&LookupTable> = None;

pub trait LookupTableExt: 'static {
    fn append_candidate<P: IsA<Text>>(&self, text: &P);

    fn append_label<P: IsA<Text>>(&self, text: &P);

    fn clear(&self);

    fn cursor_down(&self) -> bool;

    fn cursor_up(&self) -> bool;

    fn get_candidate(&self, index: u32) -> Option<Text>;

    fn get_cursor_in_page(&self) -> u32;

    fn get_cursor_pos(&self) -> u32;

    fn get_label(&self, index: u32) -> Option<Text>;

    fn get_number_of_candidates(&self) -> u32;

    fn get_orientation(&self) -> i32;

    fn get_page_size(&self) -> u32;

    fn is_cursor_visible(&self) -> bool;

    fn is_round(&self) -> bool;

    fn page_down(&self) -> bool;

    fn page_up(&self) -> bool;

    fn set_cursor_pos(&self, cursor_pos: u32);

    fn set_cursor_visible(&self, visible: bool);

    fn set_label<P: IsA<Text>>(&self, index: u32, text: &P);

    fn set_orientation(&self, orientation: i32);

    fn set_page_size(&self, page_size: u32);

    fn set_round(&self, round: bool);
}

impl<O: IsA<LookupTable>> LookupTableExt for O {
    fn append_candidate<P: IsA<Text>>(&self, text: &P) {
        unsafe {
            ibus_sys::ibus_lookup_table_append_candidate(self.as_ref().to_glib_none().0, text.as_ref().to_glib_none().0);
        }
    }

    fn append_label<P: IsA<Text>>(&self, text: &P) {
        unsafe {
            ibus_sys::ibus_lookup_table_append_label(self.as_ref().to_glib_none().0, text.as_ref().to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            ibus_sys::ibus_lookup_table_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn cursor_down(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_cursor_down(self.as_ref().to_glib_none().0))
        }
    }

    fn cursor_up(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_cursor_up(self.as_ref().to_glib_none().0))
        }
    }

    fn get_candidate(&self, index: u32) -> Option<Text> {
        unsafe {
            from_glib_none(ibus_sys::ibus_lookup_table_get_candidate(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_cursor_in_page(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_lookup_table_get_cursor_in_page(self.as_ref().to_glib_none().0)
        }
    }

    fn get_cursor_pos(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_lookup_table_get_cursor_pos(self.as_ref().to_glib_none().0)
        }
    }

    fn get_label(&self, index: u32) -> Option<Text> {
        unsafe {
            from_glib_none(ibus_sys::ibus_lookup_table_get_label(self.as_ref().to_glib_none().0, index))
        }
    }

    fn get_number_of_candidates(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_lookup_table_get_number_of_candidates(self.as_ref().to_glib_none().0)
        }
    }

    fn get_orientation(&self) -> i32 {
        unsafe {
            ibus_sys::ibus_lookup_table_get_orientation(self.as_ref().to_glib_none().0)
        }
    }

    fn get_page_size(&self) -> u32 {
        unsafe {
            ibus_sys::ibus_lookup_table_get_page_size(self.as_ref().to_glib_none().0)
        }
    }

    fn is_cursor_visible(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_is_cursor_visible(self.as_ref().to_glib_none().0))
        }
    }

    fn is_round(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_is_round(self.as_ref().to_glib_none().0))
        }
    }

    fn page_down(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_page_down(self.as_ref().to_glib_none().0))
        }
    }

    fn page_up(&self) -> bool {
        unsafe {
            from_glib(ibus_sys::ibus_lookup_table_page_up(self.as_ref().to_glib_none().0))
        }
    }

    fn set_cursor_pos(&self, cursor_pos: u32) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_cursor_pos(self.as_ref().to_glib_none().0, cursor_pos);
        }
    }

    fn set_cursor_visible(&self, visible: bool) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_cursor_visible(self.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn set_label<P: IsA<Text>>(&self, index: u32, text: &P) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_label(self.as_ref().to_glib_none().0, index, text.as_ref().to_glib_none().0);
        }
    }

    fn set_orientation(&self, orientation: i32) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_orientation(self.as_ref().to_glib_none().0, orientation);
        }
    }

    fn set_page_size(&self, page_size: u32) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_page_size(self.as_ref().to_glib_none().0, page_size);
        }
    }

    fn set_round(&self, round: bool) {
        unsafe {
            ibus_sys::ibus_lookup_table_set_round(self.as_ref().to_glib_none().0, round.to_glib());
        }
    }
}

impl fmt::Display for LookupTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LookupTable")
    }
}
