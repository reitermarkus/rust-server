extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate chrono;

use iron::prelude::*;
use iron::{Handler, IronResult};

use staticfile::Static;
use mount::Mount;

use std::env;
use std::str::FromStr;

use std::path::Path;

static DEFAULT_PORT: u16 = 3000;
static DEFAULT_WEBROOT: &str = "/www";

fn main() {
  let port = match env::var("PORT") {
    Ok(val) => u16::from_str(&val).unwrap(),
    Err(_) => DEFAULT_PORT,
  };

  let webroot_str = env::var("WEBROOT").unwrap_or(String::from(DEFAULT_WEBROOT));
  let webroot = Path::new(&webroot_str);

  println!("Starting server on port {} in {:?} …", port, webroot);

  let handler = Static::new(webroot);

  let log_request = move |req: &mut Request| -> IronResult<Response> {
    let path = req.url.path().join("/");

    let time = chrono::Utc::now().to_rfc3339();
    println!("{} - {} /{}", time, req.method, path);
    handler.handle(req)
  };

  let mut mount = Mount::new();
  mount.mount("/", log_request);

  let bind_addr = format!("0.0.0.0:{}", port);
  Iron::new(mount).http(bind_addr).unwrap();
}
