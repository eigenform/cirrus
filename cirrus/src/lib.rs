
pub mod mlir;

/// Implemented for objects that Rust is responsible for dropping
/// (ie. on the appropriate "raw" bindgen types we might create). 
pub trait Ownable {
    fn destroy(&mut self);
}

/// Wrapper newtype for objects that are "owned" by us. 
///
/// We need to make this distinction because some objects may *not* be
/// directly owned by us (ie. indirectly created/destroyed by CIRCT/LLVM).
pub struct Owned<T: Ownable>(T);

/// When we wrap an object in [Owned], we intend for Rust to handle dropping
/// it when it goes out-of-scope.
impl <T: Ownable> Drop for Owned<T> {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

/// Implemented on types that can be converted into some "raw" bindgen type.
pub trait IntoRaw<T> {
    fn raw(&self) -> T;
}

/// Implemented on types that can be created from some "raw" bindgen type.
pub trait FromRaw<T> where Self: Sized {
    fn from_raw(raw: T) -> Option<Self>;
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

    #[test]
    fn parse_mlir_module_firrtl() {
        let ctx = mlir::Context::new();

        unsafe { 
            let handle = cirrus_sys::mlirGetDialectHandle__firrtl__();
            cirrus_sys::mlirDialectHandleRegisterDialect(
                handle, ctx.raw()
            );
            cirrus_sys::mlirDialectHandleLoadDialect(
                handle, ctx.raw()
            );
        }

        let mut f = File::open("../MyAlu.fir.mlir").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let foo = mlir::Module::parse(&ctx, &s);

        unsafe {
            let op = cirrus_sys::mlirModuleGetOperation(foo.raw());
            cirrus_sys::mlirOperationDump(op);
        }
    }

}
