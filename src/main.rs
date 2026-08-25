use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Widget};

use scryfall::http;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.shitty.scryfall")
        .build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Scryfall, but made shitty!")
            .default_width(640)
            .default_height(480)
            .build();

        match connect_to_api() {
            Err(_) => failed_to_connect_window(&window),
            Ok(stream) => the_real_app(stream, &window),
        }

        window.present();
    });

    app.run()
}

fn connect_to_api() -> 
    Result<http::Connection<StreamOwned<ClientConnection, TcpStream>>, Box<dyn std::error::Error>> 
{
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    
    let config = ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();

    let server_name = "api.scryfall.com".try_into()?;
    let config = Arc::new(config);

    let conn = ClientConnection::new(config, server_name)?;
    let tcp = TcpStream::connect("api.scryfall.com:443")?;
    let stream = StreamOwned::new(conn, tcp);

    let connection = http::Connection::new(
        stream,
        [
            ("Host".to_string(), "api.scryfall.com".to_string()),
            ("User-Agent".to_string(), "rustls-demo".to_string()),
            ("Accept".to_string(), "application/json".to_string()),
            ("Connection".to_string(), "close".to_string())
        ]
    );

    Ok(connection)
}

fn failed_to_connect_window(window: &gtk::ApplicationWindow) {
    let label = gtk::Label::new(Some("Failed to connect to Scryfall.\nFix it, you dumb fuck!"));
    window.set_child(Some(&label));
}

fn the_real_app<T>(stream: http::Connection<T>, window: &gtk::ApplicationWindow) 
    where T: Read + Write + 'static
{
    let stream = Rc::new(RefCell::new(stream));
    let rows = gtk::Box::new(gtk::Orientation::Vertical, 0);
    rows.append(&search_bar(stream, &window));

    let button = gtk::Button::new();
    button.set_vexpand(true);
    //rows.append(&button);

    window.set_child(Some(&rows));
}

fn search_bar<T>(stream: Rc<RefCell<http::Connection<T>>>, window: &ApplicationWindow) -> gtk::Box 
    where T: Read + Write + 'static
{
    let bar_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let bar_field = gtk::SearchEntry::builder()
        .hexpand(true)
        .build();

    let window = window.clone();
    bar_field.connect_activate(move |entry| {
        let mut inner_stream = stream.borrow_mut();
        
        let request = http::RestRequest::GET { 
            path: "/cards/search".to_string(), 
            parameters: Box::new([("q".to_string(), entry.text().to_string())])
        };

        let _ = inner_stream.send_rest_request(request);
        let res = inner_stream.read_response();
        let res = format!("{:#?}", res);
        std::fs::write("output.txt", res.as_bytes());
    });

    bar_box.append(&bar_field);

    bar_box
}