pub mod overlay_fullscreen;

use std::time::{Duration,
                Instant};

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font,
                     TextLayout,
                     TextOptions};
pub use speedy2d::window::UserEventSender;
use speedy2d::window::{WindowCreationOptions,
                       WindowHandler,
                       WindowHelper,
                       WindowPosition,
                       WindowSize};
use speedy2d::{Graphics2D,
               Window};

pub enum Messages {
    TitleOn,
    TitleOff,
    IsOn,
    IsOff,
    Draw,
    Msg(&'static str),
}

// const FONT: Font = Font::new(include_bytes!("../../assets/NotoSans-Regular.ttf")).unwrap();

#[derive(Default)]
pub struct DebugState {
    pub is_on: bool,
    pub title_on: bool,
}

pub struct Overlay {
    font: Font,
    debug_state: DebugState,
    current_msgs: [Option<(&'static str, Instant)>; 3],
    to_show_msgs: Vec<&'static str>,
}

impl WindowHandler<Messages> for Overlay {
    fn on_user_event(&mut self, helper: &mut WindowHelper<Messages>, user_event: Messages) {
        let mut has_changes = false;
        match user_event {
            Messages::TitleOn => {
                if !self.debug_state.title_on {
                    self.debug_state.title_on = true;
                    has_changes = true;
                }
            }
            Messages::TitleOff => {
                if self.debug_state.title_on {
                    self.debug_state.title_on = false;
                    has_changes = true;
                }
            }
            Messages::IsOn => {
                if !self.debug_state.is_on {
                    self.debug_state.is_on = true;
                    has_changes = true;
                }
            }
            Messages::IsOff => {
                if self.debug_state.is_on {
                    self.debug_state.is_on = false;
                    has_changes = true;
                }
            }
            Messages::Msg(msg) => self.to_show_msgs.push(msg),
            Messages::Draw => helper.request_redraw(),
        }

        if has_changes {
            helper.request_redraw();
        }
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper<Messages>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);

        {
            let block = self.font.layout_text("CharOn: ", 18.0, TextOptions::new());
            graphics.draw_text((0.0, 100.0), Color::YELLOW, &block);
            if self.debug_state.is_on {
                graphics.draw_text(
                    (0.0 + block.width() + 1.0, 100.0),
                    Color::GREEN,
                    &self.font.layout_text("TRUE", 18.0, TextOptions::new()),
                );
            } else {
                graphics.draw_text(
                    (0.0 + block.width() + 1.0, 100.0),
                    Color::RED,
                    &self.font.layout_text("FALSE", 18.0, TextOptions::new()),
                );
            }
        }

        {
            let block = self.font.layout_text("PoEWindowOn: ", 18.0, TextOptions::new());
            graphics.draw_text((0.0, 120.0), Color::YELLOW, &block);
            if self.debug_state.title_on {
                graphics.draw_text(
                    (0.0 + block.width() + 1.0, 120.0),
                    Color::GREEN,
                    &self.font.layout_text("TRUE", 18.0, TextOptions::new()),
                );
            } else {
                graphics.draw_text(
                    (0.0 + block.width() + 1.0, 120.0),
                    Color::RED,
                    &self.font.layout_text("FALSE", 18.0, TextOptions::new()),
                );
            }
        }

        {
            // let block = self.font.layout_text("PoEWindowOn: ", 18.0, TextOptions::new());

            for handle in 0..3 {
                match &self.current_msgs[handle] {
                    Some((_, inst)) => {
                        if inst.elapsed().cmp(&Duration::from_secs(4)).is_gt() {
                            self.current_msgs[handle] = None;
                        }
                    }
                    None => {
                        if let Some(msg) = self.to_show_msgs.pop() {
                            self.current_msgs[handle] = Some((msg, Instant::now()));
                        }
                    }
                }
            }

            for (i, opt) in self.current_msgs.iter().enumerate() {
                if let Some((msg, _)) = opt {
                    graphics.draw_text(
                        (0.0, 0.0 + i as f32 * 20.0),
                        Color::YELLOW,
                        &self.font.layout_text(msg, 18.0, TextOptions::new()),
                    );
                }
            }
        }
    }
}

pub struct BuilderOverlay {
    font: Font,
    debug_state: DebugState,
    window: Window<Messages>,
}

impl BuilderOverlay {
    pub fn new() -> Self {
        let options = WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Vector2 { x: 200, y: 200 }),
            Some(WindowPosition::PrimaryMonitorPixelsFromTopLeft(Vector2::new(705, 870))),
        )
        .with_transparent(false)
        .with_maximized(false)
        .with_vsync(false)
        .with_always_on_top(true)
        .with_decorations(false);
        let window = Window::<Messages>::new_with_user_events("pors", options).unwrap();

        Self {
            font: Font::new(include_bytes!("../../assets/NotoSans-Regular.ttf")).unwrap(),
            debug_state: DebugState::default(),
            window,
        }
    }

    pub fn use_events(&self) -> UserEventSender<Messages> {
        self.window.create_user_event_sender()
    }

    pub fn build_and_run(self) {
        let BuilderOverlay { font, debug_state, window } = self;
        let overlay = Overlay {
            font,
            debug_state,
            current_msgs: [None, None, None],
            to_show_msgs: Vec::new(),
        };
        window.run_loop(overlay);
    }
}

impl Default for BuilderOverlay {
    fn default() -> Self {
        Self::new()
    }
}

// impl Overlay {
//     pub fn new() -> Self {
//         Self {
//             font: Font::new(include_bytes!("../../assets/NotoSans-Regular.ttf")).unwrap(),
//             debug_state: DebugState::default(),
//         }
//     }
// }
