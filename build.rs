#![ allow(warnings)]
#![windows_subsystem = "windows"]

use std::env;
use std::fs::File;
use std::path::PathBuf;

use std::path::Path;
use std::fs;

use cfg_aliases::cfg_aliases;
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    // XXX this is taken from glutin/build.rs.

    // Setup alias to reduce `cfg` boilerplate.
    cfg_aliases! {
        // Systems.
        android_platform: { target_os = "android" },
        wasm_platform: { target_family = "wasm" },
        macos_platform: { target_os = "macos" },
        ios_platform: { target_os = "ios" },
        apple: { any(ios_platform, macos_platform) },
        free_unix: { all(unix, not(apple), not(android_platform)) },

        // Native displays.
        x11_platform: { all(feature = "x11", free_unix, not(wasm_platform)) },
        wayland_platform: { all(feature = "wayland", free_unix, not(wasm_platform)) },

        // Backends.
        egl_backend: { all(feature = "egl", any(windows, unix), not(apple), not(wasm_platform)) },
        glx_backend: { all(feature = "glx", x11_platform, not(wasm_platform)) },
        wgl_backend: { all(feature = "wgl", windows, not(wasm_platform)) },
        cgl_backend: { all(macos_platform, not(wasm_platform)) },
    }

    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();

    let pastaExe = env::var("PROFILE").unwrap();
    let pastaExe = PathBuf::from(format!("target/{}/{}", pastaExe, "resources"));

    if pastaExe.exists() {
        fs::remove_dir_all(&pastaExe).unwrap();
    }

    fs::create_dir(&pastaExe).unwrap();

    copy_dir("resources", &pastaExe);
}

fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>> (de: P, para: Q) {
    let para = para.as_ref().to_path_buf();

    for caminho in fs::read_dir(de).unwrap() {
        let caminho = caminho.unwrap().path();
        let para = para.clone().join(caminho.file_name().unwrap());

        if caminho.is_file() {
            fs::copy(caminho, para).unwrap();
        } else if caminho.is_dir() {
            if !para.exists() {
                fs::create_dir(&para).unwrap();
            }

            copy_dir(&caminho, para);
        }
    }
}
