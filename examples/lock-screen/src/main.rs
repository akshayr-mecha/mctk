use mctk_core::component::{Component, Message, RenderContext, RootComponent};
use mctk_core::reexports::cosmic_text;
use mctk_core::renderables::{types, Renderable};
use mctk_core::style::Styled;
use mctk_core::widgets::Button;
use mctk_core::{lay, msg, size, txt, Color};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};
use mctk_smithay::lock_window::SessionLockWindowParams;
use mctk_smithay::WindowOptions;
use smithay_client_toolkit::reexports::calloop;
use std::collections::HashMap;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

// App level channel
pub enum AppMessage {}

#[derive(Debug, Default)]
pub struct AppState {
    value: f32,
    btn_pressed: bool,
}

#[derive(Debug, Clone)]
enum HelloEvent {
    ButtonPressed { name: String },
}

#[component(State = "AppState")]
#[derive(Debug, Default)]
pub struct App {}

#[state_component_impl(AppState)]
impl Component for App {
    fn init(&mut self) {
        self.state = Some(AppState {
            value: 30.,
            btn_pressed: false,
        })
    }

    fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
        None
    }

    fn view(&self) -> Option<Node> {
        let btn_pressed = self.state_ref().btn_pressed;

        Some(node!(
            Button::new(txt!("Click me!"))
                .on_click(Box::new(|| msg!(HelloEvent::ButtonPressed {
                    name: "It me, a button!".to_string()
                })))
                .style(
                    "background_color",
                    match btn_pressed {
                        true => Color::rgb(255., 0., 0.),
                        false => Color::rgb(0., 0., 255.),
                    }
                )
                .style("font_size", 16.0),
            lay!(size: size!(60.0, 60.0)),
        ))
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        println!("App has sent: {:?}", message.downcast_ref::<HelloEvent>());
        match message.downcast_ref::<HelloEvent>() {
            Some(HelloEvent::ButtonPressed { name }) => {
                println!("{}", name);
                self.state_mut().btn_pressed = true;
            }
            _ => (),
        }
        vec![]
    }
}

// Layer Surface App
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let assets: HashMap<String, String> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    fonts.load_font_data(include_bytes!("assets/fonts/SpaceGrotesk-Regular.ttf").into());

    let window_opts = WindowOptions {
        height: 480 as u32,
        width: 480 as u32,
        scale_factor: 1.0,
    };

    let (session_lock_tx, session_lock_rx) = calloop::channel::channel();

    let (mut app, mut event_loop, ..) =
        mctk_smithay::lock_window::SessionLockWindow::open_blocking::<App, AppMessage>(
            SessionLockWindowParams {
                window_opts,
                fonts,
                assets,
                svgs,
                session_lock_tx,
                session_lock_rx,
            },
            None,
        );
    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }

    Ok(())
}

impl RootComponent<AppMessage> for App {}
