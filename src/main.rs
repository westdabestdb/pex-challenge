use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;

struct Fibonacci {
    cache: Vec<u128>,
    current: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            cache: vec![0, 1],
            current: 0,
        }
    }

    pub fn next(&mut self) -> u128 {
        if self.current >= self.cache.len() - 1 {
            let next: u128 = self.cache[self.current] + self.cache[self.current - 1];
            self.cache.push(next);
        }
        self.current += 1;
        self.cache[self.current]
    }

    fn previous(&mut self) -> Option<u128> {
        if self.current > 0 {
            self.current -= 1;
            Some(self.cache[self.current])
        } else {
            None
        }
    }

    fn current(&self) -> u128 {
        self.cache[self.current]
    }
}

async fn next_fibonacci(fib: web::Data<Mutex<Fibonacci>>) -> impl Responder {
    let mut fib: std::sync::MutexGuard<'_, Fibonacci> = fib.lock().unwrap();
    fib.next().to_string()
}

async fn previous_fibonacci(fib: web::Data<Mutex<Fibonacci>>) -> impl Responder {
    let mut fib: std::sync::MutexGuard<'_, Fibonacci> = fib.lock().unwrap();
    fib.previous()
        .map(|n: u128| n.to_string())
        .unwrap_or_else(|| "0".to_string())
}

async fn current_fibonacci(fib: web::Data<Mutex<Fibonacci>>) -> impl Responder {
    let fib: std::sync::MutexGuard<'_, Fibonacci> = fib.lock().unwrap();
    fib.current().to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fibonacci: web::Data<Mutex<Fibonacci>> = web::Data::new(Mutex::new(Fibonacci::new()));

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET"])
            .allow_any_header();
        App::new()
            .app_data(fibonacci.clone())
            .wrap(cors)
            .route("/next", web::get().to(next_fibonacci))
            .route("/previous", web::get().to(previous_fibonacci))
            .route("/current", web::get().to(current_fibonacci))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
