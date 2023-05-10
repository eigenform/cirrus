
use std::env;
use std::path::{ Path, PathBuf };

// Error message emitted when $CIRCT_PATH is missing
const MISSING_CIRCT_PATH_ERR: &[&'static str] = {
    &[
        "The $CIRCT_PATH environment variable must be defined.",
        "You can do this by defining it in a '.cargo/config.toml file.",
        "For an absolute path to the CIRCT installation:",
        "",
        "  [env]",
        "  CIRCT_PATH = { value = \"path/to/circt\" }",
        "",
        "Or for a relative path to the CIRCT installation:",
        "",
        "  [env]",
        "  CIRCT_PATH = { value = \"path/to/circt\", relative = true }",
        "",
    ]
};

const LLVM_LIBRARIES: &[&'static str] = { &[
    "LLVMCore",
    "LLVMTargetParser",
    "LLVMBinaryFormat",
    "LLVMDemangle",
    "LLVMSupport",
]};
const MLIR_LIBRARIES: &[&'static str] = { &[
    "MLIRSupport",
    "MLIRLLVMCommonConversion",
    "MLIRIR",
    "MLIRDialectUtils",
    "MLIRAnalysis",
    "MLIRCAPIIR",
    "MLIRCallInterfaces",
    "MLIRCAPIControlFlow",
    "MLIRCAPIFunc",
    "MLIRControlFlowDialect",
    "MLIRControlFlowInterfaces",
    "MLIRLoopLikeInterface",
    "MLIRFuncDialect",
    "MLIRFuncTransforms",
    "MLIRInferTypeOpInterface",
    "MLIRInferIntRangeInterface",
    "MLIRInferIntRangeCommon",
    "MLIRViewLikeInterface",
    "MLIRShapedOpInterfaces",
    "MLIRPDLToPDLInterp",
    "MLIRPDLDialect",
    "MLIRPDLInterpDialect",
    "MLIRParser",
    "MLIRAsmParser",
    "MLIRBytecodeReader",
    "MLIRBytecodeWriter",
    "MLIRPass",
    "MLIRRewrite",
    "MLIRSideEffectInterfaces",
    "MLIRTransformUtils",
    "MLIRTransforms",
    "MLIRMemRefDialect",
    "MLIRMemRefTransforms",
    "MLIRMemRefTransformOps",
    "MLIRArithTransforms",
    "MLIRArithDialect",
    "MLIRArithUtils",
    "MLIRAffineDialect",
    "MLIRAffineUtils",
    "MLIRAffineTransformOps",
    "MLIRRuntimeVerifiableOpInterface",

]};
const CIRCT_LIBRARIES: &[&'static str] = { &[
    "CIRCTSupport",
    "CIRCTTransforms",
    "CIRCTHW",
    "CIRCTCAPIHWArith",
    "CIRCTHWArith",
    "CIRCTHWArithToHW",
    "CIRCTPipelineToHW",
    "CIRCTCAPIHW",
    "CIRCTHWTransforms",
    "CIRCTHWToLLVM",
    "CIRCTHandshakeToHW",
    "CIRCTCAPIComb",
    "CIRCTComb",
    "CIRCTCombToLLVM",
    "CIRCTSeq",
    "CIRCTCAPISeq",
    "CIRCTSeqTransforms",
    "CIRCTFSM",
    "CIRCTCAPIFSM",
    "CIRCTFSMTransforms",
    "CIRCTFSMToSV",
    "CIRCTSV",
    "CIRCTCAPISV",
    "CIRCTSVTransforms",
    "CIRCTExportVerilog",
    "CIRCTCAPIExportVerilog",
    "CIRCTCAPIFIRRTL",
    "CIRCTFIRRTL",
    "CIRCTExportChiselInterface",
    "CIRCTFIRRTLToHW",
    "CIRCTFIRRTLTransforms",
]};

fn main() -> Result<(), &'static str> {
    let Ok(CIRCT_PATH) = env::var("CIRCT_PATH") else {
        for line in MISSING_CIRCT_PATH_ERR {
            println!("cargo:warning={}", line);
        }
        return Err("Missing environment variable $CIRCT_PATH");
    };

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed={}", "CIRCT_PATH");

    // We're going to write output to ./bindings
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize().unwrap();
    let tgt_dir  = root_dir.join("bindings");
    std::fs::create_dir_all(&tgt_dir).unwrap();

    let circt_dir = PathBuf::from(CIRCT_PATH);
    let lib_dir = circt_dir.join("lib");
    let inc_dir = circt_dir.join("include");
    //let bin_dir = circt_dir.join("bin");

    println!("cargo:warning=[*] cirrus-sys using CIRCT_PATH={}", 
        circt_dir.to_str().unwrap()
    );

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=stdc++");

    // FIXME: Actually link up LLVM --system-libs?
    //println!("cargo:rustc-link-lib=rt");
    //println!("cargo:rustc-link-lib=dl");
    //println!("cargo:rustc-link-lib=m");
    //println!("cargo:rustc-link-lib=z");
    //println!("cargo:rustc-link-lib=zstd");
    //println!("cargo:rustc-link-lib=xml2");

    for libname in LLVM_LIBRARIES {
        println!("cargo:rustc-link-lib=static={}", libname);
    }
    for libname in MLIR_LIBRARIES {
        println!("cargo:rustc-link-lib=static={}", libname);
    }
    for libname in CIRCT_LIBRARIES {
        println!("cargo:rustc-link-lib=static={}", libname);
    }

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_block(true)
        .generate_inline_functions(true)
        .clang_args(&["-I", inc_dir.to_str().unwrap()])
        .generate()
        .expect("Couldn't generate bindings")
        .write_to_file(&tgt_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");

    Ok(())
}
