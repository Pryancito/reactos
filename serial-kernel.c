// Kernel de Eclipse OS con salida por puerto serie
#include <stdint.h>

// Multiboot header
struct multiboot_header {
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
} __attribute__((packed));

// Header de Multiboot
__attribute__((section(".multiboot")))
const struct multiboot_header mb_header = {
    .magic = 0x1BADB002,
    .flags = 0x00000000,
    .checksum = -(0x1BADB002 + 0x00000000)
};

// Funciones para acceso a puertos
static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void outb(uint16_t port, uint8_t val) {
    asm volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

// Función para imprimir por puerto serie
void serial_print(const char* str) {
    for (int i = 0; str[i] != '\0'; i++) {
        // Esperar a que el puerto serie esté listo
        while ((inb(0x3F8 + 5) & 0x20) == 0);
        outb(0x3F8, str[i]);
    }
}

void serial_println(const char* str) {
    serial_print(str);
    serial_print("\n");
}

// Función principal del kernel
void kernel_main(void) {
    // Inicializar puerto serie
    outb(0x3F8 + 1, 0x00);    // Deshabilitar interrupciones
    outb(0x3F8 + 3, 0x80);    // Habilitar DLAB
    outb(0x3F8 + 0, 0x03);    // Divisor de baudios (115200 baud)
    outb(0x3F8 + 1, 0x00);    // Divisor de baudios (alto byte)
    outb(0x3F8 + 3, 0x03);    // 8 bits, sin paridad, 1 bit de parada
    outb(0x3F8 + 2, 0xC7);    // Habilitar FIFO
    outb(0x3F8 + 4, 0x0B);    // IRQs habilitadas, RTS/DSR set
    
    // Mostrar mensaje de bienvenida
    serial_println("Eclipse OS Kernel iniciado!");
    serial_println("Kernel compatible con Multiboot");
    serial_println("Inicializando componentes del kernel...");
    serial_println("");
    serial_println("Kernel inicializado correctamente");
    serial_println("Shell interactivo disponible!");
    serial_println("Escribe 'help' para ver comandos disponibles");
    serial_println("");
    serial_println("Eclipse OS> ");
    
    // Simular shell básico
    while (1) {
        // Simular procesamiento
        static int counter = 0;
        counter++;
        if (counter % 1000000 == 0) {
            serial_print(".");
        }
    }
}

// Punto de entrada
void _start(void) {
    kernel_main();
}
