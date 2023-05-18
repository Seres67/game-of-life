use std::ops::{Deref, DerefMut};
use sdl2::pixels::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Clone)]
pub struct Board
{
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Board
{
    pub fn new(width: u32, height: u32) -> Board
    {
        let mut cells = Vec::new();
        for _ in 0..width * height
        {
            cells.push(Cell::Dead);
        }
        Board { width, height, cells }
    }

    pub fn update(&mut self)
    {
        let cells = self.cells.clone();

        let mut x = 0;
        let mut y = 0;
        for cell in &cells {
            let mut alive_neighbors = 0;
            if x > 0 && y > 0 && cells[(x - 1) + (y - 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if y > 0 && cells[x + (y - 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if x < (self.width - 1) as usize && y > 0 && cells[(x + 1) + (y - 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if x > 0 && cells[(x - 1) + y * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if x < (self.width - 1) as usize && cells[(x + 1) + y * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if x > 0 && y < (self.height - 1) as usize && cells[(x - 1) + (y + 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if y < (self.height - 1) as usize && cells[x + (y + 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }
            if x < (self.width - 1) as usize && y < (self.height - 1) as usize && cells[(x + 1) + (y + 1) * self.width as usize] == Cell::Alive
            {
                alive_neighbors += 1;
            }

            if *cell == Cell::Alive
            {
                if !(2..=3).contains(&alive_neighbors)
                {
                    self.cells[x + y * self.width as usize] = Cell::Dead;
                }
            } else if alive_neighbors == 3
            {
                self.cells[x + y * self.width as usize] = Cell::Alive;
            }
            if x == (self.width - 1) as usize
            {
                x = 0;
                y += 1;
            } else {
                x += 1;
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, width: i32, square_size: i32)
    {
        let mut x = 0;
        let mut y = 0;
        for cell in self.clone() {
            match cell {
                Cell::Dead => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.fill_rect(sdl2::rect::Rect::new(x, y, 10, 10)).unwrap();
                }
                Cell::Alive => {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(sdl2::rect::Rect::new(x, y, 10, 10)).unwrap();
                }
            }
            x += square_size;
            if x >= width {
                x = 0;
                y += square_size;
            }
        }
    }
}

impl IntoIterator for Board {
    type Item = Cell;
    type IntoIter = <Vec<Cell> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl Deref for Board {
    type Target = [Cell];

    fn deref(&self) -> &[Cell] {
        &self.cells[..]
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut [Cell] {
        &mut self.cells[..]
    }
}