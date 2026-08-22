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

    //tls.write_all(
    //    b"GET /cards/search?q=Asmora HTTP/1.1\r\n\
    //    Host: api.scryfall.com\r\n\
    //    User-Agent: rustls-demo/0.1\r\n\
    //    Accept: application/json\r\n\
    //    Connection: close\r\n\r\n",
    //);

    //let response = http::Response::read_from_stream(&mut tls).unwrap();
    //let fmt = format!("{:#?}", response);
    //std::fs::write("http_response.txt", fmt.as_bytes()).unwrap();
}

fn connect_to_api() -> 
    Result<StreamOwned<ClientConnection, TcpStream>, Box<dyn std::error::Error>> 
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
    Ok(StreamOwned::new(conn, tcp))
}

fn failed_to_connect_window(window: &gtk::ApplicationWindow) {
    let label = gtk::Label::new(Some("Failed to connect to Scryfall.\nFix it, you dumb fuck!"));
    window.set_child(Some(&label));
}

fn the_real_app<T>(stream: T, window: &gtk::ApplicationWindow) 
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

fn search_bar<T>(stream: Rc<RefCell<T>>, window: &ApplicationWindow) -> gtk::Box 
    where T: Read + Write + 'static
{
    let bar_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let bar_field = gtk::SearchEntry::builder()
        .hexpand(true)
        .build();

    let window = window.clone();
    bar_field.connect_activate(move |entry| {
        let label = gtk::Label::new(None);
        window.set_child(Some(&label));
        let mut inner_stream = stream.borrow_mut();
        println!("Searching for \"{}\"", entry.text());
        // use inner_stream here
    });

    bar_box.append(&bar_field);

    bar_box
}