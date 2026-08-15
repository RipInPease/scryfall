use std::io:: Write;
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Widget};

use scryfall::http;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_app();
    
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    
    let config = ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();

    let server_name = "api.scryfall.com".try_into()?;
    let config = Arc::new(config);

    let conn = ClientConnection::new(config, server_name)?;

    let tcp = TcpStream::connect("api.scryfall.com:443")?;
    let mut tls = StreamOwned::new(conn, tcp);

    tls.write_all(
        b"GET /cards/search?q=Asmora HTTP/1.1\r\n\
        Host: api.scryfall.com\r\n\
        User-Agent: rustls-demo/0.1\r\n\
        Accept: application/json\r\n\
        Connection: close\r\n\r\n",
    )?;

    //let response = http::Response::read_from_stream(&mut tls).unwrap();
    //let fmt = format!("{:#?}", response);
    //std::fs::write("http_response.txt", fmt.as_bytes()).unwrap();

    Ok(())
}

fn start_app() -> glib::ExitCode {
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

        let rows = gtk::Box::new(gtk::Orientation::Vertical, 0);
        rows.append(&search_bar());

        let button = gtk::Button::new();
        button.set_vexpand(true);
        rows.append(&button);

        window.set_child(Some(&rows));
        window.present();
    });

    app.run()
}

fn search_bar() -> gtk::Box {
    let bar_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let bar_field = gtk::SearchEntry::builder()
        .hexpand(true)
        .build();

    bar_field.connect_activate(|entry| {
        println!("Searching for \"{}\"", entry.text());
    });

    bar_box.append(&bar_field);

    bar_box
}