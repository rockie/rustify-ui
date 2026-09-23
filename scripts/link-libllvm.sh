#!/bin/sh
# Lets the toolchain's own LLVM tools start without rustup's help.
#
# A toolchain that links LLVM dynamically ships libLLVM in `<sysroot>/lib`,
# but rust-lld and rust-objcopy look for it in `lib/rustlib/<host>/lib`, next
# to themselves. Under rustup's proxies that goes unnoticed, because the proxy
# puts `<sysroot>/lib` on the dynamic library path. mbx starts the toolchain's
# cargo directly and clears that path, so the wasm link fails with
# "Library not loaded: @rpath/libLLVM.dylib". A link next to the tools is
# where their rpath already looks.
#
# Run by mise after it installs rust (see mise.toml); safe to run again, and a
# no-op for a toolchain that has no libLLVM of its own.
set -eu

sysroot=$(rustc --print sysroot)
host=$(rustc -vV | sed -n 's/^host: //p')
tools_lib="$sysroot/lib/rustlib/$host/lib"

for lib in "$sysroot"/lib/libLLVM*; do
    [ -e "$lib" ] || continue
    name=$(basename "$lib")
    if [ ! -e "$tools_lib/$name" ]; then
        ln -s "../../../$name" "$tools_lib/$name"
        echo "linked $tools_lib/$name"
    fi
done
