
use cirrus_sys::*;
use crate::*;

use std::marker::PhantomData;

pub struct Context(MlirContext);
impl IntoRaw<MlirContext> for Context { 
    fn raw(&self) -> MlirContext { self.0 }
}
impl Drop for Context { 
    fn drop(&mut self) { 
        unsafe { mlirContextDestroy(self.0) }
    }
}
impl Context { 
    pub fn new() -> Self { 
        Self(unsafe { mlirContextCreate() })
    }
}

pub struct Module(MlirModule);
impl IntoRaw<MlirModule> for Module {
    fn raw(&self) -> MlirModule {
        self.0
    }
}
impl Module {
    pub fn parse(ctx: &Context, s: &str) -> Self {
        let sr = StringRef::from_str(s);
        unsafe {
            Self(mlirModuleCreateParse(ctx.raw(), sr.raw()))
        }
    }
}



pub struct StringRef<'a>(MlirStringRef, PhantomData<&'a MlirStringRef>);
impl <'a> IntoRaw<MlirStringRef> for StringRef<'a> { 
    fn raw(&self) -> MlirStringRef {
        self.0
    }
}
impl <'a> FromRaw<MlirStringRef> for StringRef<'a> {
    fn from_raw(raw: MlirStringRef) -> Option<Self> {
        if raw.data.is_null() { return None; }
        Some(Self(raw, PhantomData))
    }
}

impl <'a> StringRef<'a> {
    pub fn as_bytes(&'a self) -> &'a [u8] {
        unsafe { 
            std::slice::from_raw_parts(self.0.data as _, self.0.length)
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        StringRef(
            MlirStringRef { 
                data: s.as_ptr() as _,
                length: s.len(),
            }, 
            PhantomData
        )
    }

    pub fn as_str(&'a self) -> &'a str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}



