use std::thread;
use std::time::Duration;

use raylib::prelude::*;

// Este ejemplo demuestra el sistema de colores del Juego de la Vida
// Las células heredan colores de sus vecinos y evolucionan con variaciones

// Definir las estructuras necesarias para el ejemplo
struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub current_color: Color,
    pub color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            background_color,
            current_color: Color::WHITE,
            color_buffer,
        }
    }
    
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }
    
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }
    
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    
    pub fn swap_buffer(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut render = window.begin_drawing(raylib_thread);
            render.draw_texture(&texture, 0, 0, Color::WHITE)
        }
    }
}

// Estructura para representar un color basado en matiz (hue)
#[derive(Clone, Copy, Debug)]
struct Hue {
    pub value: f32, // Valor entre 0.0 y 1.0
}

impl Hue {
    pub fn new(value: f32) -> Self {
        Hue {
            value: value.rem_euclid(1.0), // Mantener entre 0.0 y 1.0
        }
    }

    pub fn random() -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let mut hasher = DefaultHasher::new();
        time.hash(&mut hasher);
        let hash = hasher.finish();
        
        Hue::new((hash as f32) / (u64::MAX as f32))
    }

    pub fn to_color(&self) -> Color {
        // Convertir hue a RGB usando el algoritmo HSV
        let h = self.value * 6.0;
        let c = 1.0; // Saturación
        let v = 1.0; // Valor
        
        let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
        let m = v - c;
        
        let (r, g, b) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        Color::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            255
        )
    }
}

impl std::ops::Add for Hue {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Hue::new(self.value + other.value)
    }
}

struct GameOfLife {
    pub width: u32,
    pub height: u32,
    pub current_generation: Vec<Vec<bool>>,
    pub next_generation: Vec<Vec<bool>>,
    pub current_colors: Vec<Vec<Option<Hue>>>,
    pub next_colors: Vec<Vec<Option<Hue>>>,
    pub alive_color: Color,
    pub dead_color: Color,
    pub color_variation: f32,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let current_generation = vec![vec![false; height as usize]; width as usize];
        let next_generation = vec![vec![false; height as usize]; width as usize];
        let current_colors = vec![vec![None; height as usize]; width as usize];
        let next_colors = vec![vec![None; height as usize]; width as usize];
        
        GameOfLife {
            width,
            height,
            current_generation,
            next_generation,
            current_colors,
            next_colors,
            alive_color: Color::WHITE,
            dead_color: Color::BLACK,
            color_variation: 0.05, // Variación de color por defecto
        }
    }

    pub fn clear_grid(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
                self.current_colors[x][y] = None;
            }
        }
    }

    pub fn set_cell_with_color(&mut self, x: usize, y: usize, alive: bool, hue: Option<Hue>) {
        if x < self.width as usize && y < self.height as usize {
            self.current_generation[x][y] = alive;
            self.current_colors[x][y] = hue;
        }
    }

    fn average_hue(&self, hues: &[Hue]) -> Hue {
        if hues.is_empty() {
            return Hue::random();
        }
        
        let mut x = 0.0;
        let mut y = 0.0;
        
        // Convertir cada hue a coordenadas en el círculo unitario
        for hue in hues {
            x += (hue.value * 2.0 * std::f32::consts::PI).cos();
            y += (hue.value * 2.0 * std::f32::consts::PI).sin();
        }
        
        x /= hues.len() as f32;
        y /= hues.len() as f32;
        
        // Convertir de vuelta a hue
        let angle = y.atan2(x);
        Hue::new(angle / (2.0 * std::f32::consts::PI))
    }

    fn count_neighbors_and_colors(&self, x: usize, y: usize) -> (u8, Vec<Hue>) {
        let mut count = 0;
        let mut colors = Vec::new();
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if self.current_generation[nx][ny] {
                        count += 1;
                        if let Some(hue) = self.current_colors[nx][ny] {
                            colors.push(hue);
                        }
                    }
                }
            }
        }
        
        (count, colors)
    }

    pub fn update(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let (neighbors, neighbor_colors) = self.count_neighbors_and_colors(x, y);
                let is_alive = self.current_generation[x][y];
                
                let will_live = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                
                self.next_generation[x][y] = will_live;
                
                if will_live {
                    if is_alive {
                        self.next_colors[x][y] = self.current_colors[x][y];
                    } else {
                        if !neighbor_colors.is_empty() {
                            let mut new_hue = self.average_hue(&neighbor_colors);
                            
                            if self.color_variation > 0.0 {
                                use std::collections::hash_map::DefaultHasher;
                                use std::hash::{Hash, Hasher};
                                
                                let mut hasher = DefaultHasher::new();
                                (x, y).hash(&mut hasher);
                                let hash = hasher.finish();
                                let variation = (hash as f32 / u64::MAX as f32) * 2.0 - 1.0;
                                
                                new_hue = Hue::new(new_hue.value + variation * self.color_variation);
                            }
                            
                            self.next_colors[x][y] = Some(new_hue);
                        } else {
                            self.next_colors[x][y] = Some(Hue::random());
                        }
                    }
                } else {
                    self.next_colors[x][y] = None;
                }
            }
        }
        
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
        std::mem::swap(&mut self.current_colors, &mut self.next_colors);
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, offset_x: u32, offset_y: u32, scale: u32) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let color = if self.current_generation[x][y] {
                    if let Some(hue) = self.current_colors[x][y] {
                        hue.to_color()
                    } else {
                        self.alive_color
                    }
                } else {
                    self.dead_color
                };
                
                framebuffer.set_current_color(color);
                
                for sx in 0..scale {
                    for sy in 0..scale {
                        let pixel_x = offset_x + (x as u32 * scale) + sx;
                        let pixel_y = offset_y + (y as u32 * scale) + sy;
                        framebuffer.set_pixel(pixel_x, pixel_y);
                    }
                }
            }
        }
    }

    pub fn get_stats(&self) -> (u32, u32) {
        let mut alive_count = 0;
        let total_cells = self.width * self.height;
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                if self.current_generation[x][y] {
                    alive_count += 1;
                }
            }
        }
        
        (alive_count, total_cells)
    }

    pub fn set_color_variation(&mut self, variation: f32) {
        self.color_variation = variation.max(0.0).min(1.0);
    }
}

fn main() {
    let window_width = 1000;
    let window_height = 700;

    let framebuffer_width = 600;
    let framebuffer_height = 450;

    let game_width = 150;
    let game_height = 100;
    let cell_scale = 3;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Juego de la Vida Colorido - Células que Heredan Colores")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);
    
    let mut game = GameOfLife::new(game_width, game_height);
    
    // Configurar variación de color
    game.set_color_variation(0.08); // Más variación para más diversidad
    
    // Crear población inicial con diferentes colores
    create_colorful_population(&mut game);

    let offset_x = (framebuffer_width - (game_width * cell_scale)) / 2;
    let offset_y = (framebuffer_height - (game_height * cell_scale)) / 2;

    let mut frame_count = 0;
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(100);

    println!("=== Juego de la Vida Colorido ===");
    println!("Grid: {}x{} células", game_width, game_height);
    println!("Las células heredan colores de sus vecinos");
    println!("Variación de color: {:.2}", game.color_variation);
    println!("================================");

    while !window.window_should_close() {
        let now = std::time::Instant::now();
        
        if now.duration_since(last_update) >= update_interval {
            game.update();
            last_update = now;
        }

        framebuffer.clear();
        game.render(&mut framebuffer, offset_x, offset_y, cell_scale);

        if frame_count % 50 == 0 {
            let (alive, total) = game.get_stats();
            println!("Frame {}: {} células vivas de {} totales ({:.1}%)", 
                     frame_count, alive, total, 
                     (alive as f32 / total as f32) * 100.0);
        }

        framebuffer.swap_buffer(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
        
        frame_count += 1;
    }
}

fn create_colorful_population(game: &mut GameOfLife) {
    game.clear_grid();
    
    // Crear diferentes regiones con colores específicos
    create_color_region(game, 20, 20, 40, 40, Hue::new(0.0));      // Rojo
    create_color_region(game, 80, 20, 100, 40, Hue::new(0.33));    // Verde
    create_color_region(game, 20, 60, 40, 80, Hue::new(0.66));     // Azul
    create_color_region(game, 80, 60, 100, 80, Hue::new(0.17));    // Amarillo
    
    // Agregar algunos gliders coloridos
    add_colorful_glider(game, 50, 10, Hue::new(0.83));  // Magenta
    add_colorful_glider(game, 10, 50, Hue::new(0.5));   // Cian
    add_colorful_glider(game, 120, 50, Hue::new(0.08)); // Naranja
    
    // Agregar células aleatorias con colores
    for _ in 0..50 {
        let x = (rand::random::<f32>() * game.width as f32) as usize;
        let y = (rand::random::<f32>() * game.height as f32) as usize;
        let hue = Hue::new(rand::random::<f32>());
        game.set_cell_with_color(x, y, true, Some(hue));
    }
    
    println!("Creada población colorida con diferentes regiones y gliders");
}

fn create_color_region(game: &mut GameOfLife, x1: usize, y1: usize, x2: usize, y2: usize, hue: Hue) {
    for x in x1..x2 {
        for y in y1..y2 {
            if (x + y) % 3 == 0 {
                game.set_cell_with_color(x, y, true, Some(hue));
            }
        }
    }
}

fn add_colorful_glider(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        game.set_cell_with_color(x, y, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 1, true, Some(hue));
        game.set_cell_with_color(x, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 2, true, Some(hue));
    }
}

// Función simple de random para el ejemplo
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    
    pub fn random<T>() -> T 
    where
        T: Copy + From<f32>,
    {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let mut hasher = DefaultHasher::new();
        time.hash(&mut hasher);
        let hash = hasher.finish();
        
        let value = (hash as f32) / (u64::MAX as f32);
        T::from(value)
    }
} 