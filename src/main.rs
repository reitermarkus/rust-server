extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate chrono;

use iron::prelude::*;
use iron::{Handler, IronResult};
use staticfile::Static;
use mount::Mount;

use chrono::Utc;

use std::env;
use std::str::FromStr;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::path::Path;

static DEFAULT_PORT: u16 = 3000;
static DEFAULT_WEBROOT: &str = "/www";

fn main() {
  let port = env::var("PORT").ok().and_then(|p| u16::from_str(&p).ok()).unwrap_or(DEFAULT_PORT);
  let webroot_str = env::var("WEBROOT").unwrap_or(DEFAULT_WEBROOT.to_string());
  let webroot = Path::new(&webroot_str);

  println!("Starting server on port {} in {} …", port, webroot.to_string_lossy());

  let handler = Static::new(webroot);

  let log_request = move |req: &mut Request| -> IronResult<Response> {
    let path = req.url.path().join("/");

    let time = Utc::now().to_rfc3339();
    println!("{} - {} /{}", time, req.method, path);
    handler.handle(req)
  };

  let mut mount = Mount::new();
  mount.mount("/", log_request);

  let bind_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
  Iron::new(mount).http(bind_addr).expect("Failed to start server");

  println!("Server started.");
}
