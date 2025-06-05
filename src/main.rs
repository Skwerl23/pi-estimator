use rand::Rng;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    const B: f64 = 100_000_000.0;


    let threads = 8;
    let runs_per_thread = B as u32 / threads +1;
    let mut a = 0;
    {
        let mut rounds = vec![0; threads as usize];
        crossbeam::scope(|spawner| {
            for value in &mut rounds {
                spawner.spawn(move |_| {
                    let answer_join = sum_chunk(runs_per_thread);
                    *value = answer_join;
                });
            }

        }).unwrap();
        for i in rounds {
            a+= i;
        }
    }

    let end = Instant::now();
    
    let elapsed = end.duration_since(start);
    println!("Estimated pi: {}", a as f64 /B*4.0);
    println!("Rust Multithread Elapsed time: {:?}", elapsed);
    single_thread();
}


fn single_thread() {
    let start = Instant::now();
    const B: f64 = 100_000_000.0;
    let a = sum_chunk(B as u32);
    let end = Instant::now();

    let elapsed = end.duration_since(start);
    println!("Estimated pi: {}", a as f64 /B*4.0);
    println!("Rust Single Thread Elapsed time: {:?}", elapsed);
}



fn sum_chunk(b:u32) -> u32{
    let mut a = 0;
    for _ in 1..=b {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);
        if f64::sqrt(x*x + y*y) < 1.0 {
            a += 1;
        }
    }
    a
}
