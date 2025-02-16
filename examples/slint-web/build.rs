// #[cfg(not(feature = "server"))]
fn main() {
    slint_build::compile("ui/window.slint").unwrap();
}
// #[cfg(feature = "server")]
// fn main() {}
