use std::thread;
use std::time::Duration;

use raylib::prelude::*;

// Este ejemplo demuestra cómo crear poblaciones personalizadas complejas
// Incluye patrones avanzados y comportamientos emergentes

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

    pub fn clear_grid(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
            }
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width as usize && y < self.height as usize {
            self.current_generation[x][y] = alive;
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
        .title("Juego de la Vida - Población Personalizada Avanzada")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);
    
    let mut game = GameOfLife::new(game_width, game_height);
    
    // ========================================
    // POBLACIÓN PERSONALIZADA AVANZADA
    // ========================================
    
    // Crear una población compleja y dinámica
    create_advanced_population(&mut game);

    let offset_x = (framebuffer_width - (game_width * cell_scale)) / 2;
    let offset_y = (framebuffer_height - (game_height * cell_scale)) / 2;

    let mut frame_count = 0;
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(80);

    println!("=== Población Personalizada Avanzada ===");
    println!("Grid: {}x{} células", game_width, game_height);
    println!("Población compleja con múltiples patrones interactuando");
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

// ========================================
// FUNCIONES PARA PATRONES AVANZADOS
// ========================================

fn create_advanced_population(game: &mut GameOfLife) {
    game.clear_grid();
    
    // 1. Línea de gliders en la parte superior
    for i in 0..8 {
        add_glider_at(game, 10 + i * 12, 5);
    }
    
    // 2. Región densa en el centro
    create_dense_region(game, 40, 30, 60, 50);
    
    // 3. Patrones osciladores en las esquinas
    add_blinker_at(game, 5, 5);
    add_blinker_at(game, 140, 5);
    add_blinker_at(game, 5, 90);
    add_blinker_at(game, 140, 90);
    
    // 4. Línea de beacons en el lado derecho
    for i in 0..5 {
        add_beacon_at(game, 120, 15 + i * 15);
    }
    
    // 5. Patrón de toads en el lado izquierdo
    for i in 0..4 {
        add_toad_at(game, 10, 20 + i * 18);
    }
    
    // 6. Región aleatoria en la parte inferior
    create_random_region(game, 20, 70, 130, 95);
    
    // 7. Gliders adicionales para movimiento
    add_glider_at(game, 80, 10);
    add_glider_at(game, 20, 60);
    add_glider_at(game, 100, 70);
    
    println!("Creada población avanzada con múltiples patrones");
}

fn create_dense_region(game: &mut GameOfLife, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..x2 {
        for y in y1..y2 {
            if (x + y) % 3 == 0 || (x * y) % 5 == 0 {
                game.set_cell(x, y, true);
            }
        }
    }
}

fn create_random_region(game: &mut GameOfLife, x1: usize, y1: usize, x2: usize, y2: usize) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    for x in x1..x2 {
        for y in y1..y2 {
            let mut hasher = DefaultHasher::new();
            (x, y, 42).hash(&mut hasher); // Semilla fija para reproducibilidad
            let hash = hasher.finish();
            if hash % 4 == 0 {
                game.set_cell(x, y, true);
            }
        }
    }
}

fn add_glider_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        game.set_cell(x, y, true);
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 2, y + 1, true);
        game.set_cell(x, y + 2, true);
        game.set_cell(x + 1, y + 2, true);
    }
}

fn add_blinker_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 2 < game.width as usize {
        game.set_cell(x, y, true);
        game.set_cell(x + 1, y, true);
        game.set_cell(x + 2, y, true);
    }
}

fn add_toad_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 3 < game.width as usize && y + 1 < game.height as usize {
        game.set_cell(x + 1, y, true);
        game.set_cell(x + 2, y, true);
        game.set_cell(x + 3, y, true);
        game.set_cell(x, y + 1, true);
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 2, y + 1, true);
    }
}

fn add_beacon_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 3 < game.width as usize && y + 3 < game.height as usize {
        game.set_cell(x, y, true);
        game.set_cell(x + 1, y, true);
        game.set_cell(x, y + 1, true);
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 2, y + 2, true);
        game.set_cell(x + 3, y + 2, true);
        game.set_cell(x + 2, y + 3, true);
        game.set_cell(x + 3, y + 3, true);
    }
} 