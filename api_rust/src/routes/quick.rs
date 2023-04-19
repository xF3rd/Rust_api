use actix_web::*;
use rand::Rng;
use std::time::{Instant};

pub async fn quick() -> HttpResponse {
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(10000..1000000);
    let mut arr = Vec::with_capacity(len);
    for _i in 0..len {
        arr.push(rng.gen_range(20..2000000));
    }

    let n = arr.len();
    if n > 1 {
        let pivot = arr.pop().unwrap();
        let mut left = Vec::new();
        let mut right = Vec::new();

        for x in arr {
            if x <= pivot {
                left.push(x);
            } else {
                right.push(x);
            }
        }

        let mut res = Vec::new();
        res.append(&mut quick_sort_helper(left));
        res.push(pivot);
        res.append(&mut quick_sort_helper(right));
        arr = res;
    }
    let end = start.elapsed();
    HttpResponse::Ok().json(end)
}

fn quick_sort_helper(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n > 1 {
        let pivot = arr.pop().unwrap();
        let mut left = Vec::new();
        let mut right = Vec::new();

        for x in arr {
            if x <= pivot {
                left.push(x);
            } else {
                right.push(x);
            }
        }

        let mut res = Vec::new();
        res.append(&mut quick_sort_helper(left));
        res.push(pivot);
        res.append(&mut quick_sort_helper(right));
        arr = res;
    }

    arr
}