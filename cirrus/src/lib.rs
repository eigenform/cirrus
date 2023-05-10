
// Presumably we can start by replicating simple CAPI tests, see:
//  https://github.com/llvm/circt/blob/main/test/CAPI/ir.c
//  https://github.com/llvm/llvm-project/blob/main/mlir/examples/standalone/


use cirrus_sys::*;


pub struct Context(MlirContext);
impl Context { 
    pub fn new() -> Self {
        unsafe {
            Self(mlirContextCreate())
        }
    }
}
impl Drop for Context { 
    fn drop(&mut self) {
        unsafe { mlirContextDestroy(self.0) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_context() {
        let ctx = Context::new();
    }
}
