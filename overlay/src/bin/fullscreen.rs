use speedy2d::window::WindowCreationOptions;
use speedy2d::Window;

fn main() {
    tracing_subscriber::fmt::init();
    // let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();

    let options = WindowCreationOptions::new_fullscreen_borderless()
        .with_transparent(true)
        .with_maximized(true)
        .with_vsync(false)
        .with_decorations(false);

    let _window = Window::new_with_options("pors", options).unwrap();

    // window.run_loop(overlay_fullscreen::OverlayFullscreen::new())
}
