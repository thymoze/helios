fn main() {
    if cfg!(feature = "rpi") {
        println!("cargo:rustc-link-search=native=firmware/opt/vc/lib/");
    }
}
