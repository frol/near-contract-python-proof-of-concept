use rustpython_vm as vm;
use rustpython_vm::{
    pyclass, pymodule, PyObject, PyPayload, PyResult, TryFromBorrowedObject, VirtualMachine,
};

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[cfg(target = "wasm32-unknown-unknown")]
#[no_mangle]
pub extern "C" fn method2() {
    let value = b"Hello from Python!".to_vec();
    unsafe { near_sys::value_return(value.len() as _, value.as_ptr() as _) };
}

// #[cfg(target = "wasm32-unknown-unknown")]
#[no_mangle]
pub extern "C" fn method() {
    vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_module(
            "rust_py_module".to_owned(),
            Box::new(rust_py_module::make_module),
        );
    })
    .enter(run);
}

fn run(vm: &vm::VirtualMachine) -> vm::PyResult<()> {
    let scope = vm.new_scope_with_builtins();

    // the file parameter is relative to the directory where the crate's Cargo.toml is located, see $CARGO_MANIFEST_DIR:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    let module = vm::py_compile!(file = "src/freeze.py");

    let res = vm.run_code_obj(vm.ctx.new_code(module), scope);

    // if let Err(exc) = res {
    //     vm.print_exception(exc);
    // }

    Ok(())
}

#[pymodule]
mod rust_py_module {
    use super::*;
    use rustpython_vm::{builtins::PyList, convert::ToPyObject, PyObjectRef};

    #[pyfunction]
    fn rust_function(_vm: &VirtualMachine) -> PyResult<()> {
        let value = b"Hello from Python!".to_vec();
        unsafe { near_sys::value_return(value.len() as _, value.as_ptr() as _) };
        Ok(())
    }
}
