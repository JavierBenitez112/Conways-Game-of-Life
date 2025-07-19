// Spaceship (LWSS) patrón clásico
fn add_spaceship_lwss(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    // Patrón LWSS (5x4)
    let pattern = [
        [0, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 0, 0, 1, 0],
    ];
    for i in 0..4 {
        for j in 0..5 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + j, y + i, true, Some(hue));
            }
        }
    }
}
use std::thread;
use std::time::Duration;

use framebuffer::Framebuffer;
use game_of_life::{GameOfLife, Hue};
use raylib::prelude::*;

mod framebuffer;
mod game_of_life;
mod line;

fn main() {
    let window_width = 800;
    let window_height = 600;

    // Configuración del framebuffer (más pequeño que la ventana para mejor rendimiento)
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    // Configuración del juego
    let game_width = 240; // Más grande para más patrones
    let game_height = 180;
    let cell_scale = 3; // Cada célula será un cuadrado de 2x2 píxeles

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Juego de la Vida de Conway - Jardin")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);

    // Crear el juego
    let mut game = GameOfLife::new(game_width, game_height);

    // Configurar variación de color (similar al Python)
    game.set_color_variation(0.05);

    // ========================================
    // AQUÍ PUEDES AGREGAR TUS PATRONES
    // ========================================

    // Ejemplo 1: Múltiples gliders
    // add_multiple_gliders(&mut game);

    // Ejemplo 2: Patrón aleatorio
    // game.initialize_with_pattern("random");

    // Ejemplo 3: Combinación de patrones
    // add_pattern_combination(&mut game);

    // Ejemplo 4: Población densa
    // add_dense_population(&mut game);

    // Ejemplo 5: Patrones en las esquinas
    // add_corner_patterns(&mut game);

    // Ejemplo 6: Flores
    add_flowers(&mut game);

    // Calcular offset para centrar el juego en el framebuffer
    let offset_x = (framebuffer_width - (game_width * cell_scale)) / 2;
    let offset_y = (framebuffer_height - (game_height * cell_scale)) / 2;

    let mut frame_count = 0;
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(100); // 10 FPS para mejor visualización

    println!("=== Juego de la Vida de Conway ===");
    println!("Grid: {}x{} células", game_width, game_height);
    println!("Modifica el código para cambiar los patrones!");
    println!("================================");

    while !window.window_should_close() {
        let now = std::time::Instant::now();

        // Actualizar el juego cada cierto intervalo
        if now.duration_since(last_update) >= update_interval {
            game.update();
            last_update = now;
        }

        // Limpiar el framebuffer
        framebuffer.clear();

        // Renderizar el juego
        game.render(&mut framebuffer, offset_x, offset_y, cell_scale);

        // Mostrar estadísticas en la consola cada 100 frames
        if frame_count % 100 == 0 {
            let (alive, total) = game.get_stats();
            println!(
                "Frame {}: {} células vivas de {} totales ({:.1}%)",
                frame_count,
                alive,
                total,
                (alive as f32 / total as f32) * 100.0
            );
        }

        // Intercambiar buffer y mostrar
        framebuffer.swap_buffer(&mut window, &raylib_thread);

        // Pequeño delay para controlar la velocidad
        thread::sleep(Duration::from_millis(16)); // ~60 FPS para la ventana

        frame_count += 1;
    }
}

// ========================================
// FUNCIONES PARA AGREGAR PATRONES
// ========================================

/// Agrega múltiples gliders en diferentes posiciones
fn add_multiple_gliders(game: &mut GameOfLife) {
    // Limpiar el grid
    game.clear_grid();

    // Glider 1 - Centro (rojo)
    add_glider_at_with_color(game, 20, 20, Hue::new(0.0)); // Rojo

    // Glider 2 - Esquina superior izquierda (verde)
    add_glider_at_with_color(game, 5, 5, Hue::new(0.33)); // Verde

    // Glider 3 - Esquina inferior derecha (azul)
    add_glider_at_with_color(game, 70, 50, Hue::new(0.66)); // Azul

    // Glider 4 - Lado izquierdo (amarillo)
    add_glider_at_with_color(game, 10, 40, Hue::new(0.17)); // Amarillo

    // Glider 5 - Lado derecho (magenta)
    add_glider_at_with_color(game, 80, 15, Hue::new(0.83)); // Magenta

    println!("Agregados 5 gliders coloridos en diferentes posiciones");
}

/// Agrega una combinación de diferentes patrones
fn add_pattern_combination(game: &mut GameOfLife) {
    // Limpiar el grid
    game.clear_grid();

    // Glider en el centro
    add_glider_at(game, 30, 30);

    // Blinker en la esquina superior izquierda
    add_blinker_at(game, 10, 10);

    // Toad en la esquina inferior derecha
    add_toad_at(game, 70, 60);

    // Beacon en el lado izquierdo
    add_beacon_at(game, 15, 40);

    println!("Agregada combinación de patrones");
}

/// Agrega una población densa aleatoria
fn add_dense_population(game: &mut GameOfLife) {
    // Limpiar el grid
    game.clear_grid();

    // Agregar células aleatorias en una región densa
    for x in 20..60 {
        for y in 20..50 {
            if (x + y) % 3 == 0 || (x * y) % 7 == 0 {
                game.set_cell(x, y, true);
            }
        }
    }

    // Agregar algunos gliders para movimiento
    add_glider_at(game, 70, 10);
    add_glider_at(game, 10, 60);

    println!("Agregada población densa con gliders");
}

/// Agrega patrones en las esquinas del grid
fn add_corner_patterns(game: &mut GameOfLife) {
    // Limpiar el grid
    game.clear_grid();

    // Esquina superior izquierda - Blinker
    add_blinker_at(game, 5, 5);

    // Esquina superior derecha - Glider
    add_glider_at(game, 85, 5);

    // Esquina inferior izquierda - Toad
    add_toad_at(game, 5, 65);

    // Esquina inferior derecha - Beacon
    add_beacon_at(game, 80, 65);

    // Centro - Glider
    add_glider_at(game, 45, 35);

    println!("Agregados patrones en las esquinas y centro");
}

// ========================================
// FUNCIONES AUXILIARES PARA PATRONES
// ========================================

fn add_glider_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        game.set_cell(x, y, true);
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 2, y + 1, true);
        game.set_cell(x, y + 2, true);
        game.set_cell(x + 1, y + 2, true);
    }
}

fn add_glider_at_with_color(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        game.set_cell_with_color(x, y, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 1, true, Some(hue));
        game.set_cell_with_color(x, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 2, true, Some(hue));
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

fn add_small_flower_at_with_color(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        // Centro
        game.set_cell_with_color(x + 1, y + 1, true, Some(hue));

        // Pétalos
        game.set_cell_with_color(x + 0, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 0, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 2, true, Some(hue));
    }
}

/// Agrega una flor mediana con color específico
fn add_flower_at_with_color(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    if x + 4 < game.width as usize && y + 4 < game.height as usize {
        // Centro de la flor
        game.set_cell_with_color(x + 2, y + 2, true, Some(hue));

        // Pétalos (patrón que se expande)
        game.set_cell_with_color(x + 1, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 3, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 3, true, Some(hue));
        game.set_cell_with_color(x + 3, y + 3, true, Some(hue));

        // Pétalos adicionales para más detalle
        game.set_cell_with_color(x + 0, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 4, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 0, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 4, true, Some(hue));
    }
}

fn add_large_flower_at_with_color(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    if x + 6 < game.width as usize && y + 6 < game.height as usize {
        // Centro
        game.set_cell_with_color(x + 3, y + 3, true, Some(hue));

        // Anillo interno
        game.set_cell_with_color(x + 2, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 4, y + 2, true, Some(hue));
        game.set_cell_with_color(x + 2, y + 4, true, Some(hue));
        game.set_cell_with_color(x + 4, y + 4, true, Some(hue));

        // Pétalos externos
        game.set_cell_with_color(x + 1, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 5, y + 1, true, Some(hue));
        game.set_cell_with_color(x + 1, y + 5, true, Some(hue));
        game.set_cell_with_color(x + 5, y + 5, true, Some(hue));

        // Pétalos adicionales
        game.set_cell_with_color(x + 0, y + 3, true, Some(hue));
        game.set_cell_with_color(x + 6, y + 3, true, Some(hue));
        game.set_cell_with_color(x + 3, y + 0, true, Some(hue));
        game.set_cell_with_color(x + 3, y + 6, true, Some(hue));
    }
}

fn add_flower_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 4 < game.width as usize && y + 4 < game.height as usize {
        // Centro de la flor
        game.set_cell(x + 2, y + 2, true);

        // Pétalos (patrón que se expande)
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 3, y + 1, true);
        game.set_cell(x + 1, y + 3, true);
        game.set_cell(x + 3, y + 3, true);

        // Pétalos adicionales para más detalle
        game.set_cell(x + 0, y + 2, true);
        game.set_cell(x + 4, y + 2, true);
        game.set_cell(x + 2, y + 0, true);
        game.set_cell(x + 2, y + 4, true);
    }
}

fn add_small_flower_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 2 < game.width as usize && y + 2 < game.height as usize {
        // Centro
        game.set_cell(x + 1, y + 1, true);

        // Pétalos
        game.set_cell(x + 0, y + 1, true);
        game.set_cell(x + 2, y + 1, true);
        game.set_cell(x + 1, y + 0, true);
        game.set_cell(x + 1, y + 2, true);
    }
}

fn add_large_flower_at(game: &mut GameOfLife, x: usize, y: usize) {
    if x + 6 < game.width as usize && y + 6 < game.height as usize {
        // Centro
        game.set_cell(x + 3, y + 3, true);

        // Anillo interno
        game.set_cell(x + 2, y + 2, true);
        game.set_cell(x + 4, y + 2, true);
        game.set_cell(x + 2, y + 4, true);
        game.set_cell(x + 4, y + 4, true);

        // Pétalos externos
        game.set_cell(x + 1, y + 1, true);
        game.set_cell(x + 5, y + 1, true);
        game.set_cell(x + 1, y + 5, true);
        game.set_cell(x + 5, y + 5, true);

        // Pétalos adicionales
        game.set_cell(x + 0, y + 3, true);
        game.set_cell(x + 6, y + 3, true);
        game.set_cell(x + 3, y + 0, true);
        game.set_cell(x + 3, y + 6, true);
    }
}

fn add_pattern6_6x6(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    let pattern = [
        [0, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 0, 1],
        [1, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [0, 1, 1, 1, 1, 0],
    ];
    for i in 0..15 {
        for j in 0..15 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + i, y + j, true, Some(hue));
            }
        }
    }
}
fn flower2(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    let pattern = [
        [0, 0, 1, 1, 0, 0],
        [0, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 1],
        [1, 0, 1, 1, 0, 1],
        [0, 1, 0, 0, 1, 0],
        [0, 0, 1, 1, 0, 0],
    ];
    for i in 0..6 {
        for j in 0..6 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + i, y + j, true, Some(hue));
            }
        }
    }
}

fn flower3(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    let pattern = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
        [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    for i in 0..15 {
        for j in 0..15 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + i, y + j, true, Some(hue));
            }
        }
    }
}

fn butterfly(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    let pattern = [
        [0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 1, 0, 1, 0, 0],
        [0, 0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0],
    ];
    for i in 0..6 {
        for j in 0..6 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + i, y + j, true, Some(hue));
            }
        }
    }
}

fn bottle(game: &mut GameOfLife, x: usize, y: usize, hue: Hue) {
    let pattern = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    for i in 0..15 {
        for j in 0..15 {
            if pattern[i][j] == 1 {
                game.set_cell_with_color(x + i, y + j, true, Some(hue));
            }
        }
    }
}

fn add_flowers(game: &mut GameOfLife) {
    // Limpiar el grid
    game.clear_grid();

    // Llenar el grid alternando bottle, butterfly y flower3
    let colores = [0.0, 0.17, 0.33, 0.5, 0.66, 0.83];
    let patrones = ["bottle", "butterfly", "flower3"];
    let pat_sizes = [15, 6, 15]; // bottle:15x15, butterfly:6x6, flower3:15x15
    let sep = 8; // separación entre patrones
    let mut idx = 0;
    let mut count = 0;
    for y in (0..(game.height as usize - 5)).step_by(15 + sep) {
        for x in (0..(game.width as usize - 5)).step_by(15 + sep) {
            let pat_idx = idx % patrones.len();
            let pat = patrones[pat_idx];
            let size = pat_sizes[pat_idx];
            if x + size > game.width as usize || y + size > game.height as usize {
                continue;
            }
            let hue = Hue::new(colores[idx % colores.len()]);
            match pat {
                "bottle" => bottle(game, x, y, hue),
                "butterfly" => butterfly(game, x, y, hue),
                "flower3" => flower3(game, x, y, hue),
                _ => {}
            }
            idx += 1;
            count += 1;
        }
    }
    // Spaceship en la esquina superior derecha
    let hue = Hue::new(0.1); // color diferente
    let x = game.width as usize - 5;
    let y = 0;
    add_spaceship_lwss(game, x, y, hue);
    let hue1 = Hue::new(0.3); // color diferente
    let x1 = game.width as usize - 20;
    let y1 = 0;
    add_spaceship_lwss(game, x1, y1, hue1);

    // Spaceship en la esquina superior izquierda
    let hue2 = Hue::new(0.5);
    let x2 = 0;
    let y2 = 0;
    add_spaceship_lwss(game, x2, y2, hue2);
    let hue3 = Hue::new(0.7);
    let x3 = 15;
    let y3 = 0;
    add_spaceship_lwss(game, x3, y3, hue3);

    // Spaceship en la esquina inferior derecha
    let hue4 = Hue::new(0.9);
    let x4 = game.width as usize - 5;
    let y4 = game.height as usize - 4;
    add_spaceship_lwss(game, x4, y4, hue4);
    let hue5 = Hue::new(0.2);
    let x5 = game.width as usize - 20;
    let y5 = game.height as usize - 4;
    add_spaceship_lwss(game, x5, y5, hue5);

    // Spaceship en la esquina inferior izquierda
    let hue6 = Hue::new(0.8);
    let x6 = 0;
    let y6 = game.height as usize - 4;
    add_spaceship_lwss(game, x6, y6, hue6);
    let hue7 = Hue::new(0.4);
    let x7 = 15;
    let y7 = game.height as usize - 4;
    add_spaceship_lwss(game, x7, y7, hue7);
    println!(
        "Agregados {count} patrones alternando bottle, butterfly y flower3 en el grid, y spaceship en la esquina superior derecha"
    );
}
