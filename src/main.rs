extern crate tokio;

use std::env::args;

use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    // fetch server address from first arg
    let addr = args().nth(1).unwrap_or("127.0.0.1:3000".to_string());

    // parse server address
    let addr = addr.parse().expect("unable to parse server address");

    // bind TCP listener to address
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
    println!("server successfully started at {}", &addr);

    // initialise listener
    let server = listener
        .incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            // split reader/writer of socket
            let (reader, writer) = sock.split();

            // obtain a future which will copy input to output
            let bytes_copied = copy(reader, writer);
            let handle_conn = bytes_copied
                .map(|_amt| {
                    // safely ignore number of bytes read on success
                })
                .map_err(|e| {
                    // report on error
                    eprintln!("IO error {:?}", e)
                });

            // spawn the socket to handle connections
            tokio::spawn(handle_conn)
        });

    // run the server with the given listener
    tokio::run(server)
}
