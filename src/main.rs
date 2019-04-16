#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/add/<num1>/<num2>")]
fn add(num1: i32, num2: i32) -> String {
    let s: String = math_utils::add(num1, num2).to_string();
    format!("The sum of {} and {} is {}", num1, num2, s)
}

fn main() {
    rocket::ignite().mount("/", routes![hello, add]).launch();
}
