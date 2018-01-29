extern crate rget;
use rget::http::{Request, Response};
fn main() {
    let mut r = Request::new("127.0.0.1:80");
    r.add_header("Accept".to_string(), "*/*".to_string());
    let h = String::from_utf8(r.build()).unwrap();
    print!("{}", h);
}
