fn main() {
    // don't link with stdlib
    println!("cargo:rustc-link-arg-bin=no_std=-nostartfiles");
}