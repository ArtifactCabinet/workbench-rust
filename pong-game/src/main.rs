extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::EventPump;

// TODO: keep window boarder?
// TODO: read more examples
//

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_suqbsystem = sdl_context.video().unwrap();
 
    let window = video_suqbsystem.window("Pong game", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut timer = sdl_context.timer().unwrap();

    let x=0;
    let mut a = Rect::new(0,100,100,100);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rect(a);

        let ticks = timer.ticks() as i32;
        a.set_right(ticks/10);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();

        // Control the loop speed.
        // However, we use ticks based animation.
        //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}