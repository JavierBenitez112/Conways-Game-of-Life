use std::thread;
use std::time::Duration;

use raylib::prelude::*;

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

struct GameOfLife {
    pub width: u32,
    pub height: u32,
    pub current_generation: Vec<Vec<bool>>,
    pub next_generation: Vec<Vec<bool>>,
    pub alive_color: Color,
    pub dead_color: Color,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let current_generation = vec![vec![false; height as usize]; width as usize];
        let next_generation = vec![vec![false; height as usize]; width as usize];
        
        GameOfLife {
            width,
            height,
            current_generation,
            next_generation,
            alive_color: Color::WHITE,
            dead_color: Color::BLACK,
        }
    }

    pub fn initialize_with_pattern(&mut self, pattern: &str) {
        match pattern {
            "glider" => self.init_glider(),
            "blinker" => self.init_blinker(),
            "toad" => self.init_toad(),
            "beacon" => self.init_beacon(),
            "random" => self.init_random(),
            _ => self.init_glider(),
        }
    }

    fn init_glider(&mut self) {
        let center_x = (self.width / 2) as usize;
        let center_y = (self.height / 2) as usize;
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
            }
        }
        
        if center_x + 2 < self.width as usize && center_y + 2 < self.height as usize {
            self.current_generation[center_x][center_y] = true;
            self.current_generation[center_x + 1][center_y + 1] = true;
            self.current_generation[center_x + 2][center_y + 1] = true;
            self.current_generation[center_x][center_y + 2] = true;
            self.current_generation[center_x + 1][center_y + 2] = true;
        }
    }

    fn init_blinker(&mut self) {
        let center_x = (self.width / 2) as usize;
        let center_y = (self.height / 2) as usize;
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
            }
        }
        
        if center_x + 2 < self.width as usize {
            self.current_generation[center_x][center_y] = true;
            self.current_generation[center_x + 1][center_y] = true;
            self.current_generation[center_x + 2][center_y] = true;
        }
    }

    fn init_toad(&mut self) {
        let center_x = (self.width / 2) as usize;
        let center_y = (self.height / 2) as usize;
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
            }
        }
        
        if center_x + 3 < self.width as usize && center_y + 1 < self.height as usize {
            self.current_generation[center_x + 1][center_y] = true;
            self.current_generation[center_x + 2][center_y] = true;
            self.current_generation[center_x + 3][center_y] = true;
            self.current_generation[center_x][center_y + 1] = true;
            self.current_generation[center_x + 1][center_y + 1] = true;
            self.current_generation[center_x + 2][center_y + 1] = true;
        }
    }

    fn init_beacon(&mut self) {
        let center_x = (self.width / 2) as usize;
        let center_y = (self.height / 2) as usize;
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
            }
        }
        
        if center_x + 3 < self.width as usize && center_y + 3 < self.height as usize {
            self.current_generation[center_x][center_y] = true;
            self.current_generation[center_x + 1][center_y] = true;
            self.current_generation[center_x][center_y + 1] = true;
            self.current_generation[center_x + 1][center_y + 1] = true;
            self.current_generation[center_x + 2][center_y + 2] = true;
            self.current_generation[center_x + 3][center_y + 2] = true;
            self.current_generation[center_x + 2][center_y + 3] = true;
            self.current_generation[center_x + 3][center_y + 3] = true;
        }
    }

    fn init_random(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let mut hasher = DefaultHasher::new();
                (x, y).hash(&mut hasher);
                let hash = hasher.finish();
                self.current_generation[x][y] = hash % 3 == 0;
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.current_generation[nx as usize][ny as usize] {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    pub fn update(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let neighbors = self.count_neighbors(x, y);
                let is_alive = self.current_generation[x][y];
                
                self.next_generation[x][y] = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, offset_x: u32, offset_y: u32, scale: u32) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let color = if self.current_generation[x][y] {
                    self.alive_color
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
}

// Este ejemplo demuestra el Juego de la Vida en alta resolución
// Útil para ver patrones más complejos y comportamientos emergentes

fn main() {
    let window_width = 1200;
    let window_height = 800;

    // Framebuffer más grande para alta resolución
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    // Grid más grande para más detalle
    let game_width = 200;
    let game_height = 150;
    let cell_scale = 3; // Mantener escala pequeña para ver más células

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Juego de la Vida - Alta Resolución - Controles: G(Glider), B(Blinker), T(Toad), A(Beacon), R(Random), ESPACIO(Pausar)")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);
    
    let mut game = GameOfLife::new(game_width, game_height);
    
    // Inicializar con patrón aleatorio para alta resolución
    game.initialize_with_pattern("random");

    let offset_x = (framebuffer_width - (game_width * cell_scale)) / 2;
    let offset_y = (framebuffer_height - (game_height * cell_scale)) / 2;

    let mut frame_count = 0;
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(50); // Más rápido para alta resolución
    let mut paused = false;

    println!("=== Juego de la Vida - Alta Resolución ===");
    println!("Grid: {}x{} células", game_width, game_height);
    println!("Controles:");
    println!("  G - Patrón Glider");
    println!("  B - Patrón Blinker");
    println!("  T - Patrón Toad");
    println!("  A - Patrón Beacon");
    println!("  R - Patrón Aleatorio");
    println!("  ESPACIO - Pausar/Reanudar");
    println!("  C - Limpiar grid");
    println!("================================");

    while !window.window_should_close() {
        let now = std::time::Instant::now();
        
        // Manejar controles
        if window.is_key_pressed(KeyboardKey::KEY_G) {
            game.initialize_with_pattern("glider");
            println!("Cambiado a patrón Glider");
        } else if window.is_key_pressed(KeyboardKey::KEY_B) {
            game.initialize_with_pattern("blinker");
            println!("Cambiado a patrón Blinker");
        } else if window.is_key_pressed(KeyboardKey::KEY_T) {
            game.initialize_with_pattern("toad");
            println!("Cambiado a patrón Toad");
        } else if window.is_key_pressed(KeyboardKey::KEY_A) {
            game.initialize_with_pattern("beacon");
            println!("Cambiado a patrón Beacon");
        } else if window.is_key_pressed(KeyboardKey::KEY_R) {
            game.initialize_with_pattern("random");
            println!("Cambiado a patrón Aleatorio");
        } else if window.is_key_pressed(KeyboardKey::KEY_C) {
            for x in 0..game_width as usize {
                for y in 0..game_height as usize {
                    game.current_generation[x][y] = false;
                }
            }
            println!("Grid limpiado");
        } else if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
            if paused {
                println!("Simulación pausada");
            } else {
                println!("Simulación reanudada");
            }
        }
        
        if !paused && now.duration_since(last_update) >= update_interval {
            game.update();
            last_update = now;
        }

        framebuffer.clear();
        game.render(&mut framebuffer, offset_x, offset_y, cell_scale);

        if frame_count % 50 == 0 { // Mostrar estadísticas más frecuentemente
            let (alive, total) = game.get_stats();
            let status = if paused { "PAUSADO" } else { "EJECUTANDO" };
            println!("Frame {} [{}]: {} células vivas de {} totales ({:.1}%)", 
                     frame_count, status, alive, total, 
                     (alive as f32 / total as f32) * 100.0);
        }

        framebuffer.swap_buffer(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
        
        frame_count += 1;
    }
} 