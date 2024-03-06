use std::env;
use chip8_core::*;
use sdl2::event::Event;
use std::process;
use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::io::Read;

const SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn draw_screen(emulator: &Emulator, canvas: &mut Canvas<Window>){
    // Make and clear canvas to black
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    let screen_buffer = emulator.get_display();
    canvas.set_draw_color(Color::RGB(255,255,255));
    // Set drawing color to white to iterate over each point and see if it must be drawn
    for (i,pixel) in screen_buffer.iter().enumerate(){
        if *pixel{
            // Convert to get 1D array to 2D position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            

            // Draw rectangle at (x,y) scaled by the scale factor
            let rect = Rect::new((x*SCALE) as i32, (y*SCALE) as i32,SCALE,SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
    
}


fn main() -> Result<(),String>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Usage : cargo run -- <rom_name>");
        process::exit(1);
    }

    let mut buffer = Vec::new();
    let mut rom = File::open(&args[1]).map_err(|e| e.to_string())?;
    rom.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Chip8-Emulator",WINDOW_WIDTH,WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut chip8 = Emulator::new();
    chip8.load_game(&buffer);
    
    'gameloop : loop{
        for evt in event_pump.poll_iter(){
            match evt{
                Event::Quit{..} =>{
                    break 'gameloop;
                },
                _ => ()
            }
        }
        chip8.tick();
        draw_screen(&chip8,&mut canvas);
    }



    
    Ok(())


}
