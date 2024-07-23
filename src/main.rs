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
    // Blinker de esquinas
    (10,9),(10,10),(10,11),(11,10),
    (90,9),(90,10),(90,11),(89,10),
    // galaxia:
    (45, 26), (45, 27), (46, 27), (47, 22), (47, 23),
    (47, 24), (47, 27), (47, 28), (48, 20), (48, 21),
    (48, 22), (48, 24), (48, 28), (49, 20), (49, 25),
    (49, 27), (49, 28), (50, 24), (50, 26), (51, 22),
    (51, 23), (51, 25), (51, 30), (52, 22), (52, 26),
    (52, 28), (52, 29), (52, 30), (53, 22), (53, 23),
    (53, 26), (53, 27), (53, 28), (54, 23), (55, 23),
    (55, 24),

    (56,32),(57,32),(58,32),
    (57,33),(58,33),(59,33),

    (44, 32),
(43, 32),
(42, 32),
(43, 33),
(42, 33),
(41, 33),
    //penta-decathlon -13
    (10,30),
    (9,31),(10,31),(11,31),
    (9,34),(10,34),(11,34),
    (9,36),(11,36),
    (9,37),(11,37),
    (9,39),(10,39),(11,39),
    (9,42),(10,42),(11,42),
    (10,43),
    
    (90,30),
    (91,31),(90,31),(89,31),
    (91,34),(90,34),(89,34),
    (91,36),(89,36),
    (91,37),(89,37),
    (91,39),(90,39),(89,39),
    (91,42),(90,42),(89,42),
    (90,43),

        // carita feliz
        (47,69),(47,68),(48,70),(48,69),(49,70),(49,67),(49,66),(50,70),(51,70),(51,67),(51,66),(52,70),(52,69),(53,69),(53,68),
    
        // MWSS
        (21,20),(21,21),
        (22,17),(22,18),(22,19),(22,21),(22,22),
        (23,17),(23,18),(23,19),(23,20),(23,21),
        (24,18),(24,19),(24,20),
    
        (79,20),(79,21),
        (78,17),(78,18),(78,19),(78,21),(78,22),
        (77,17),(77,18),(77,19),(77,20),(77,21),
        (76,18),(76,19),(76,20),
    
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

    let frame_delay = Duration::from_millis(70);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Conway's game of life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let dead = 0x3798A5;
    let alive = 0xFF789D;
    framebuffer.set_bgcolor(dead);
    framebuffer.set_current_color(alive);
    framebuffer.clear();
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