extern crate glib;
extern crate javascriptcore_sys;

use std::ptr;

use glib::translate::FromGlibPtr;
use javascriptcore_sys::{JSGlobalContextRef, JSValueRef, JSValueIsArray, JSValueIsDate, JSValueIsBoolean, JSValueIsNull, JSValueIsNumber, JSValueIsObject, JSValueIsString, JSValueIsUndefined, JSValueToBoolean, JSValueToNumber};

pub struct GlobalContext {
    raw: JSGlobalContextRef,
}

pub struct Value {
    raw: JSValueRef,
}

impl Value {
    pub fn is_boolean(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsBoolean(context.raw, self.raw) != 0 }
    }

    pub fn is_null(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsNull(context.raw, self.raw) != 0 }
    }

    pub fn is_undefined(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsUndefined(context.raw, self.raw) != 0 }
    }

    pub fn is_number(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsNumber(context.raw, self.raw) != 0 }
    }

    pub fn is_string(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsString(context.raw, self.raw) != 0 }
    }

    pub fn is_object(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsObject(context.raw, self.raw) != 0 }
    }

    pub fn is_array(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsArray(context.raw, self.raw) != 0 }
    }

    pub fn is_date(&self, context: &GlobalContext) -> bool {
        unsafe { JSValueIsDate(context.raw, self.raw) != 0 }
    }

    pub fn to_number(&self, context: &GlobalContext) -> Option<f64> {
        let mut exception = ptr::null_mut();
        let result = unsafe { JSValueToNumber(context.raw, self.raw, &mut exception) };
        if exception.is_null() {
            Some(result)
        }
        else {
            None
        }
    }

    pub fn to_boolean(&self, context: &GlobalContext) -> bool {
        let value = unsafe { JSValueToBoolean(context.raw, self.raw) };
        value != 0
    }
}

impl FromGlibPtr<JSValueRef> for Value {
    unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
        Value {
            raw: ptr,
        }
    }

    unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
        Value {
            raw: ptr,
        }
    }
}

impl FromGlibPtr<JSGlobalContextRef> for GlobalContext {
    unsafe fn from_glib_none(ptr: JSValueRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }

    unsafe fn from_glib_full(ptr: JSValueRef) -> Self {
        GlobalContext {
            raw: ptr,
        }
    }
}
