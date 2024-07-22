use minifb::{Key, Window, WindowOptions};
use std::{  time::Duration};

mod color;
mod framebuffer;
mod bmp;

fn boundry(mut c: i32, max: i32)-> i32{
    if c > max {
      c = c-max-1;
    } else if c<0{
      c = max+c+1;
    }
    c
  }

fn render(framebuffer: &mut framebuffer::Framebuffer, alive: u32){
let mut alive_cells: Vec<(usize,usize)> = Vec::new();

  // Primero se registran cuales celulas estarían vivas en el siguiente turno
    for y in 0..framebuffer.height {
      for x in 0.. framebuffer.width {
        let cell = framebuffer.get_point(x as i32, y as i32);
        let mut is_alive = false;
        let mut neighbors = 0;
        if cell.to_hex() == alive
        {
          is_alive=true;
        }

        for i in 0..9{
          if i!= 4{
            if  framebuffer.get_point(
            boundry(x as i32 +(i%3)-1, framebuffer.width as i32-1), 
            boundry(y as i32 +(i/3)-1, framebuffer.height as i32-1)
            ).to_hex() == alive{
              neighbors+=1;
            }
          }
        }

        if neighbors > 3{
          alive_cells.retain(|&c| c != (x,y));
        } else if neighbors==3{
          alive_cells.push((x,y));
        } else if neighbors == 2 {
          if is_alive{
            alive_cells.push((x,y));
          }
        } else {
          alive_cells.retain(|&c| c != (x,y));
        }
      }
    }

    // luego con el array de alive cells, se hace un clear del 
    // framebuffer y se pintan las celulas calculadas que estan vivas
    framebuffer.clear();
    for cell in &alive_cells {
      framebuffer.point(cell.0, cell.1)
    }
}
fn init_state(framebuffer: &mut framebuffer::Framebuffer){
    let pulsar_points = vec![
    // carita feliz
    (47,89),(47,88),
    (48,90),(48,89),
    (49,90),(49,87),(49,86),
    (50,90),
    (51,90),(51,87),(51,86),
    (52,90),(52,89),
    (53,89),(53,88),
    ];

    for &(x, y) in &pulsar_points {
        framebuffer.point(x, y);
    }
}

fn main() {
    let window_width = 600;
    let window_height = 600;

    let framebuffer_width = 101;
    let framebuffer_height = 101;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Conway's game of life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let dead = 0x000000;
    let alive = 0xffffff;
    framebuffer.set_bgcolor(dead);
    framebuffer.set_current_color(alive);

    init_state(&mut framebuffer);

    let mut counter: usize = 0;
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::S){
          let _ = framebuffer.render("frame.bmp");
        }
        if counter > 3{

          render(&mut framebuffer, alive);
        }
        counter+=1;

        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}