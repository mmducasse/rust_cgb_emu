use macroquad::{
    input::{is_key_pressed, KeyCode},
    window::next_frame,
};
use xf::{
    mq::window::{Window, WindowParams},
    num::ivec2::i2,
};

use crate::{consts::PIXEL_SCALE, debug::Debug, sys::Sys};

pub async fn run_blargg_test(rom_path: &str) {
    let window = Window::new(WindowParams {
        resolution: i2(256, 256),
        scale: PIXEL_SCALE,
    });

    let mut sys = Sys::new();
    Sys::initialize(&mut sys);

    sys.cart.load(rom_path);
    sys.debug.kill_after_cpu_ticks = Some(1_000_000);
    sys.debug.kill_after_nop_count = Some(64);
    sys.debug.enable_debug_print = false; //true;

    window.render_pass(|| {});
    next_frame().await;

    while !sys.hard_lock {
        sys.run_one();
        print_output_char(&sys);
    }

    println!("Done");
    Debug::print_system_state(&sys);

    while !is_key_pressed(KeyCode::Escape) {
        window.render_pass(|| {});
        next_frame().await;
    }
}

fn print_output_char(sys: &Sys) {
    if sys.mem_get(0xFF02u16) == 0x81 {
        let data = sys.mem_get(0xFF01u16);
        let c = char::from_u32(data as u32).unwrap_or('?');
        // if c.is_whitespace() {
        //     println!();
        // } else {
        //     print!("{}", c);
        // }
        println!("{:02x}", data);
    }
}
