
use cirrus_sys::*;

use crate::*;

use std::marker::PhantomData;

decl_wrapper!(Context, MlirContext);
impl IntoOwned for Context {
    fn destroy(&mut self) { 
        unsafe { mlirContextDestroy(self.raw()) }
    }
}
impl Context { 
    pub fn new() -> Owned<Self> { 
        Owned(Self(unsafe { mlirContextCreate() }))
    }
    pub fn load_dialect(&self, handle: &DialectHandle) {
        unsafe {
            mlirDialectHandleRegisterDialect(handle.raw(), self.raw());

            // FIXME: Do we need to return an [MlirDialect]? 
            mlirDialectHandleLoadDialect(handle.raw(), self.raw());
        }
    }
}

decl_wrapper!(Module, MlirModule);
impl IntoOwned for Module { 
    fn destroy(&mut self) {
        unsafe { mlirModuleDestroy(self.raw()) }
    }
}
impl Module {
    pub fn parse(ctx: &Context, s: &str) -> Owned<Self> {
        let sr = StringRef::from_str(s);
        unsafe {
            Owned(Self(mlirModuleCreateParse(ctx.raw(), sr.raw())))
        }
    }

    pub fn body(&self) -> Block {
        Block::try_from_raw(
            unsafe { mlirModuleGetBody(self.raw()) }
        ).unwrap()
    }
    pub fn op(&self) -> Option<Operation> {
        Operation::try_from_raw(
            unsafe { cirrus_sys::mlirModuleGetOperation(self.raw()) }
        )
    }
}

decl_wrapper!(DialectHandle, MlirDialectHandle);
impl DialectHandle {
    pub fn firrtl() -> Self { 
        Self(unsafe { mlirGetDialectHandle__firrtl__() })
    }
}

decl_wrapper!(Operation, MlirOperation);
impl Operation {
    pub fn next(&self) -> Option<Self> {
        Self::try_from_raw(
            unsafe { mlirOperationGetNextInBlock(self.raw()) }
        )
    }
    pub fn first_region(&self) -> Option<Region> {
        Region::try_from_raw(
            unsafe { mlirOperationGetFirstRegion(self.raw()) }
        )
    }
}

decl_wrapper!(Identifier, MlirIdentifier);
decl_wrapper!(Region, MlirRegion);
impl Region {
    pub fn first_block(&self) -> Option<Block> {
        Block::try_from_raw(
            unsafe { mlirRegionGetFirstBlock(self.raw()) }
        )
    }
    pub fn next(&self) -> Option<Self> {
        Self::try_from_raw(
            unsafe { mlirRegionGetNextInOperation(self.raw()) }
        )
    }
}

decl_wrapper!(Block, MlirBlock);
impl Block {
    pub fn first_op(&self) -> Operation {
        Operation::try_from_raw(
            unsafe { 
                mlirBlockGetFirstOperation(self.raw())
            }
        ).unwrap()
    }
}

pub struct StringRef<'a>(MlirStringRef, PhantomData<&'a MlirStringRef>);
impl <'a> Wrapper for StringRef<'a> {
    type Inner = MlirStringRef;
    fn raw(&self) -> Self::Inner { self.0 }
    fn raw_ref(&self) -> &Self::Inner { &self.0 }
    fn raw_mut(&mut self) -> &mut Self::Inner { &mut self.0 }
    fn try_from_raw(raw: Self::Inner) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw, PhantomData))
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
            MlirStringRef { data: s.as_ptr() as _, length: s.len(), }, 
            PhantomData
        )
    }
    pub fn as_str(&'a self) -> &'a str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}



