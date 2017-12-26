# Dependencies

bindgen requires LLVM/Clang:

    sudo apt install llvm-3.9-dev libclang-3.9-dev clang-3.9 autoconf texinfo

Build and test PETSc (complex scalars, debug mode) with:

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

`PETSC_LIB` -- the directory containing the petsc static library files.
Typically equal to `PETSC_DIR/PETSC_ARCH/lib`.

The `run_build` script chooses values for these environment variables appropriate for
the PETSc build procedure described here in the 'Dependencies' section.

# TODO

Add PETSc build instructions for release mode. The following should work:

    ./configure PETSC_ARCH=arch-linux2-complex-opt --with-cc=mpicc --with-cxx=mpicxx --with-fc=mpif90 --download-fblaslapack --with-scalar-type=complex --with-shared-libraries=0 --with-debugging=0 COPTFLAGS='-O3 -march=native -mtune=native' FOPTFLAGS='-O3 -march=native -mtune=native'
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-opt all
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-opt test

Add PETSc build instructions for real scalars.

Find a clean way to swap PETSc libraries with different build modes: debug vs release, complex vs real scalars.
As-is, one of these is chosen via the environment variables described in 'Usage' the first time
a module depending on `petsc-sys` is compiled, and a clean build (`cargo clean`) is required to
rebuild to apply new environment variables.
(At least, expect that this is true - have not verified.)

Relatedly, improve handling of PETSc build in `build.rs`. Follow convention of searching for
system PETSc, then building in `build.rs` if not available. Can use `pkg-config` to search for
system PETSc. Retain environment variable-based approach to allow for system PETSc installation
in non-standard path. If searching for system PETSc, need to discriminate between the
various builds (debug/release, scalar type).

May want to adopt the approach of the [github,com/JuliaParallel/PETSc.jl](Julia wrapper) `PETSc.jl` and make multiple PETSc
builds available simultaneously from Rust. There, single-precision and double-precision real scalars and double-precision
complex scalars are all available at once (see `PETSc.jl/src/generated/defs.jl`).
With this, still need to allow clean switch between debug and release builds of PETSc.
