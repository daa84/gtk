// This file was generated by gir (e8e8262) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
#[cfg(gtk_3_10)]
use StackTransitionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Stack(Object<ffi::GtkStack>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[cfg(gtk_3_10)]
    pub fn new() -> Stack {
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_10)]
    pub fn add_named<T: Upcast<Widget>>(&self, child: &T, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn add_titled<T: Upcast<Widget>>(&self, child: &T, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_16)]
    pub fn get_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_homogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_transition_type(&self) -> StackTransitionType {
        unsafe {
            ffi::gtk_stack_get_transition_type(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_16)]
    pub fn get_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition);
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_visible_child<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(self.to_glib_none().0, name.to_glib_none().0, transition);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

}
