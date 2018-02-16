extern crate blinkt;
extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;

use blinkt::Blinkt;

fn main() {
    let mut rng = rand::thread_rng();
    let mut blinkt = Blinkt::new().unwrap();

    blinkt.set_all_pixels_brightness(0.1);

    loop {
        for n in 0..8 {
            blinkt.set_pixel(n, rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());
        }
        blinkt.show();

        thread::sleep(Duration::from_millis(50));
    }
}
