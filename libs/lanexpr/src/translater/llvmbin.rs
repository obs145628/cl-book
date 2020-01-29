use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;

use crate::ast;
use crate::bindapp::BindApp;
use crate::translater::llvmtl;

use clangutils::{ClangCommandBuilder, ClangOutputType};

fn calculate_hash(t: &str) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// write LLVM ir file to out_path
pub fn write_ir_to_file(root: &ast::ASTExprPtr, ba: &BindApp, out_path: &str) {
    let mut tl = llvmtl::Translater::new(root, ba);
    tl.set_output_ll_path(out_path);
    tl.translate();
}

/// Compile the runtime library: contains all standard functions, and the entry main function that calls the user main
/// Build a .o file, and return its path
/// Doesn't do anything if the file already exists
pub fn compile_runtime_lib() -> &'static str {
    let rt_src_path = "./misc/lanexpr_llvm_runtime.c";
    let rt_o_path = "/tmp/tmp_lanexpr_llvm_runtime.o";

    if Path::new(rt_o_path).exists() {
        return rt_o_path;
    }

    ClangCommandBuilder::new()
        .set_input(rt_src_path)
        .set_output(rt_o_path)
        .set_output_type(ClangOutputType::OBJECT)
        .run();

    rt_o_path
}

/// Generate LLVM IR, and use it to create a standalone binary at out_path
pub fn compile_to_binary(root: &ast::ASTExprPtr, ba: &BindApp, out_path: &str) {
    let tmp_ir_path = format!("/tmp/tmp_cl_lanexpr_mod_{}.ll", calculate_hash(out_path));
    let tmp_o_path = format!("/tmp/tmp_cl_lanexpr_mod_{}.o", calculate_hash(out_path));

    write_ir_to_file(root, ba, &tmp_ir_path);

    ClangCommandBuilder::new()
        .set_input(&tmp_ir_path)
        .set_output(&tmp_o_path)
        .set_output_type(ClangOutputType::OBJECT)
        .run();

    let rt_o_path = compile_runtime_lib();

    ClangCommandBuilder::new()
        .set_inputs(&[&tmp_o_path, rt_o_path])
        .set_output(out_path)
        .set_output_type(ClangOutputType::BINARY)
        .run();

    std::fs::remove_file(&tmp_ir_path).expect("Failed to remove temporary LLVM IR file");
    std::fs::remove_file(&tmp_o_path).expect("Failed to remove temporary LLVM o file");
}
