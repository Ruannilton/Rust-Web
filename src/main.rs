#![feature(decl_macro)]

use rocket::{self, get, routes};

#[get("/")]
fn root() -> &'static str {
    "Essa é a pagina raiz\nuse /hello ou /hello/<name> to hello page"
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello Rust"
}

#[get("/hello/<name>")]
fn print_name(name: String) -> String {
    format!("Hello {}", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![root, hello_world, print_name])
        .launch();
}
