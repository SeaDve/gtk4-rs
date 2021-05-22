// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::EventController;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventControllerFocus(Object<ffi::GtkEventControllerFocus, ffi::GtkEventControllerFocusClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_focus_get_type(),
    }
}

impl EventControllerFocus {
    #[doc(alias = "gtk_event_controller_focus_new")]
    pub fn new() -> EventControllerFocus {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_focus_new()).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_event_controller_focus_contains_focus")]
    pub fn contains_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_contains_focus(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_focus_is_focus")]
    pub fn is_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_is_focus(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "enter")]
    pub fn connect_enter<F: Fn(&EventControllerFocus) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "leave")]
    pub fn connect_leave<F: Fn(&EventControllerFocus) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "contains-focus")]
    pub fn connect_contains_focus_notify<F: Fn(&EventControllerFocus) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_focus_trampoline<
            F: Fn(&EventControllerFocus) + 'static,
        >(
            this: *mut ffi::GtkEventControllerFocus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::contains-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_contains_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-focus")]
    pub fn connect_is_focus_notify<F: Fn(&EventControllerFocus) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_focus_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerFocus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`EventControllerFocus`].
pub struct EventControllerFocusBuilder {
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl EventControllerFocusBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`EventControllerFocusBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerFocus`].
    pub fn build(self) -> EventControllerFocus {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        glib::Object::new::<EventControllerFocus>(&properties)
            .expect("Failed to create an instance of EventControllerFocus")
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for EventControllerFocus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerFocus")
    }
}
