use std::env;
use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("idquantique/Libs-Apps")
        .canonicalize()
        .expect("cannot canonicalize path");
    let buildname = if os_type::current_platform().os_type == os_type::OSType::Alpine {
        "build_alpine"
    } else if in_container::in_container() {
        "build_docker"
    } else {
        "build"
    };
    let builddir = libdir_path.join(buildname);
    std::fs::create_dir_all(&builddir).unwrap();
    let cmakemodules = libdir_path.parent().unwrap().join("CMake");

    #[allow(clippy::wildcard_in_or_patterns)]
    let easy_quantis_opt = match os_type::current_platform().os_type {
        os_type::OSType::Debian | os_type::OSType::Ubuntu => "-DDISABLE_EASYQUANTIS=0",
        os_type::OSType::Alpine | _ => "-DDISABLE_EASYQUANTIS=1",
    };

    let cmdres = std::process::Command::new("cmake")
        .current_dir(&builddir)
        .arg(format!(
            "-D CMAKE_MODULE_PATH=\"{}\"",
            cmakemodules.to_str().unwrap()
        ))
        .arg("-DDISABLE_QUANTIS_JAVA=1")
        .arg("-DDISABLE_QUANTIS_PCI=1")
        .arg(easy_quantis_opt)
        .arg("-DDISABLE_EASYQUANTIS_GUI=1")
        .arg("-DUSE_CXX11=1")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("..")
        .output()
        .expect("could not spawn `cmake`");
    if !cmdres.status.success() {
        // Panic if the command was not successful.
        println!(
            "cmake -D CMAKE_MODULE_PATH=\"{}\" -D DISABLE_QUANTIS_JAVA=TRUE ..\n************************************************",
            cmakemodules.to_str().unwrap()
        );
        println!(
            "cmake stdout: \n{}\n************************************************",
            std::str::from_utf8(&cmdres.stdout).unwrap()
        );
        println!(
            "cmake stderr: \n{}\n************************************************",
            std::str::from_utf8(&cmdres.stderr).unwrap()
        );
        panic!("could not execute cmake");
    }

    let cmdres = std::process::Command::new("make")
        .current_dir(&builddir)
        .output()
        .expect("could not spawn `make`");
    if !cmdres.status.success() {
        println!(
            "make stdout: \n{}\n************************************************",
            std::str::from_utf8(&cmdres.stdout).unwrap()
        );
        println!(
            "make stderr: \n{}\n************************************************",
            std::str::from_utf8(&cmdres.stderr).unwrap()
        );
        panic!("could not execute make");
    }

    // Tell cargo to look for libraries in the specified directory
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search={}/idquantique/Libs-Apps/{}/Quantis",
        dir, buildname
    );

    // Tell cargo to tell rustc to link the library.
    // println!("cargo:rustc-link-lib=libQuantis");
    println!("cargo:rustc-link-lib=static=Quantis");

    // link libusb. On Debian that is  /usr/lib/x86_64-linux-gnu/libusb-1.0.a   on Alpine it is /usr/lib/libusb-1.0.a
    #[allow(clippy::wildcard_in_or_patterns)]
    match os_type::current_platform().os_type {
        // OSType::Unknown covers other debian-based OS
        os_type::OSType::Debian | os_type::OSType::Ubuntu | os_type::OSType::Unknown => {
            println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
        }
        os_type::OSType::Alpine | _ => {
            println!("cargo:rustc-link-search=/usr/lib");
        }
    }
    println!("cargo:rustc-link-lib=static=usb-1.0");

    #[allow(clippy::wildcard_in_or_patterns)]
    match os_type::current_platform().os_type {
        // OSType::Unknown covers other debian-based OS
        os_type::OSType::Debian | os_type::OSType::Ubuntu | os_type::OSType::Unknown => {
            // udev is only available as a dynamic library
            // https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=915566
            println!("cargo:rustc-link-lib=udev");
        }
        os_type::OSType::Alpine | _ => {
            // udev is neither available nor needed
        }
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
