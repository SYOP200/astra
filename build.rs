use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_native();

    if cfg!(target_os = "macos") {
        build_macos();
    }
}

fn build_native() {
    let files = [
        "native/terminal.c",
        "native/process.c",
        "native/signal.c",
    ];

    let mut build = cc::Build::new();

    build.include("native/include");

    let mut found = false;

    for file in files {
        if Path::new(file).exists() {
            println!("cargo:rerun-if-changed={}", file);
            build.file(file);
            found = true;
        }
    }

    if found {
        build.compile("astra_native");
    }
}

fn build_macos() {
    let files = [
        "platform/macos/keychain.m",
        "platform/macos/notifications.m",
        "platform/macos/workspace.m",
        "platform/macos/finder.m",
        "platform/macos/dock.m",
    ];

    let mut build = cc::Build::new();

    build.include("native/include");

    build.flag("-fobjc-arc");

    let mut found = false;

    for file in files {
        if Path::new(file).exists() {
            println!("cargo:rerun-if-changed={}", file);
            build.file(file);
            found = true;
        }
    }

    if found {
        build.compile("astra_macos");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=UserNotifications");
    }
}
