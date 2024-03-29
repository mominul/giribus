// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use ibus_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use LookupTable;
use Text;

glib_wrapper! {
    pub struct Engine(Object<ibus_sys::IBusEngine, ibus_sys::IBusEngineClass, EngineClass>);

    match fn {
        get_type => || ibus_sys::ibus_engine_get_type(),
    }
}

impl Engine {
    //pub fn new(engine_name: &str, object_path: &str, connection: /*Ignored*/&gio::DBusConnection) -> Engine {
    //    unsafe { TODO: call ibus_sys:ibus_engine_new() }
    //}

    //pub fn with_type(engine_type: glib::types::Type, engine_name: &str, object_path: &str, connection: /*Ignored*/&gio::DBusConnection) -> Engine {
    //    unsafe { TODO: call ibus_sys:ibus_engine_new_with_type() }
    //}
}

pub const NONE_ENGINE: Option<&Engine> = None;

pub trait EngineExt: 'static {
    fn commit_text<P: IsA<Text>>(&self, text: &P);

    fn delete_surrounding_text(&self, offset: i32, nchars: u32);

    fn forward_key_event(&self, keyval: u32, keycode: u32, state: u32);

    fn get_content_type(&self) -> (u32, u32);

    fn get_name(&self) -> Option<GString>;

    fn get_surrounding_text(&self) -> (Text, u32, u32);

    fn hide_auxiliary_text(&self);

    fn hide_lookup_table(&self);

    fn hide_preedit_text(&self);

    //fn register_properties(&self, prop_list: /*Ignored*/&PropList);

    fn show_auxiliary_text(&self);

    fn show_lookup_table(&self);

    fn show_preedit_text(&self);

    fn update_auxiliary_text<P: IsA<Text>>(&self, text: &P, visible: bool);

    fn update_lookup_table<P: IsA<LookupTable>>(&self, lookup_table: &P, visible: bool);

    fn update_lookup_table_fast<P: IsA<LookupTable>>(&self, lookup_table: &P, visible: bool);

    fn update_preedit_text<P: IsA<Text>>(&self, text: &P, cursor_pos: u32, visible: bool);

    //fn update_preedit_text_with_mode<P: IsA<Text>>(&self, text: &P, cursor_pos: u32, visible: bool, mode: /*Ignored*/PreeditFocusMode);

    //fn update_property(&self, prop: /*Ignored*/&Property);

    fn get_property_engine_name(&self) -> Option<GString>;

    fn connect_cancel_hand_writing<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_candidate_clicked<F: Fn(&Self, u32, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cursor_down<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cursor_up<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_disable<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_enable<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_focus_in<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_focus_out<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_down<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_up<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_process_hand_writing_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_process_key_event<F: Fn(&Self, u32, u32, u32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_activate<F: Fn(&Self, &str, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hide<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_reset<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_set_capabilities<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_set_content_type<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_set_cursor_location<F: Fn(&Self, i32, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_set_surrounding_text<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Engine>> EngineExt for O {
    fn commit_text<P: IsA<Text>>(&self, text: &P) {
        unsafe {
            ibus_sys::ibus_engine_commit_text(self.as_ref().to_glib_none().0, text.as_ref().to_glib_none().0);
        }
    }

    fn delete_surrounding_text(&self, offset: i32, nchars: u32) {
        unsafe {
            ibus_sys::ibus_engine_delete_surrounding_text(self.as_ref().to_glib_none().0, offset, nchars);
        }
    }

    fn forward_key_event(&self, keyval: u32, keycode: u32, state: u32) {
        unsafe {
            ibus_sys::ibus_engine_forward_key_event(self.as_ref().to_glib_none().0, keyval, keycode, state);
        }
    }

    fn get_content_type(&self) -> (u32, u32) {
        unsafe {
            let mut purpose = mem::MaybeUninit::uninit();
            let mut hints = mem::MaybeUninit::uninit();
            ibus_sys::ibus_engine_get_content_type(self.as_ref().to_glib_none().0, purpose.as_mut_ptr(), hints.as_mut_ptr());
            let purpose = purpose.assume_init();
            let hints = hints.assume_init();
            (purpose, hints)
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ibus_sys::ibus_engine_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_surrounding_text(&self) -> (Text, u32, u32) {
        unsafe {
            let mut text = ptr::null_mut();
            let mut cursor_pos = mem::MaybeUninit::uninit();
            let mut anchor_pos = mem::MaybeUninit::uninit();
            ibus_sys::ibus_engine_get_surrounding_text(self.as_ref().to_glib_none().0, &mut text, cursor_pos.as_mut_ptr(), anchor_pos.as_mut_ptr());
            let cursor_pos = cursor_pos.assume_init();
            let anchor_pos = anchor_pos.assume_init();
            (from_glib_none(text), cursor_pos, anchor_pos)
        }
    }

    fn hide_auxiliary_text(&self) {
        unsafe {
            ibus_sys::ibus_engine_hide_auxiliary_text(self.as_ref().to_glib_none().0);
        }
    }

    fn hide_lookup_table(&self) {
        unsafe {
            ibus_sys::ibus_engine_hide_lookup_table(self.as_ref().to_glib_none().0);
        }
    }

    fn hide_preedit_text(&self) {
        unsafe {
            ibus_sys::ibus_engine_hide_preedit_text(self.as_ref().to_glib_none().0);
        }
    }

    //fn register_properties(&self, prop_list: /*Ignored*/&PropList) {
    //    unsafe { TODO: call ibus_sys:ibus_engine_register_properties() }
    //}

    fn show_auxiliary_text(&self) {
        unsafe {
            ibus_sys::ibus_engine_show_auxiliary_text(self.as_ref().to_glib_none().0);
        }
    }

    fn show_lookup_table(&self) {
        unsafe {
            ibus_sys::ibus_engine_show_lookup_table(self.as_ref().to_glib_none().0);
        }
    }

    fn show_preedit_text(&self) {
        unsafe {
            ibus_sys::ibus_engine_show_preedit_text(self.as_ref().to_glib_none().0);
        }
    }

    fn update_auxiliary_text<P: IsA<Text>>(&self, text: &P, visible: bool) {
        unsafe {
            ibus_sys::ibus_engine_update_auxiliary_text(self.as_ref().to_glib_none().0, text.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn update_lookup_table<P: IsA<LookupTable>>(&self, lookup_table: &P, visible: bool) {
        unsafe {
            ibus_sys::ibus_engine_update_lookup_table(self.as_ref().to_glib_none().0, lookup_table.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn update_lookup_table_fast<P: IsA<LookupTable>>(&self, lookup_table: &P, visible: bool) {
        unsafe {
            ibus_sys::ibus_engine_update_lookup_table_fast(self.as_ref().to_glib_none().0, lookup_table.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn update_preedit_text<P: IsA<Text>>(&self, text: &P, cursor_pos: u32, visible: bool) {
        unsafe {
            ibus_sys::ibus_engine_update_preedit_text(self.as_ref().to_glib_none().0, text.as_ref().to_glib_none().0, cursor_pos, visible.to_glib());
        }
    }

    //fn update_preedit_text_with_mode<P: IsA<Text>>(&self, text: &P, cursor_pos: u32, visible: bool, mode: /*Ignored*/PreeditFocusMode) {
    //    unsafe { TODO: call ibus_sys:ibus_engine_update_preedit_text_with_mode() }
    //}

    //fn update_property(&self, prop: /*Ignored*/&Property) {
    //    unsafe { TODO: call ibus_sys:ibus_engine_update_property() }
    //}

    fn get_property_engine_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"engine-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `engine-name` getter")
        }
    }

    fn connect_cancel_hand_writing<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_hand_writing_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut ibus_sys::IBusEngine, n_strokes: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), n_strokes)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cancel-hand-writing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(cancel_hand_writing_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_candidate_clicked<F: Fn(&Self, u32, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn candidate_clicked_trampoline<P, F: Fn(&P, u32, u32, u32) + 'static>(this: *mut ibus_sys::IBusEngine, index: libc::c_uint, button: libc::c_uint, state: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), index, button, state)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"candidate-clicked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(candidate_clicked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_cursor_down<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cursor_down_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cursor-down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(cursor_down_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_cursor_up<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cursor_up_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cursor-up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(cursor_up_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_disable<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disable_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"disable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(disable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_enable<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enable_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"enable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(enable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_focus_in<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn focus_in_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"focus-in\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(focus_in_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_focus_out<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn focus_out_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"focus-out\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(focus_out_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_page_down<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_down_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-down\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(page_down_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_page_up<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_up_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-up\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(page_up_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //fn connect_process_hand_writing_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented coordinates: *.Pointer
    //}

    fn connect_process_key_event<F: Fn(&Self, u32, u32, u32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn process_key_event_trampoline<P, F: Fn(&P, u32, u32, u32) -> bool + 'static>(this: *mut ibus_sys::IBusEngine, keyval: libc::c_uint, keycode: libc::c_uint, state: libc::c_uint, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), keyval, keycode, state).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"process-key-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(process_key_event_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_activate<F: Fn(&Self, &str, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn property_activate_trampoline<P, F: Fn(&P, &str, u32) + 'static>(this: *mut ibus_sys::IBusEngine, name: *mut libc::c_char, state: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(name), state)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"property-activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(property_activate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_hide<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn property_hide_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut ibus_sys::IBusEngine, name: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(name))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"property-hide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(property_hide_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_show<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn property_show_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut ibus_sys::IBusEngine, name: *mut libc::c_char, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(name))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"property-show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(property_show_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_reset<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ibus_sys::IBusEngine, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"reset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(reset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_set_capabilities<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn set_capabilities_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut ibus_sys::IBusEngine, caps: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), caps)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"set-capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(set_capabilities_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_set_content_type<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn set_content_type_trampoline<P, F: Fn(&P, u32, u32) + 'static>(this: *mut ibus_sys::IBusEngine, purpose: libc::c_uint, hints: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), purpose, hints)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"set-content-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(set_content_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_set_cursor_location<F: Fn(&Self, i32, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn set_cursor_location_trampoline<P, F: Fn(&P, i32, i32, i32, i32) + 'static>(this: *mut ibus_sys::IBusEngine, x: libc::c_int, y: libc::c_int, w: libc::c_int, h: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<Engine>
        {
            let f: &F = &*(f as *const F);
            f(&Engine::from_glib_borrow(this).unsafe_cast_ref(), x, y, w, h)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"set-cursor-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(set_cursor_location_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //fn connect_set_surrounding_text<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored text: GObject.Object
    //}
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Engine")
    }
}
