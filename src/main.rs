use clap::Parser;
use std::{
    borrow::Cow,
    io::{stdin, Read},
};
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{Method, Request, Response, StatusCode},
    WebViewBuilder,
};

/// Display a Web View, usually for Vega graphs.
#[derive(Parser)]
struct Args {
    /// URL to display
    url: Option<String>,

    /// The window title.
    #[arg(long)]
    title: Option<String>,
}

fn main() -> wry::Result<()> {
    let args = Args::parse();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(args.title.unwrap_or("Vega View".to_string()))
        .build(&event_loop)
        .unwrap();
    let _webview = WebViewBuilder::new(&window)
        .with_custom_protocol("view".to_string(), handler)
        .with_url(args.url.unwrap_or("view:test".to_string()))
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                println!("Window closed!")
            }
            _ => (),
        }
    });
}

fn handler(request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    println!("{request:?}");
    match *request.method() {
        Method::GET => match request.uri().to_string().as_str() {
            "view:test" => Response::builder()
                .body(Cow::from("Hi there!".as_bytes()))
                .unwrap(),
            "view:stdin" => Response::builder().body(Cow::from(all_input())).unwrap(),
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
    stdin().read_to_end(&mut buf).expect("unable to read stdin");
    buf
}
