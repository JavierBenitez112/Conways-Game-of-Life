use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use std::f32::consts::PI;

// Estructura para representar un color basado en matiz (hue)
#[derive(Clone, Copy, Debug)]
pub struct Hue {
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

pub struct GameOfLife {
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

    // Limpiar todo el grid
    pub fn clear_grid(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.current_generation[x][y] = false;
                self.current_colors[x][y] = None;
            }
        }
    }

    // Establecer el estado de una célula específica
    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width as usize && y < self.height as usize {
            self.current_generation[x][y] = alive;
            if alive {
                self.current_colors[x][y] = Some(Hue::random());
            } else {
                self.current_colors[x][y] = None;
            }
        }
    }

    // Establecer una célula con color específico
    pub fn set_cell_with_color(&mut self, x: usize, y: usize, alive: bool, hue: Option<Hue>) {
        if x < self.width as usize && y < self.height as usize {
            self.current_generation[x][y] = alive;
            self.current_colors[x][y] = hue;
        }
    }

    // Obtener el estado de una célula específica
    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        if x < self.width as usize && y < self.height as usize {
            self.current_generation[x][y]
        } else {
            false
        }
    }

    // Obtener el color de una célula específica
    pub fn get_cell_color(&self, x: usize, y: usize) -> Option<Hue> {
        if x < self.width as usize && y < self.height as usize {
            self.current_colors[x][y]
        } else {
            None
        }
    }

    // Calcular el promedio de matices (similar al Python)
    fn average_hue(&self, hues: &[Hue]) -> Hue {
        if hues.is_empty() {
            return Hue::random();
        }
        
        let mut x = 0.0;
        let mut y = 0.0;
        
        // Convertir cada hue a coordenadas en el círculo unitario
        for hue in hues {
            x += (hue.value * 2.0 * PI).cos();
            y += (hue.value * 2.0 * PI).sin();
        }
        
        x /= hues.len() as f32;
        y /= hues.len() as f32;
        
        // Convertir de vuelta a hue
        let angle = y.atan2(x);
        Hue::new(angle / (2.0 * PI))
    }

    // Contar vecinos vivos y obtener sus colores
    fn count_neighbors_and_colors(&self, x: usize, y: usize) -> (u8, Vec<Hue>) {
        let mut count = 0;
        let mut colors = Vec::new();
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Saltar la célula actual
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                // Verificar límites
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

    // Aplicar las reglas del Juego de la Vida con colores
    pub fn update(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let (neighbors, neighbor_colors) = self.count_neighbors_and_colors(x, y);
                let is_alive = self.current_generation[x][y];
                
                // Aplicar reglas de Conway
                let will_live = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true,  // Sobrevive
                    (false, 3) => true,             // Nace
                    _ => false,                     // Muere o permanece muerta
                };
                
                self.next_generation[x][y] = will_live;
                
                // Manejar colores
                if will_live {
                    if is_alive {
                        // Célula sobrevive, mantiene su color
                        self.next_colors[x][y] = self.current_colors[x][y];
                    } else {
                        // Nueva célula nace, hereda color de vecinos
                        if !neighbor_colors.is_empty() {
                            let mut new_hue = self.average_hue(&neighbor_colors);
                            
                            // Agregar variación de color
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
                    // Célula muere
                    self.next_colors[x][y] = None;
                }
            }
        }
        
        // Intercambiar generaciones
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
        std::mem::swap(&mut self.current_colors, &mut self.next_colors);
    }

    // Renderizar el estado actual en el framebuffer con colores
    pub fn render(&self, framebuffer: &mut Framebuffer, offset_x: u32, offset_y: u32, scale: u32) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let color = if self.current_generation[x][y] {
                    // Célula viva: usar color del hue o color por defecto
                    if let Some(hue) = self.current_colors[x][y] {
                        hue.to_color()
                    } else {
                        self.alive_color
                    }
                } else {
                    self.dead_color
                };
                
                framebuffer.set_current_color(color);
                
                // Dibujar cada célula como un cuadrado de tamaño 'scale'
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

    // Obtener estadísticas del juego
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

    // Configurar variación de color
    pub fn set_color_variation(&mut self, variation: f32) {
        self.color_variation = variation.max(0.0).min(1.0);
    }
}
