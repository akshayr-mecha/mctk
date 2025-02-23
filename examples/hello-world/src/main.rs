use std::collections::HashMap;
use std::time::Duration;
// mod counter;
use mctk_core::component::{Component, Message, RenderContext, RootComponent};
use mctk_core::layout::Direction;
use mctk_core::reexports::cosmic_text;
use mctk_core::renderables::{types, Renderable};
use mctk_core::style::Styled;
use mctk_core::widgets::{self, Button, Div, TextBox};
use mctk_core::{lay, msg, rect, size, txt, Color};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};
use mctk_smithay::layer_surface::LayerOptions;
use mctk_smithay::layer_window::LayerWindowParams;
use mctk_smithay::WindowOptions;
use smithay_client_toolkit::shell::wlr_layer;
// use tracing_subscriber::EnvFilter;

// App level channel
pub enum AppMessage {}

#[derive(Debug, Default)]
pub struct AppState {
    value: f32,
    btn_pressed: bool,
}

#[derive(Debug, Clone)]
enum HelloEvent {
    ButtonPressed {
        name: String,
    },
    TextBox {
        name: String,
        value: String,
        update_type: String,
    },
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

        Some(
            node!(
                Div::new(),
                lay![
                    size: [480, 480],
                    direction: Direction::Column
                ]
            )
            .push(node!(
                Button::new(txt!("A!"))
                    .on_click(Box::new(|| msg!(HelloEvent::ButtonPressed {
                        name: "Clicked".to_string()
                    })))
                    .on_double_click(Box::new(|| msg!(HelloEvent::ButtonPressed {
                        name: "Double clicked".to_string()
                    })))
                    .style("color", Color::rgb(255., 0., 0.))
                    .style("background_color", Color::rgb(255., 255., 255.))
                    .style("active_color", Color::rgb(200., 200., 200.))
                    .style("font_size", 24.0),
                lay![size: size!(180.0, 180.0), margin: [0., 0., 20., 0.]]
            ))
            .push(node!(
                TextBox::new(Some("".to_string()))
                    .style("background_color", Color::WHITE)
                    .style("font_size", 16.)
                    .style("text_color", Color::BLACK)
                    .style("border_width", 0.)
                    .style("cursor_color", Color::BLACK)
                    .style("placeholder_color", Color::MID_GREY)
                    .placeholder("Type here")
                    .on_change(Box::new(|s| msg!(HelloEvent::TextBox {
                        name: "textbox".to_string(),
                        value: s.to_string(),
                        update_type: "change".to_string(),
                    })))
                    .on_commit(Box::new(|s| msg!(HelloEvent::TextBox {
                        name: "textbox".to_string(),
                        value: s.to_string(),
                        update_type: "commit".to_string(),
                    }))),
                [size: [300, 30]])),
        )
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
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    // tracing_subscriber::fmt()
    //     .compact()
    //     .with_env_filter(env_filter)
    //     .init();

    // let mut fonts: Vec<String> = Vec::new();
    let assets: HashMap<String, String> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    svgs.insert(
        "eye_icon".to_string(),
        "./src/assets/icons/eye.svg".to_string(),
    );

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    fonts.load_font_data(include_bytes!("assets/fonts/SpaceGrotesk-Regular.ttf").into());

    let namespace = "mctk.layer_shell.demo".to_string();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Overlay,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0,
    };

    let window_opts = WindowOptions {
        height: 480 as u32,
        width: 480 as u32,
        scale_factor: 1.0,
    };

    let (mut app, mut event_loop, ..) =
        mctk_smithay::layer_window::LayerWindow::open_blocking::<App, AppMessage>(
            LayerWindowParams {
                title: "Hello world!".to_string(),
                namespace,
                window_opts,
                fonts,
                assets,
                svgs,
                layer_shell_opts,
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
