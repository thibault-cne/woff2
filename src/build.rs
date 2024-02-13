extern crate cc;
extern crate cpp_build;

use ::cc::Build;
use ::cpp_build::Config;
use ::std::env::var;

fn main() {
    compile_cpp_glue_code();
    compile_woff2_lib();
}

fn compile_cpp_glue_code() {
    Config::new()
        .include("lib/woff2/include")
        .build("src/lib.rs");
}

fn compile_woff2_lib() {
    let brotli_include_folder_path = var("DEP_BROTLI_INCLUDE").unwrap();
    let target = var("TARGET").unwrap();
    let is_compiling_for_macosx = target.rfind("-darwin").is_some();

    let mut build = Build::new();
    build
        .cpp(true)
        .shared_flag(false)
        .static_flag(true)
        .warnings(false)
        .flag("-fno-omit-frame-pointer")
        .flag("-no-canonical-prefixes")
        .flag("-std=c++11")
        .define(" __STDC_FORMAT_MACROS", None);

    if is_compiling_for_macosx {
        build.define("OS_MACOSX", None);
    } else {
        build.flag("-fno-tree-vrp");
    }

    build
        .include(brotli_include_folder_path)
        .include("lib/woff2/include")
        .file("lib/woff2/src/font.cc")
        .file("lib/woff2/src/glyph.cc")
        .file("lib/woff2/src/normalize.cc")
        .file("lib/woff2/src/table_tags.cc")
        .file("lib/woff2/src/transform.cc")
        .file("lib/woff2/src/woff2_dec.cc")
        .file("lib/woff2/src/woff2_enc.cc")
        .file("lib/woff2/src/woff2_common.cc")
        .file("lib/woff2/src/woff2_out.cc")
        .file("lib/woff2/src/variable_length.cc")
        .compile("woff2");
}
