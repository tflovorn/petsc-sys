# Dependencies

bindgen requires LLVM/Clang:

    sudo apt install llvm-3.9-dev libclang-3.9-dev clang-3.9 autoconf texinfo

Build PETSc with:

    cd ~
    git clone -b maint https://bitbucket.org/petsc/petsc petsc
    ./configure PETSC_ARCH=arch-linux2-complex-debug --with-cc=mpicc --with-cxx=mpicxx --with-fc=mpif90 --download-fblaslapack --with-scalar-type=complex --with-shared-libraries=0
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug all
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug test

# Usage

The `build.rs` script requires the following environment variables to be set:

`PETSC_DIR` -- the root petsc directory

`PETSC_ARCH` -- the `PETSC_ARCH` setting that petsc was compiled with.
Files depending on this arch setting will be located in `PETSC_DIR/PETSC_ARCH`.

`PETSC_LIB` -- the directory containing the petsc shared library files.
Typically equal to `PETSC_DIR/PETSC_ARCH/lib`.

The `run_build` script chooses values for these environment variables appropriate for
the PETSc build procedure described here in the 'Dependencies' section.
