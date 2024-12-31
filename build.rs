fn main() {
    slint_build::compile("components/app.slint").unwrap();

    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
            .set_icon("icons/ico.ico")
            .compile()
            .unwrap();
    }
}