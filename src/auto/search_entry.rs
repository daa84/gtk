// This file was generated by gir (e8e8262) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Editable;
use Entry;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>): Widget, Entry, Buildable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(gtk_3_6)]
    pub fn new() -> SearchEntry {
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }

    //#[cfg(gtk_3_16)]
    //pub fn handle_event(&self, event: /*Unknown conversion*/Unknown rust type: "Event") -> bool {
    //    unsafe { TODO: call ffi::gtk_search_entry_handle_event() }
    //}

}
