use actix_web::{web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use num_bigint::BigUint;
use redis::Commands;
use std::sync::Arc;

struct Fibonacci {
    redis: Arc<redis::Client>,
}

impl Fibonacci {
    fn new(redis: Arc<redis::Client>) -> Self {
        let fib = Fibonacci { redis };

        let mut con: redis::Connection = fib.redis.get_connection().unwrap();

        let i0 = con.get("0").unwrap_or("0".to_string());
        let i1 = con.get("1").unwrap_or("1".to_string());
        let curr = con.get("curr").unwrap_or("0".to_string());

        if i0 == "0" && i1 == "1" && curr == "0" {
            let _: () = con.set("0", 0).unwrap();
            let _: () = con.set("1", 1).unwrap();
            let _: () = con.set("curr", 0).unwrap();
        }

        fib
    }

    pub fn next(&self) -> String {
        let mut con: redis::Connection = self.redis.get_connection().unwrap();
        let current_str: String = con.get("curr").unwrap_or("0".to_string());
        let current: BigUint = current_str.parse().unwrap();

        let next_key: &str = &(current.clone() + BigUint::from(1u32)).to_string();

        // Check if the next key exists
        if let Ok(next) = con.get(&next_key) {
            let _: () = con
                .set("curr", (current.clone() + BigUint::from(1u32)).to_string())
                .unwrap();
            next
        } else {
            let previous_key: &str = &(current.clone() - BigUint::from(1u32)).to_string();

            let previous_str: String = con.get(previous_key).unwrap_or("0".to_string());
            let current_str: String = con.get(&current.to_string()).unwrap_or("0".to_string());

            let previous: BigUint = previous_str.parse().unwrap();
            let current: BigUint = current_str.parse().unwrap();

            let next: BigUint = &previous + &current;

            let _: () = con.set(next_key, next.to_string()).unwrap();

            next.to_string()
        }
    }

    pub fn previous(&self) -> Option<u64> {
        let mut con: redis::Connection = self.redis.get_connection().unwrap();
        let current: u64 = con.get("curr").unwrap();

        if current > 1 {
            let prev_key: String = (current - 1).to_string();
            if let Ok(prev) = con.get(&prev_key) {
                let _: () = con.set("curr", current - 1).unwrap();
                Some(prev)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<String> {
        let mut con: redis::Connection = self.redis.get_connection().unwrap();

        let current_str: String = con.get("curr").unwrap_or("0".to_string());
        let curr = con.get(&current_str).unwrap_or("0".to_string());

        Some(curr)
    }
}

lazy_static! {
    static ref REDIS_CLIENT: Arc<redis::Client> = redis::Client::open("redis://127.0.0.1:6379")
        .unwrap()
        .into();
}

async fn next_fibonacci(fib: web::Data<Fibonacci>) -> impl Responder {
    fib.next().to_string()
}

async fn previous_fibonacci(fib: web::Data<Fibonacci>) -> impl Responder {
    fib.previous()
        .map(|n: u64| n.to_string())
        .unwrap_or_else(|| "0".to_string())
}

async fn current_fibonacci(fib: web::Data<Fibonacci>) -> impl Responder {
    fib.current().unwrap_or_else(|| "0".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fibonacci: web::Data<Fibonacci> = web::Data::new(Fibonacci::new(REDIS_CLIENT.clone()));

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
