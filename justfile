default:
    just --list

build_directory := "build"

build:
    #!/usr/bin/env bash
    mkdir {{build_directory}}
    cd {{build_directory}}
    meson ..
    ninja


test: build
    #!/usr/bin/env bash
    cd {{build_directory}}
    /usr/bin/env python3 -m pytest test



do-conversion:
    #!/usr/bin/env bash
    mkdir rust_output
    mkdir {{build_directory}}
    cd {{build_directory}}
    meson ..
    #intercept-build ninja
    c2rust transpile --emit-modules --output-dir ../rust_output --binary sshfs ./compile_commands.json
