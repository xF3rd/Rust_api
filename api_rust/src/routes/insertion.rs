use actix_web::*;
use rand::Rng;
use std::time::{Instant};


pub async fn insertion() -> HttpResponse {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(10000..10001);
    let mut arr = Vec::with_capacity(len);
    for _i in 0..len {
        arr.push(rng.gen_range(20..2000000));
    }

    let start = Instant::now();

    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
    let end = start.elapsed();
    HttpResponse::Ok().json(end)
}