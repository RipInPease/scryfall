use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};

use scryfall::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    let mut f = File::open("src/test/uuid1.json").unwrap();

    let uuid = UUID::derialize(&mut f).unwrap();
    println!("{:#?}", uuid);

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
        b"GET /cards/search?q=o:%22loses%20all%20abilities%22 HTTP/1.1\r\n\
        Host: api.scryfall.com\r\n\
        User-Agent: rustls-demo/0.1\r\n\
        Accept: application/json\r\n\
        Connection: close\r\n\r\n",
    )?;

    let mut response = String::new();
    tls.read_to_string(&mut response)?;

    //println!("{response}");

    Ok(())
}