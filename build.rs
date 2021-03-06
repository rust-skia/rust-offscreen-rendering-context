use std::env;
use std::fs::File;
use std::path::PathBuf;
use gl_generator::{Registry, Api, Profile, Fallbacks};

fn main() {
    let target = env::var("TARGET").unwrap();
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    if target.contains("android") ||
        (target.contains("windows") && cfg!(feature = "no_wgl")) ||
        cfg!(feature = "test_egl_in_linux")
    {
        let mut file = File::create(&dest.join("egl_bindings.rs")).unwrap();
        Registry::new(Api::Egl, (1, 5), Profile::Core, Fallbacks::All, [])
            .write_bindings(gl_generator::StaticGenerator, &mut file).unwrap();

        // Historically, Android builds have succeeded with rust-link-lib=EGL.
        // On Windows when relying on %LIBS% to contain libEGL.lib, however,
        // we must explicitly use rustc-link-lib=libEGL or rustc will attempt
        // to link EGL.lib instead.
        if target.contains("windows") {
            println!("cargo:rustc-link-lib=libEGL");
        } else {
            println!("cargo:rustc-link-lib=EGL");
        }
    }

    if target.contains("apple-ios") {
        println!("cargo:rustc-link-lib=framework=OpenGLES");
    }

    if target.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    } else if target.contains("windows") && env::var("CARGO_FEATURE_NO_WGL").is_err() {
        let mut file = File::create(&dest.join("wgl_bindings.rs")).unwrap();
        Registry::new(Api::Wgl, (1, 0), Profile::Core, Fallbacks::All, [])
            .write_bindings(gl_generator::StaticGenerator, &mut file)
            .unwrap();

        let mut file = File::create(&dest.join("wgl_extra_bindings.rs")).unwrap();
        Registry::new(Api::Wgl, (1, 0), Profile::Core, Fallbacks::All, [
                          "WGL_ARB_create_context",
                          "WGL_ARB_create_context_profile",
                          "WGL_ARB_create_context_robustness",
                          "WGL_ARB_context_flush_control",
                          "WGL_ARB_extensions_string",
                          "WGL_ARB_framebuffer_sRGB",
                          "WGL_ARB_multisample",
                          "WGL_ARB_pixel_format",
                          "WGL_ARB_pixel_format_float",
                          "WGL_EXT_create_context_es2_profile",
                          "WGL_EXT_extensions_string",
                          "WGL_EXT_framebuffer_sRGB",
                          "WGL_EXT_swap_control",
                      ])
            .write_bindings(gl_generator::StructGenerator, &mut file).unwrap();

        println!("cargo:rustc-link-lib=opengl32");
    } else if cfg!(feature = "x11") &&
        !target.contains("android") &&
        !target.contains("apple-ios") &&
        !target.contains("windows")
    {
        let mut file = File::create(&dest.join("glx_bindings.rs")).unwrap();
        Registry::new(Api::Glx, (1, 4), Profile::Core, Fallbacks::All, [])
            .write_bindings(gl_generator::StaticGenerator, &mut file).unwrap();

        let mut file = File::create(&dest.join("glx_extra_bindings.rs")).unwrap();
        Registry::new(Api::Glx, (1, 4), Profile::Core, Fallbacks::All, [
                          "GLX_ARB_create_context",
                      ])
            .write_bindings(gl_generator::StructGenerator, &mut file).unwrap();
        println!("cargo:rustc-link-lib=GL");
    }
}
