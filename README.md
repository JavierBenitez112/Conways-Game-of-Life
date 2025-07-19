# Juego de la Vida de Conway - Versión Colorida

![gameoflife](https://github.com/user-attachments/assets/9be6875d-d115-432c-b9e1-99bee27ed390)



Una implementación del clásico Juego de la Vida de Conway usando Rust y Raylib, con un framebuffer personalizado y **sistema de colores heredables**.

## Características

- **Implementación completa** de las reglas del Juego de la Vida de Conway
- **Sistema de colores heredables**: Las células heredan colores de sus vecinos
- **Variación de color**: Las nuevas células pueden tener variaciones en su color
- **Framebuffer personalizado** más pequeño que la ventana para mejor rendimiento
- **Múltiples patrones predefinidos**: Glider, Blinker, Toad, Beacon y patrón aleatorio
- **Visualización en tiempo real** con estadísticas en consola
- **Configuración flexible** de resolución y velocidad

## Reglas del Juego

El Juego de la Vida de Conway sigue estas reglas simples:

1. **Subpoblación**: Cualquier célula viva con menos de 2 vecinos vivos muere
2. **Supervivencia**: Una célula viva con 2 o 3 vecinos vivos sobrevive
3. **Sobrepoblación**: Cualquier célula viva con más de 3 vecinos vivos muere
4. **Reproducción**: Cualquier célula muerta con exactamente 3 vecinos vivos nace

## Sistema de Colores

Además de las reglas clásicas, este proyecto incluye un sistema de colores heredables:

- **Herencia de colores**: Las nuevas células heredan el color promedio de sus vecinos
- **Variación de color**: Se puede configurar una variación para que las nuevas células tengan colores ligeramente diferentes
- **Persistencia**: Las células vivas mantienen su color hasta que mueren
- **Conversión HSV**: Los colores se manejan en el espacio HSV para mejor mezcla

### Configuración de Colores

```rust
// Configurar variación de color (0.0 = sin variación, 1.0 = máxima variación)
game.set_color_variation(0.05);

// Establecer una célula con color específico
game.set_cell_with_color(x, y, true, Some(Hue::new(0.0))); // Rojo
game.set_cell_with_color(x, y, true, Some(Hue::new(0.33))); // Verde
game.set_cell_with_color(x, y, true, Some(Hue::new(0.66))); // Azul
```

## Patrones Incluidos

- **Glider**: Un patrón que se mueve diagonalmente por el grid
- **Blinker**: Un oscilador simple que alterna entre dos estados
- **Toad**: Un oscilador de período 2
- **Beacon**: Un oscilador de período 2 más complejo
- **Random**: Un patrón aleatorio para experimentar

## Configuración

El juego está configurado con:
- **Ventana**: 800x600 píxeles
- **Framebuffer**: 400x300 píxeles (más pequeño para mejor rendimiento)
- **Grid del juego**: 100x75 células
- **Escala de células**: 3x3 píxeles por célula
- **Velocidad**: 10 FPS para mejor visualización

## Cómo Ejecutar

```bash
# Versión estándar
cargo run

# Versión en alta resolución (más células, más detalle)
cargo run --example high_resolution

# Población personalizada avanzada (múltiples patrones interactuando)
cargo run --example custom_population

# Jardín de flores (patrones florales que se expanden)
cargo run --example flowers

# Juego de la Vida Colorido (sistema de colores heredables)
cargo run --example colorful_life
```

## Creación de Poblaciones

El juego ahora permite crear poblaciones personalizadas directamente en el código. Para cambiar los patrones:

1. Abre `src/main.rs`
2. Encuentra la sección "AQUÍ PUEDES AGREGAR TUS PATRONES"
3. Descomenta la función que quieras usar
4. Comenta las otras funciones
5. Ejecuta `cargo run`

### Patrones Disponibles:
- **Múltiples Gliders**: 5 gliders en diferentes posiciones
- **Combinación de Patrones**: Mezcla de gliders, blinkers, toads y beacons
- **Población Densa**: Región densa con células aleatorias
- **Patrones en Esquinas**: Diferentes patrones en cada esquina
- **Patrón Aleatorio**: Población completamente aleatoria
- **Jardín de Flores**: Flores de diferentes tamaños que se expanden
- **Población Avanzada**: Múltiples patrones interactuando (ejemplo separado)

Consulta `PATRONES.md` para ejemplos detallados y más opciones de personalización.

## Personalización

Para cambiar el patrón inicial, modifica esta línea en `main.rs`:

```rust
game.initialize_with_pattern("glider"); // Cambia por: "blinker", "toad", "beacon", "random"
```

Para ajustar la velocidad, modifica el `update_interval`:

```rust
let update_interval = Duration::from_millis(100); // Menor valor = más rápido
```

Para cambiar la resolución del juego, modifica:

```rust
let game_width = 100;   // Ancho del grid
let game_height = 75;   // Alto del grid
let cell_scale = 3;     // Tamaño de cada célula en píxeles
```

## Estructura del Proyecto

- `src/main.rs`: Punto de entrada y bucle principal
- `src/game_of_life.rs`: Lógica del Juego de la Vida
- `src/framebuffer.rs`: Implementación del framebuffer
- `src/line.rs`: Funciones de dibujo de líneas (no usado en esta versión)

## Dependencias

- **raylib**: Biblioteca de gráficos 2D/3D
- **Rust**: Lenguaje de programación

## Notas Técnicas

- El framebuffer es más pequeño que la ventana para optimizar el rendimiento
- No se limpia el framebuffer entre frames como se recomienda en las instrucciones
- Cada célula se renderiza como un cuadrado de píxeles para mejor visibilidad
- Las estadísticas se muestran en la consola cada 100 frames 
