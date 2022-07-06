
use cc;

use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    let source = PathBuf::from("deps").join("termbox2").join("termbox.h");

    // Annoying, cc only works if the file's extension is '.c' not '.h'
    let source_c = PathBuf::from(env::var("OUT_DIR").unwrap()).join("termbox.c");
    fs::copy(&source, &source_c).unwrap();

    // Compilation command taken from the termbox makefile
    // cc -DTB_IMPL -DTB_OPT_TRUECOLOR -DTB_OPT_EGC -fPIC -xc -c -std=c99
    //    -Wall -Wextra -pedantic -Wno-unused-result -g -O0 -D_XOPEN_SOURCE
    //    -D_DEFAULT_SOURCE termbox.h -o termbox.o
    // ar rcs libtermbox2.a termbox.o
    cc::Build::new()
        .file(&source_c)
        .define("TB_IMPL", None)
        .define("TB_OPT_TRUECOLOR", None)
        .define("TB_OPT_EGC", None)
        .pic(true)
        .warnings(true)
        .define("_XOPEN_SOURCE", None)
        .define("_DEFAULT_SOURCE", None)
        .compile("termbox2");
    println!("cargo:rerun-if-changed={}", source.display());
}
