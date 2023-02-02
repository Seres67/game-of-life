use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

mod game;

const SQUARE_SIZE: i32 = 10;
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Game of Life", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut board = game::board::Board::new((WIDTH / SQUARE_SIZE) as u32, (HEIGHT / SQUARE_SIZE) as u32);
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    let x = x / SQUARE_SIZE;
                    let y = y / SQUARE_SIZE;
                    board.cells[x as usize + y as usize * board.width as usize] = game::board::Cell::Alive;
                }
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Right, .. } => {
                    let x = x / SQUARE_SIZE;
                    let y = y / SQUARE_SIZE;
                    board.cells[x as usize + y as usize * board.width as usize] = game::board::Cell::Dead;
                }
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
                    running = !running;
                }
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
                    board = game::board::Board::new((WIDTH / SQUARE_SIZE) as u32, (HEIGHT / SQUARE_SIZE) as u32);
                }
                _ => {}
            }
        }

        if running {
            board.update();
        }
        board.draw(&mut canvas, WIDTH, SQUARE_SIZE);
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
