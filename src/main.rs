use clap::Parser;
use std::{
    borrow::Cow,
    fs::File,
    io::{stdin, Read},
    path::{Path, PathBuf},
};
use tao::{
    dpi::PhysicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{Method, Request, Response, StatusCode},
    WebViewBuilder,
};

const SCHEME: &str = "view";
const BASE: &str = "view://local/page";

/// Display a Web View, usually for Vega visualizations.
#[derive(Parser, Clone)]
struct Args {
    /// vega-lite specification for this visualization
    spec: String,

    /// file containing a HTML template for the page
    #[arg(long)]
    page: Option<PathBuf>,

    /// file containing data to visualize (default is stdin)
    #[arg(long)]
    data: Option<PathBuf>,

    /// The window title.
    #[arg(long)]
    title: Option<String>,

    /// The window width.
    #[arg(long)]
    width: Option<u32>,

    /// The window height.
    #[arg(long)]
    height: Option<u32>,
}

fn main() -> wry::Result<()> {
    let args = Args::parse();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(args.title.as_deref().unwrap_or("Vega View"))
        .with_inner_size(PhysicalSize::new(
            args.width.unwrap_or(1000),
            args.height.unwrap_or(800),
        ))
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();
    let _webview = WebViewBuilder::new(&window)
        .with_custom_protocol(SCHEME.to_string(), move |r| handler(&args, r))
        .with_url(BASE)
        .with_devtools(true)
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                // println!("Wry has started!")
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                // println!("Window closed!")
            }
            _ => (),
        }
    });
}

fn handler(args: &Args, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    // println!("{request:?}");
    match *request.method() {
        Method::GET => match request.uri().path() {
            "/page" => {
                let body = if let Some(path) = &args.page {
                    Cow::from(file_contents(path.as_path()))
                } else {
                    Cow::from(PAGE)
                };
                Response::builder()
                    .header("Content-Type", "text/html")
                    .body(body)
                    .unwrap()
            }
            "/spec" => {
                let body = Cow::from(args.spec.clone().into_bytes());
                Response::builder()
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap()
            }
            "/data" => {
                let body = if let Some(path) = &args.data {
                    Cow::from(file_contents(path.as_path()))
                } else {
                    Cow::from(all_input())
                };
                Response::builder()
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap()
            }
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Cow::from("Not found".as_bytes()))
                .unwrap(),
        },
        _ => Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Cow::from("Wrong method".as_bytes()))
            .unwrap(),
    }
}

fn all_input() -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    let _n = stdin().read_to_end(&mut buf).expect("unable to read stdin");
    // println!("data length = {_n}");
    buf
}

fn file_contents(path: &Path) -> Vec<u8> {
    let mut handle = File::open(path).expect("file not found");
    let mut buf = Vec::<u8>::new();
    handle.read_to_end(&mut buf).expect("unable to read file");
    buf
}

const PAGE: &[u8] = br#"
<!doctype html>
<html>
    <head>
        <meta charset='utf-8' />
        <script src='https://cdn.jsdelivr.net/npm/vega@5.27.0'></script>
        <script src='https://cdn.jsdelivr.net/npm/vega-lite@5.17.0'></script>
        <script src='https://cdn.jsdelivr.net/npm/vega-embed@6.24.0'></script>
        <style>
            #vis { width: 100% }
        </style>
    </head>
    <body>
        <div id='vis'></div>
        <script  type="text/javascript">
            vegaEmbed('#vis', '/spec', { actions: false })
        </script>
    </body>
</html>
"#;
