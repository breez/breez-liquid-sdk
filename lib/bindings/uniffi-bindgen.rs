use camino::Utf8Path;
use uniffi_bindgen_kotlin_multiplatform::KotlinBindingGenerator;

fn main() {
    uniffi::uniffi_bindgen_main();

    let udl_file = "./src/breez_sdk_liquid.udl";
    let out_dir = Utf8Path::new("ffi/kmp");
    let config = Utf8Path::new("uniffi.toml");
    uniffi_bindgen::generate_external_bindings(
        KotlinBindingGenerator {},
        udl_file,
        Some(config),
        Some(out_dir),
        None::<&Utf8Path>,
        None,
    )
    .unwrap();
}
