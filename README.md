# Dependencies

bindgen requires LLVM/Clang:

    sudo apt install llvm-3.9-dev libclang-3.9-dev clang-3.9 autoconf texinfo

Build PETSc with:

    cd ~
    git clone -b maint https://bitbucket.org/petsc/petsc petsc
    ./configure PETSC_ARCH=arch-linux2-complex-debug --with-cc=mpicc --with-cxx=mpicxx --with-fc=mpif90 --download-fblaslapack --with-scalar-type=complex --with-shared-libraries=0
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug all
    make PETSC_DIR=$HOME/petsc PETSC_ARCH=arch-linux2-complex-debug test
