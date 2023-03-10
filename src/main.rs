use rand::Rng;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
mod jay_patel;

fn main() {
    let mut rng_gen = rand::thread_rng();
    loop {
        let rng = rng_gen.gen();
        let this_jay = jay_patel::randomize_jay(rng);
        print!("{this_jay}");
        stdout().flush().unwrap();
        sleep(Duration::from_millis(70 * (rng % 10)));
    }
}
