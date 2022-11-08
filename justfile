default:
    just --list

build_directory := "build"

build-c:
    #!/usr/bin/env bash
    mkdir {{build_directory}}
    cd {{build_directory}}
    meson ..
    ninja

build-rust:
    cargo build

test: build-rust
    #!/usr/bin/env bash
    /usr/bin/env python3 -m pytest test -s



do-conversion:
    #!/usr/bin/env bash
    mkdir rust_output
    mkdir {{build_directory}}
    cd {{build_directory}}
    meson ..
    #intercept-build ninja
    c2rust transpile --emit-modules --output-dir ../rust_output --binary sshfs ./compile_commands.json
