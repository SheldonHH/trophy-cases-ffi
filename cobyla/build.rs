extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // 初始化CMake配置
    let mut config = cmake::Config::new("./cobyla");
    
    // 根据目标环境配置编译器
    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        config.very_verbose(true).generator("NMake Makefiles");
    } else {
        // 为非MSVC环境添加AddressSanitizer标志
        config.cflag("-fsanitize=address")
              .cxxflag("-fsanitize=address");
            //   .ldflag("-fsanitize=address");
    }

    // 设置构建目标
    config.build_target("cobyla");
    let dst = config.build();

    // 设置链接路径和库
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=cobyla");

    // 告诉cargo链接系统库
    println!("cargo:rustc-link-lib=cobyla");

    // 当wrapper.h文件变化时重新生成绑定
    println!("cargo:rerun-if-changed=wrapper.h");

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("raw_cobyla")
        .generate()
        .expect("Unable to generate bindings");

    // 写入绑定到OUT_DIR/bindings.rs文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
