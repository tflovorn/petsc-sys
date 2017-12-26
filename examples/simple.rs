extern crate libc;
extern crate petsc_sys;

use libc::{c_char, c_int};
use std::ffi::CString;
use petsc_sys::{PetscInitialize, PetscPrintf, PetscFinalize, PETSC_COMM_WORLD};

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();

    let mut c_argc = argc as c_int;
    let mut c_argv = argv.into_iter().map(|arg| CString::new(arg).unwrap().into_raw()).collect::<Vec<*mut c_char>>();
    let mut c_argv_ptr = c_argv.as_mut_ptr();

    unsafe {
        let ierr = PetscInitialize(&mut c_argc, &mut c_argv_ptr, std::ptr::null(), std::ptr::null());
        if ierr != 0 {
            println!("error code {} from PetscInitialize", ierr);
        }

        let msg = CString::new("Hello from PETSc\n").unwrap();

        let ierr = PetscPrintf(PETSC_COMM_WORLD, msg.as_ptr());
        if ierr != 0 {
            println!("error code {} from PetscPrintf", ierr);
        }

        let ierr = PetscFinalize();
        if ierr != 0 {
            println!("error code {} from PetscFinalize", ierr);
        }
    };
}
