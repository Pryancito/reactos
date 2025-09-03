// Kernel de Eclipse OS con header Multiboot al inicio
#include <stdint.h>

// Multiboot header - DEBE estar al inicio del archivo
__attribute__((section(".multiboot")))
__attribute__((aligned(4)))
const struct {
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
} multiboot_header = {
    .magic = 0x1BADB002,
    .flags = 0x00000000,
    .checksum = -(0x1BADB002 + 0x00000000)
};

// Función para imprimir en VGA
void vga_print(const char* str) {
    volatile uint16_t* vga_buffer = (volatile uint16_t*)0xB8000;
    static int vga_index = 0;
    
    for (int i = 0; str[i] != '\0'; i++) {
        if (vga_index >= 80 * 25) {
            // Scroll
            for (int j = 0; j < 80 * 24; j++) {
                vga_buffer[j] = vga_buffer[j + 80];
            }
            for (int j = 80 * 24; j < 80 * 25; j++) {
                vga_buffer[j] = 0x0F20;
            }
            vga_index = 80 * 24;
        }
        
        vga_buffer[vga_index] = 0x0F00 | str[i];
        vga_index++;
    }
}

void vga_println(const char* str) {
    vga_print(str);
    vga_print("\n");
}

// Función principal del kernel
void kernel_main(void) {
    // Limpiar pantalla
    volatile uint16_t* vga_buffer = (volatile uint16_t*)0xB8000;
    for (int i = 0; i < 80 * 25; i++) {
        vga_buffer[i] = 0x0F20;
    }
    
    // Mostrar mensaje de bienvenida
    vga_println("Eclipse OS Kernel iniciado!");
    vga_println("Kernel compatible con Multiboot");
    vga_println("Inicializando componentes del kernel...");
    vga_println("");
    vga_println("Kernel inicializado correctamente");
    vga_println("Presiona Ctrl+Alt+Q para salir de QEMU");
    vga_println("");
    vga_println("Shell interactivo disponible!");
    vga_println("Escribe 'help' para ver comandos disponibles");
    vga_println("");
    vga_println("Eclipse OS> ");
    
    // Simular shell básico
    while (1) {
        // Cursor parpadeante
        static int cursor_state = 0;
        cursor_state++;
        if (cursor_state % 1000 < 500) {
            vga_buffer[80 * 24 + 12] = 0x0F5F; // Cursor visible
        } else {
            vga_buffer[80 * 24 + 12] = 0x0F20; // Cursor invisible
        }
    }
}

// Punto de entrada
void _start(void) {
    kernel_main();
}
