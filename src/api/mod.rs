use core::str::Utf8Error;
use std::path::PathBuf;
use std::ops::Deref;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use url::percent_encoding::percent_decode;

use collection;

pub fn get_api_handler() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/browse/", self::browse)
        .mount("/flatten/", self::flatten);
    mount
}

fn path_from_request(request: &Request) -> Result<PathBuf, Utf8Error> {
    let path_string = request.url.path().join("/");
    let decoded_path = try!(percent_decode(path_string.as_bytes()).decode_utf8());
    Ok(PathBuf::from(decoded_path.deref()))
}

fn browse(request: &mut Request) -> IronResult<Response> {
    let path = path_from_request(request);
    if path.is_err() {
        return Ok(Response::with((status::BadRequest)));
    }
    let browse_result = collection::browse(&path.unwrap());
    println!("{:?}", browse_result.unwrap_or(vec![])); // TMP
    Ok(Response::with((status::Ok, "TODO browse data here")))
}

fn flatten(request: &mut Request) -> IronResult<Response> {
    let path = path_from_request(request);
    if path.is_err() {
        return Ok(Response::with((status::BadRequest)));
    }
    collection::flatten(&path.unwrap());
    Ok(Response::with((status::Ok, "TODO Flatten data here")))
}
