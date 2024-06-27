#[macro_use]
extern crate rocket;
use std::time::Instant;

fn fibonacci(n: u8) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/fib/<n>")]
fn fib(n: u8) -> String {
    let start = Instant::now();
    let the_fibonacci_number = fibonacci(n);
    let duration = start.elapsed();
    format!(
        "Your number is: {} \nComputation took: {:?}",
        the_fibonacci_number, duration
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, fib])
}
