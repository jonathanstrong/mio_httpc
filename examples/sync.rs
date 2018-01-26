extern crate mio_httpc;

use mio_httpc::SyncCall;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let uri = if args.len() == 1 {
        "https://www.reddit.com"
    } else {
        args[1].as_str()
    };

    println!("Calling: {}",uri);

    let (status, hdrs, body) = SyncCall::new().timeout_ms(5000).get(uri).expect("Request failed");

    println!("Status={}", status);
    println!("Hdrs={:?}", hdrs);
    if let Ok(s) = String::from_utf8(body) {
        println!("Body: {}",s);
    }
}