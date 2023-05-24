
pub struct FirrtlModuleOp;
pub enum FirrtlOp {
    Module(FirrtlModuleOp),
}

#[cfg(test)]
mod test {
    use crate::*;
    use crate::mlir;
    use cirrus_sys;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn parse_mlir_module_firrtl() -> Result<(), &'static str> {
        let ctx = mlir::Context::new();
        ctx.load_dialect(&mlir::DialectHandle::firrtl());

        //let mut f = File::open("../MyAlu.fir.mlir").unwrap();
        let mut f = File::open("../MyAlu.fir.mlir.parseonly").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        // Parse FIRRTL into a new top-level MLIR module.
        let mlir_module = mlir::Module::parse(&ctx, &s);
        let mlir_module_body = mlir_module.body();

        // The 'firrtl.circuit' op 
        let circuit_op = mlir_module_body.first_op();
        let circuit_region = circuit_op.first_region().unwrap();
        let circuit_blk = circuit_region.first_block().unwrap();

        let mut next_module_op = Some(circuit_blk.first_op());
        while let Some(module_op) = next_module_op {
            module_op.dump();
            for idx in 0..module_op.num_attributes() {
                let attr = module_op.attribute_at(idx).unwrap();
                println!("{}", attr.name.to_string_ref().as_str());
                attr.dump();
            }

            println!("{}", module_op.num_attributes());
            let module_region = module_op.first_region().unwrap();
            let module_blk = module_region.first_block().unwrap();

            let mut next_op = Some(module_blk.first_op());
            while let Some(op) = next_op {
                if let Some(results) = op.results() {
                    for res in results { 
                        let ty = res.get_type();
                        //println!("{}", 
                        //    //unsafe { firrtlTypeIsGround(ty.raw()) }
                        //    unsafe { firrtlGetBitWidth(ty.raw(), false) }
                        //);
                    }
                }
                op.dump();
                next_op = op.next();
            }

            next_module_op = module_op.next();
        }

        Ok(())
    }
}


