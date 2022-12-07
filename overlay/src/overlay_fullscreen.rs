use makro::Pos;
use speedy2d::color::Color;
use speedy2d::font::{Font,
                     TextLayout,
                     TextOptions};
use speedy2d::window::{WindowCreationOptions,
                       WindowHandler,
                       WindowHelper};
use speedy2d::{Graphics2D,
               Window};

pub enum Messages {
    LineVertical((Pos, u8)),
    LineHorizontal((Pos, u8)),
    Text((Pos, String)),
    Clear,
}

/// Transparent overlay that is supposed to be over the entire screen
/// You can't do a passthrough of mouse / keyboard events so its mostly for testing
pub struct OverlayFullscreen {
    rx: std::sync::mpsc::Receiver<Messages>,
    to_draw: Vec<Messages>,
    font: Font,
}

impl OverlayFullscreen {
    pub fn new(rx: std::sync::mpsc::Receiver<Messages>) -> Self {
        let font = Font::new(include_bytes!("../../assets/NotoSans-Regular.ttf")).unwrap();
        Self { font, rx, to_draw: Vec::new() }
    }

    pub fn run(self) {
        let options = WindowCreationOptions::new_fullscreen_borderless()
            .with_transparent(true)
            .with_maximized(true)
            .with_vsync(false)
            .with_always_on_top(true)
            .with_decorations(false);

        let window = Window::new_with_options("pors", options).unwrap();

        window.run_loop(self);
    }
}

impl WindowHandler for OverlayFullscreen {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // graphics.clear_screen(Color::TRANSPARENT);

        // graphics.draw_circle((150.0, 120.0), 75.0, Color::from_rgb(0.8, 0.9, 1.0));
        // graphics.draw_line((100.0, 100.0), (100.0, 500.0), 5.0, Color::BLACK);

        match self.rx.try_recv() {
            Ok(ok) => self.to_draw.push(ok),
            Err(_err) => {
                // dbg!(err);
            }
        }

        // let mut clear = false;
        for v in &self.to_draw {
            match v {
                Messages::LineHorizontal((pos, length)) => {
                    graphics.draw_line(
                        (pos.x as f32, pos.y as f32 + 0.5),
                        (pos.x as f32 + 0.5 + *length as f32, pos.y as f32 + 0.5),
                        0.5,
                        Color::RED,
                    );
                }
                Messages::LineVertical((pos, length)) => {
                    graphics.draw_line(
                        (pos.x as f32, pos.y as f32 + 0.5),
                        (pos.x as f32 + 0.5, pos.y as f32 + *length as f32 + 0.5),
                        0.5,
                        Color::RED,
                    );
                }
                Messages::Text((pos, text)) => {
                    let block = self.font.layout_text(text, 12.0, TextOptions::new());
                    graphics.draw_text((pos.x as f32, pos.y as f32), Color::GREEN, &block);
                }
                Messages::Clear => {
                    self.to_draw.drain(..);
                    break;
                }
            }
            helper.request_redraw();
        }

        // if clear {
        //     self.to_draw.drain(..);
        // }

        // graphics.draw_line((0.0, 10.5), (1.0, 10.5), 1.0, Color::RED);
        // let crop_window = Rect::from_tuples((0.0, 0.0), (1.0, 1.0));
        // graphics.draw_rectangle(crop_window, Color::RED);
    }
}
