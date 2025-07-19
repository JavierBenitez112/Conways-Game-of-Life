# Guía de Patrones y Poblaciones - Juego de la Vida Colorido

Este archivo te muestra cómo crear diferentes tipos de poblaciones y patrones en el Juego de la Vida de Conway con el nuevo sistema de colores heredables.

## Cómo Usar

1. Abre `src/main.rs`
2. Encuentra la sección "AQUÍ PUEDES AGREGAR TUS PATRONES"
3. Descomenta la función que quieras usar
4. Comenta las otras funciones
5. Ejecuta `cargo run`

## Patrones Disponibles

### 1. Múltiples Gliders
```rust
add_multiple_gliders(&mut game);
```
- Agrega 5 gliders en diferentes posiciones
- Los gliders se mueven diagonalmente por el grid
- Perfecto para ver interacciones entre gliders

### 2. Combinación de Patrones
```rust
add_pattern_combination(&mut game);
```
- Glider en el centro
- Blinker en la esquina superior izquierda
- Toad en la esquina inferior derecha
- Beacon en el lado izquierdo

### 3. Población Densa
```rust
add_dense_population(&mut game);
```
- Crea una región densa de células aleatorias
- Agrega gliders para movimiento
- Excelente para ver comportamientos emergentes

### 4. Patrones en las Esquinas
```rust
add_corner_patterns(&mut game);
```
- Blinker en esquina superior izquierda
- Glider en esquina superior derecha
- Toad en esquina inferior izquierda
- Beacon en esquina inferior derecha
- Glider adicional en el centro

### 5. Patrón Aleatorio
```rust
game.initialize_with_pattern("random");
```
- Genera un patrón completamente aleatorio
- Útil para experimentar con poblaciones caóticas

### 6. Jardín de Flores
```rust
add_flowers(&mut game);
```
- Crea un jardín con flores de diferentes tamaños
- Las flores se expanden y crean patrones hermosos
- Incluye flores grandes, medianas y pequeñas
- **¡Ahora con colores!** Cada flor tiene un color específico que se hereda

### 7. Sistema de Colores
```rust
// Configurar variación de color
game.set_color_variation(0.05);

// Establecer células con colores específicos
game.set_cell_with_color(x, y, true, Some(Hue::new(0.0)));   // Rojo
game.set_cell_with_color(x, y, true, Some(Hue::new(0.33)));  // Verde
game.set_cell_with_color(x, y, true, Some(Hue::new(0.66)));  // Azul
```
- Las células heredan colores de sus vecinos
- Configurable variación de color para nuevas células
- Conversión automática de HSV a RGB

## Funciones Auxiliares Disponibles

### Agregar Patrones Individuales

```rust
// Agregar un glider en posición específica
add_glider_at(&mut game, x, y);

// Agregar un blinker en posición específica
add_blinker_at(&mut game, x, y);

// Agregar un toad en posición específica
add_toad_at(&mut game, x, y);

// Agregar un beacon en posición específica
add_beacon_at(&mut game, x, y);

// Agregar flores de diferentes tamaños
add_small_flower_at(&mut game, x, y);  // Flor pequeña (3x3)
add_flower_at(&mut game, x, y);        // Flor mediana (5x5)
add_large_flower_at(&mut game, x, y);  // Flor grande (7x7)

// Agregar patrones con colores específicos
add_glider_at_with_color(&mut game, x, y, Hue::new(0.0));     // Glider rojo
add_flower_at_with_color(&mut game, x, y, Hue::new(0.33));    // Flor verde
add_small_flower_at_with_color(&mut game, x, y, Hue::new(0.66)); // Flor pequeña azul
add_large_flower_at_with_color(&mut game, x, y, Hue::new(0.17)); // Flor grande amarilla

### Manipular Células Individuales

```rust
// Establecer una célula específica como viva o muerta
game.set_cell(x, y, true);   // Viva
game.set_cell(x, y, false);  // Muerta

// Establecer una célula con color específico
game.set_cell_with_color(x, y, true, Some(Hue::new(0.0)));   // Rojo
game.set_cell_with_color(x, y, true, Some(Hue::new(0.33)));  // Verde
game.set_cell_with_color(x, y, true, Some(Hue::new(0.66)));  // Azul

// Obtener el estado de una célula
let is_alive = game.get_cell(x, y);

// Obtener el color de una célula
let color = game.get_cell_color(x, y);

// Limpiar todo el grid
game.clear_grid();
```

## Ejemplos de Poblaciones Personalizadas

### Ejemplo 1: Línea de Gliders
```rust
game.clear_grid();
for i in 0..10 {
    add_glider_at(&mut game, 10 + i * 8, 20);
}
```

### Ejemplo 2: Patrón de Blinkers
```rust
game.clear_grid();
for i in 0..5 {
    add_blinker_at(&mut game, 10 + i * 15, 10 + i * 10);
}
```

### Ejemplo 3: Región Densa Personalizada
```rust
game.clear_grid();
for x in 30..70 {
    for y in 30..60 {
        if (x + y) % 2 == 0 {
            game.set_cell(x, y, true);
        }
    }
}
```

### Ejemplo 4: Patrón Simétrico
```rust
game.clear_grid();
// Patrón en forma de cruz
for i in 0..10 {
    game.set_cell(50, 30 + i, true);  // Línea vertical
    game.set_cell(45 + i, 35, true);  // Línea horizontal
}
```

### Ejemplo 5: Población Mixta
```rust
game.clear_grid();
// Agregar diferentes patrones
add_glider_at(&mut game, 20, 20);
add_blinker_at(&mut game, 60, 20);
add_toad_at(&mut game, 20, 50);
add_beacon_at(&mut game, 60, 50);

// Agregar algunas células aleatorias
for i in 0..20 {
    let x = 40 + (i * 3) % 20;
    let y = 40 + (i * 7) % 20;
    game.set_cell(x, y, true);
}
```

### Ejemplo 6: Jardín de Flores Personalizado
```rust
game.clear_grid();

// Crear un jardín con flores en filas
for i in 0..5 {
    add_flower_at(&mut game, 10 + i * 15, 10);
    add_small_flower_at(&mut game, 15 + i * 15, 25);
    add_large_flower_at(&mut game, 5 + i * 15, 40);
}

// Agregar flores en las esquinas
add_large_flower_at(&mut game, 5, 5);
add_large_flower_at(&mut game, 70, 5);
add_large_flower_at(&mut game, 5, 60);
add_large_flower_at(&mut game, 70, 60);
```

## Consejos para Experimentar

1. **Empieza Simple**: Usa patrones básicos como gliders o blinkers
2. **Observa Interacciones**: Coloca patrones cerca para ver cómo interactúan
3. **Experimenta con Densidades**: Prueba diferentes densidades de población
4. **Usa Simetrías**: Los patrones simétricos suelen crear comportamientos interesantes
5. **Combina Patrones**: Mezcla diferentes tipos de patrones para resultados únicos

## Patrones Clásicos para Probar

### Gosper Glider Gun
```rust
// Patrón complejo que genera gliders continuamente
// (Requiere implementación específica)
```

### Pulsar
```rust
// Oscilador de período 3
// (Requiere implementación específica)
```

### Pentadecathlon
```rust
// Oscilador de período 15
// (Requiere implementación específica)
```

## Notas Técnicas

- El grid tiene dimensiones de 100x75 células
- Las coordenadas van de (0,0) a (99,74)
- Los patrones se renderizan como cuadrados de 3x3 píxeles
- La simulación corre a 10 FPS por defecto
- Las estadísticas se muestran cada 100 frames

¡Diviértete experimentando con diferentes poblaciones! 