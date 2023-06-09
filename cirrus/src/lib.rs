//! A library for interacting with CIRCT/LLVM MLIR. 
//!
//! Implementation Notes
//! ====================
//!

#[macro_use]
pub (crate) mod macros;

pub mod mlir;
pub mod firrtl; 
use cirrus_sys::*;

/// Implemented on types exposed by [cirrus_sys].
pub trait BindingType: Sized {
    fn is_null(&self) -> bool;
}

/// Implemented on types that wrap some inner [BindingType].
pub trait Wrapper: Sized {
    type Inner: BindingType; 
    fn raw(&self) -> Self::Inner;
    fn raw_ref(&self) -> &Self::Inner;
    fn raw_mut(&mut self) -> &mut Self::Inner;

    // FIXME: Might want to distinguish between this and cases where 
    // trying to wrap a null object is an *error* 
    fn try_from_raw(raw: Self::Inner) -> Option<Self>;
}

//impl <T: BindingType> AsRef<T> for dyn Wrapper<Inner=T> {
//    fn as_ref(&self) -> &<Self as Wrapper>::Inner {
//        self.raw_ref()
//    }
//}

/// Implemented for [Wrapper] objects that we may be responsible for dropping.
pub trait IntoOwned where Self: Wrapper + Sized {
    fn destroy(&mut self);
    fn into_owned(self) -> Owned<Self> {
        if self.raw().is_null() {
            panic!("Cannot take ownership over a null object?");
        }
        Owned(self)
    }
}

/// Wrapper for objects that are "owned" by us. 
///
/// We need to make this distinction because some objects may *not* be
/// directly owned by us (ie. indirectly created/destroyed by CIRCT/LLVM).
pub struct Owned<T: IntoOwned>(T);
impl <T: IntoOwned> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
impl <T: IntoOwned> Drop for Owned<T> {
    fn drop(&mut self) { 
        if self.raw().is_null() {
            panic!("We would have dropped a null object?");
        }
        self.0.destroy(); 
    }
}

impl_binding_type!(MlirAttribute, ptr);
impl_binding_type!(MlirBlock, ptr);
impl_binding_type!(MlirContext, ptr);
impl_binding_type!(MlirDialect, ptr);
impl_binding_type!(MlirDialectHandle, ptr);
impl_binding_type!(MlirIdentifier, ptr);
impl_binding_type!(MlirLocation, ptr);
impl_binding_type!(MlirModule, ptr);
impl_binding_type!(MlirOpOperand, ptr);
impl_binding_type!(MlirOpPassManager, ptr);
impl_binding_type!(MlirOperation, ptr);
impl_binding_type!(MlirPass, ptr);
impl_binding_type!(MlirPassManager, ptr);
impl_binding_type!(MlirRegion, ptr);
impl_binding_type!(MlirStringRef, data);
impl_binding_type!(MlirSymbolTable, ptr);
impl_binding_type!(MlirType, ptr);
impl_binding_type!(MlirTypeID, ptr);
impl_binding_type!(MlirValue, ptr);

pub enum FirrtlOpKind {
    Circuit,
    Module,
}
pub struct FirrtlOp {
    _op: mlir::Operation,
    kind: FirrtlOpKind,
}

// Presumably we can start by replicating simple CAPI tests, see:
//  https://github.com/llvm/circt/blob/main/test/CAPI/ir.c
//  https://github.com/llvm/llvm-project/blob/main/mlir/examples/standalone/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mlir;
    use cirrus_sys;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn create_and_drop_context() {
        let ctx = mlir::Context::new();
    }


}
