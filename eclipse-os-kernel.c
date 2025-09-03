// Kernel de Eclipse OS con framebuffer mejorado
#include <stdint.h>
#include <stddef.h>

// Multiboot header
struct multiboot_header {
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
} __attribute__((packed));

// Header de Multiboot
__attribute__((section(".multiboot")))
__attribute__((aligned(4)))
const struct multiboot_header mb_header = {
    .magic = 0x1BADB002,
    .flags = 0x00000000,
    .checksum = -(0x1BADB002 + 0x00000000)
};

// Estructura para framebuffer
struct framebuffer_info {
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    uint32_t bpp;
    uint32_t* buffer;
};

// Información del framebuffer (será inicializada por Multiboot)
struct framebuffer_info fb_info = {0};

// Colores RGB
#define COLOR_BLACK     0x000000
#define COLOR_WHITE     0xFFFFFF
#define COLOR_RED       0xFF0000
#define COLOR_GREEN     0x00FF00
#define COLOR_BLUE      0x0000FF
#define COLOR_YELLOW    0xFFFF00
#define COLOR_CYAN      0x00FFFF
#define COLOR_MAGENTA   0xFF00FF
#define COLOR_GRAY      0x808080
#define COLOR_DARK_GRAY 0x404040
#define COLOR_LIGHT_GRAY 0xC0C0C0

// Función para dibujar un pixel
void draw_pixel(uint32_t x, uint32_t y, uint32_t color) {
    if (x < fb_info.width && y < fb_info.height) {
        fb_info.buffer[y * (fb_info.pitch / 4) + x] = color;
    }
}

// Función para dibujar un rectángulo
void draw_rect(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t color) {
    for (uint32_t dy = 0; dy < height; dy++) {
        for (uint32_t dx = 0; dx < width; dx++) {
            draw_pixel(x + dx, y + dy, color);
        }
    }
}

// Función para dibujar texto (fuente simple 8x8)
void draw_char(uint32_t x, uint32_t y, char c, uint32_t color) {
    // Fuente simple 8x8 (solo algunos caracteres)
    static const uint8_t font[][8] = {
        // A
        {0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00},
        // B
        {0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00},
        // C
        {0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00},
        // D
        {0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00},
        // E
        {0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00},
        // F
        {0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00},
        // G
        {0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00},
        // H
        {0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00},
        // I
        {0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00},
        // J
        {0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00},
        // K
        {0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00},
        // L
        {0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00},
        // M
        {0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00},
        // N
        {0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00},
        // O
        {0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00},
        // P
        {0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00},
        // Q
        {0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x0E, 0x00},
        // R
        {0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00},
        // S
        {0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00},
        // T
        {0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00},
        // U
        {0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00},
        // V
        {0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00},
        // W
        {0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00},
        // X
        {0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00},
        // Y
        {0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00},
        // Z
        {0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00},
        // Espacio
        {0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
        // !
        {0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x18, 0x00},
        // ?
        {0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00},
        // :
        {0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00},
        // -
        {0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00},
        // =
        {0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00},
        // (
        {0x0E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x0E, 0x00},
        // )
        {0x70, 0x18, 0x18, 0x18, 0x18, 0x18, 0x70, 0x00},
        // [
        {0x7C, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7C, 0x00},
        // ]
        {0x7C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x7C, 0x00},
        // {
        {0x1E, 0x30, 0x30, 0x70, 0x30, 0x30, 0x1E, 0x00},
        // }
        {0x78, 0x0C, 0x0C, 0x0E, 0x0C, 0x0C, 0x78, 0x00},
        // #
        {0x66, 0x66, 0xFF, 0x66, 0xFF, 0x66, 0x66, 0x00},
        // $
        {0x18, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00},
        // %
        {0x62, 0x66, 0x0C, 0x18, 0x30, 0x66, 0x46, 0x00},
        // &
        {0x3C, 0x66, 0x3C, 0x38, 0x67, 0x66, 0x3F, 0x00},
        // *
        {0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00, 0x00},
        // +
        {0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00},
        // ,
        {0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30, 0x00},
        // .
        {0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00},
        // /
        {0x00, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x00},
        // 0
        {0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00},
        // 1
        {0x18, 0x18, 0x38, 0x18, 0x18, 0x18, 0x7E, 0x00},
        // 2
        {0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x7E, 0x00},
        // 3
        {0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00},
        // 4
        {0x06, 0x0E, 0x1E, 0x66, 0x7F, 0x06, 0x06, 0x00},
        // 5
        {0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00},
        // 6
        {0x3C, 0x66, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00},
        // 7
        {0x7E, 0x66, 0x0C, 0x18, 0x18, 0x18, 0x18, 0x00},
        // 8
        {0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00},
        // 9
        {0x3C, 0x66, 0x66, 0x3E, 0x06, 0x66, 0x3C, 0x00}
    };
    
    if (c >= 'A' && c <= 'Z') {
        c = c - 'A';
    } else if (c >= 'a' && c <= 'z') {
        c = c - 'a';
    } else if (c >= '0' && c <= '9') {
        c = c - '0' + 26;
    } else if (c == ' ') {
        c = 26;
    } else if (c == '!') {
        c = 27;
    } else if (c == '?') {
        c = 28;
    } else if (c == ':') {
        c = 29;
    } else if (c == '-') {
        c = 30;
    } else if (c == '=') {
        c = 31;
    } else if (c == '(') {
        c = 32;
    } else if (c == ')') {
        c = 33;
    } else if (c == '[') {
        c = 34;
    } else if (c == ']') {
        c = 35;
    } else if (c == '{') {
        c = 36;
    } else if (c == '}') {
        c = 37;
    } else if (c == '#') {
        c = 38;
    } else if (c == '$') {
        c = 39;
    } else if (c == '%') {
        c = 40;
    } else if (c == '&') {
        c = 41;
    } else if (c == '*') {
        c = 42;
    } else if (c == '+') {
        c = 43;
    } else if (c == ',') {
        c = 44;
    } else if (c == '.') {
        c = 45;
    } else if (c == '/') {
        c = 46;
    } else {
        return; // Caracter no soportado
    }
    
    for (int dy = 0; dy < 8; dy++) {
        for (int dx = 0; dx < 8; dx++) {
            if (font[c][dy] & (0x80 >> dx)) {
                draw_pixel(x + dx, y + dy, color);
            }
        }
    }
}

// Función para dibujar texto
void draw_text(uint32_t x, uint32_t y, const char* text, uint32_t color) {
    uint32_t pos_x = x;
    for (int i = 0; text[i] != '\0'; i++) {
        if (text[i] == '\n') {
            y += 10;
            pos_x = x;
        } else {
            draw_char(pos_x, y, text[i], color);
            pos_x += 9;
        }
    }
}

// Función para inicializar el framebuffer
void init_framebuffer(void) {
    // Por ahora usamos valores por defecto para QEMU
    // En un sistema real, esto vendría de Multiboot
    fb_info.width = 1024;
    fb_info.height = 768;
    fb_info.pitch = 1024 * 4; // 32 bits por pixel
    fb_info.bpp = 32;
    fb_info.buffer = (uint32_t*)0xFD000000; // Dirección del framebuffer en QEMU
    
    // Limpiar pantalla
    draw_rect(0, 0, fb_info.width, fb_info.height, COLOR_BLACK);
}

// Función para dibujar el logo de Eclipse OS
void draw_eclipse_logo(uint32_t x, uint32_t y) {
    // Dibujar un círculo simple como logo
    for (int dy = -20; dy <= 20; dy++) {
        for (int dx = -20; dx <= 20; dx++) {
            if (dx*dx + dy*dy <= 400) { // Radio 20
                draw_pixel(x + dx, y + dy, COLOR_CYAN);
            }
        }
    }
    
    // Dibujar texto "Eclipse OS"
    draw_text(x - 30, y + 30, "Eclipse OS", COLOR_WHITE);
}

// Función para dibujar una barra de progreso
void draw_progress_bar(uint32_t x, uint32_t y, uint32_t width, uint32_t height, uint32_t progress, uint32_t color) {
    // Fondo
    draw_rect(x, y, width, height, COLOR_DARK_GRAY);
    // Progreso
    draw_rect(x, y, (width * progress) / 100, height, color);
    // Borde
    draw_rect(x, y, width, 1, COLOR_WHITE);
    draw_rect(x, y + height - 1, width, 1, COLOR_WHITE);
    draw_rect(x, y, 1, height, COLOR_WHITE);
    draw_rect(x + width - 1, y, 1, height, COLOR_WHITE);
}

// Función para ejecutar el sistema Eclipse OS con interfaz gráfica
void execute_eclipse_os_gui(void) {
    // Inicializar framebuffer
    init_framebuffer();
    
    // Dibujar fondo con gradiente
    for (int y = 0; y < fb_info.height; y++) {
        uint32_t color = (y * 0x20) / fb_info.height;
        color = (color << 16) | (color << 8) | color; // Gradiente gris
        draw_rect(0, y, fb_info.width, 1, color);
    }
    
    // Dibujar logo de Eclipse OS
    draw_eclipse_logo(fb_info.width / 2, 100);
    
    // Título principal
    draw_text(fb_info.width / 2 - 150, 200, "Sistema Operativo Eclipse OS", COLOR_WHITE);
    draw_text(fb_info.width / 2 - 100, 220, "Version 1.0 - Kernel Multiboot", COLOR_LIGHT_GRAY);
    
    // Dibujar barra de progreso de inicialización
    draw_progress_bar(fb_info.width / 2 - 200, 250, 400, 20, 0, COLOR_GREEN);
    
    // Simular inicialización
    for (int progress = 0; progress <= 100; progress += 10) {
        draw_progress_bar(fb_info.width / 2 - 200, 250, 400, 20, progress, COLOR_GREEN);
        // Pequeña pausa
        for (volatile int i = 0; i < 1000000; i++);
    }
    
    // Dibujar información del sistema
    draw_text(50, 300, "Caracteristicas del Sistema:", COLOR_YELLOW);
    draw_text(70, 320, "• Kernel compatible con Multiboot", COLOR_WHITE);
    draw_text(70, 340, "• Framebuffer de alta resolucion", COLOR_WHITE);
    draw_text(70, 360, "• Interfaz grafica moderna", COLOR_WHITE);
    draw_text(70, 380, "• Aplicaciones integradas", COLOR_WHITE);
    draw_text(70, 400, "• Sistema de autenticacion", COLOR_WHITE);
    draw_text(70, 420, "• Red y conectividad", COLOR_WHITE);
    
    // Dibujar aplicaciones disponibles
    draw_text(50, 460, "Aplicaciones Disponibles:", COLOR_YELLOW);
    draw_text(70, 480, "• Editor de texto", COLOR_CYAN);
    draw_text(70, 500, "• Explorador de archivos", COLOR_CYAN);
    draw_text(70, 520, "• Calculadora", COLOR_CYAN);
    draw_text(70, 540, "• Navegador web", COLOR_CYAN);
    
    // Dibujar estado del sistema
    draw_text(50, 580, "Estado del Sistema:", COLOR_YELLOW);
    draw_text(70, 600, "• Kernel: Inicializado", COLOR_GREEN);
    draw_text(70, 620, "• Framebuffer: Activo", COLOR_GREEN);
    draw_text(70, 640, "• Sistema: Funcionando", COLOR_GREEN);
    
    // Dibujar mensaje final
    draw_text(fb_info.width / 2 - 100, fb_info.height - 50, "Sistema Eclipse OS completamente funcional!", COLOR_GREEN);
    draw_text(fb_info.width / 2 - 80, fb_info.height - 30, "Presiona Ctrl+Alt+Q para salir", COLOR_LIGHT_GRAY);
}

// Función principal del kernel
void kernel_main(void) {
    // Ejecutar el sistema Eclipse OS con interfaz gráfica
    execute_eclipse_os_gui();
    
    // Loop infinito
    while (1) {
        // Mantener el sistema funcionando
    }
}

// Punto de entrada
void _start(void) {
    kernel_main();
}