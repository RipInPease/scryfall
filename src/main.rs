use std::io:: Write;
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

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

    let response = http::Response::read_from_stream(&mut tls).unwrap();
    let fmt = format!("{:#?}", response);
    std::fs::write("http_response.txt", fmt.as_bytes()).unwrap();

    Ok(())
}

fn start_app() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Scryfall, but made shitty!")
        .build();

    app.connect_activate(|app| {
       let window = ApplicationWindow::builder()
       .application(app)
        .default_height(480)
        .default_width(640)
        .build();

        window.present();
    });

    app.run()
}