fn main() {
    //     // NOTE: Both of these might not be necessary, and there may be a better
    //     // way of doing this. This is just what I found to work.
        println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined");
        println!("cargo:rustc-cdylib-link-arg=-Wl,dynamic_lookup");
}
