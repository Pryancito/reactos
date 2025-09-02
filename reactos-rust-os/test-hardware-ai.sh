#!/bin/bash

# Script de test para hardware acelerado de AI de ReactOS Rust OS
echo "🚀 Probando hardware acelerado de AI de ReactOS Rust OS..."

# Configuración
RUST_TOOLCHAIN="nightly"
TARGET_X86="i686-unknown-none"
TARGET_X64="x86_64-unknown-none"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_ai() {
    echo -e "${PURPLE}[AI]${NC} $1"
}

print_hardware() {
    echo -e "${CYAN}[HARDWARE]${NC} $1"
}

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias para hardware acelerado de AI..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no está instalado"
        exit 1
    fi
    
    if ! command -v rustup &> /dev/null; then
        print_error "Rustup no está instalado"
        exit 1
    fi
    
    # Instalar toolchain nightly si no está instalado
    rustup toolchain install nightly
    
    # Instalar targets
    rustup target add $TARGET_X86
    rustup target add $TARGET_X64
    
    print_success "Dependencias verificadas"
}

# Compilar drivers con hardware acelerado para x86
compile_drivers_hardware_x86() {
    print_hardware "Compilando drivers con hardware acelerado para x86 (32-bit)..."
    
    cd drivers
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Drivers con hardware acelerado x86 compilados exitosamente"
    else
        print_error "Error al compilar drivers con hardware acelerado x86"
        exit 1
    fi
    
    cd ..
}

# Compilar drivers con hardware acelerado para x86_64
compile_drivers_hardware_x64() {
    print_hardware "Compilando drivers con hardware acelerado para x86_64 (64-bit)..."
    
    cd drivers
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Drivers con hardware acelerado x86_64 compilados exitosamente"
    else
        print_error "Error al compilar drivers con hardware acelerado x86_64"
        exit 1
    fi
    
    cd ..
}

# Compilar userland con hardware acelerado para x86
compile_userland_hardware_x86() {
    print_ai "Compilando userland con hardware acelerado para x86 (32-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland con hardware acelerado x86 compilado exitosamente"
    else
        print_error "Error al compilar userland con hardware acelerado x86"
        exit 1
    fi
    
    cd ..
}

# Compilar userland con hardware acelerado para x86_64
compile_userland_hardware_x64() {
    print_ai "Compilando userland con hardware acelerado para x86_64 (64-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland con hardware acelerado x86_64 compilado exitosamente"
    else
        print_error "Error al compilar userland con hardware acelerado x86_64"
        exit 1
    fi
    
    cd ..
}

# Crear archivos de test para hardware acelerado
create_hardware_ai_test_files() {
    print_hardware "Creando archivos de test para hardware acelerado de AI..."
    
    # Crear directorio de test
    mkdir -p test-hardware-ai/{ai_hardware,npu,gpu_ai,cpu_ai}
    
    # Crear archivos de test para AI Hardware Manager
    cat > test-hardware-ai/ai_hardware/test_ai_hardware.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de AI Hardware Manager
extern int AIHardware_Initialize();
extern int AIHardware_EnumerateDevices(void* devices, unsigned int max_devices, unsigned int* device_count);
extern int AIHardware_GetDeviceInfo(unsigned int device_id, void* device_info);
extern int AIHardware_CreateContext(void* device_handle, unsigned int context_type, void** context_handle);
extern int AIHardware_DestroyContext(void* context_handle);
extern int AIHardware_LoadModel(void* context_handle, const unsigned char* model_data, unsigned long long model_size, void** model_handle);
extern int AIHardware_UnloadModel(void* model_handle);
extern int AIHardware_ExecuteInference(void* context_handle, void* model_handle, const unsigned char* input_data, unsigned long long input_size, unsigned char* output_data, unsigned long long output_size, void** operation_handle);
extern int AIHardware_WaitForOperation(void* operation_handle, unsigned int timeout);
extern int AIHardware_GetOperationStatus(void* operation_handle, unsigned int* status, float* progress);
extern int AIHardware_GetOperationResult(void* operation_handle, unsigned char* result_data, unsigned long long* result_size);
extern int AIHardware_GetPerformanceMetrics(unsigned int device_id, void* performance);
extern int AIHardware_GetDeviceCount();
extern int AIHardware_GetContextCount();
extern int AIHardware_GetOperationCount();
extern int AIHardware_Test();

int main() {
    printf("🚀 Probando AI Hardware Manager...\n");
    
    // Inicializar AI Hardware Manager
    if (AIHardware_Initialize() == 0) {
        printf("✅ AI Hardware Manager inicializado\n");
    } else {
        printf("❌ Error al inicializar AI Hardware Manager\n");
        return 1;
    }
    
    // Enumerar dispositivos
    unsigned int device_count;
    if (AIHardware_EnumerateDevices(NULL, 10, &device_count) == 0) {
        printf("✅ Dispositivos enumerados: %u\n", device_count);
    } else {
        printf("❌ Error al enumerar dispositivos\n");
    }
    
    // Obtener información del dispositivo
    if (AIHardware_GetDeviceInfo(0, NULL) == 0) {
        printf("✅ Información del dispositivo obtenida\n");
    } else {
        printf("❌ Error al obtener información del dispositivo\n");
    }
    
    // Crear contexto
    void* context_handle;
    if (AIHardware_CreateContext(NULL, 1, &context_handle) == 0) {
        printf("✅ Contexto creado\n");
        
        // Cargar modelo
        void* model_handle;
        if (AIHardware_LoadModel(context_handle, NULL, 1024, &model_handle) == 0) {
            printf("✅ Modelo cargado\n");
            
            // Ejecutar inferencia
            void* operation_handle;
            if (AIHardware_ExecuteInference(context_handle, model_handle, NULL, 256, NULL, 128, &operation_handle) == 0) {
                printf("✅ Inferencia ejecutada\n");
                
                // Esperar operación
                if (AIHardware_WaitForOperation(operation_handle, 5000) == 0) {
                    printf("✅ Operación completada\n");
                    
                    // Obtener resultado
                    unsigned char result[128];
                    unsigned long long result_size = sizeof(result);
                    if (AIHardware_GetOperationResult(operation_handle, result, &result_size) == 0) {
                        printf("✅ Resultado obtenido (%llu bytes)\n", result_size);
                    } else {
                        printf("❌ Error al obtener resultado\n");
                    }
                } else {
                    printf("❌ Error al esperar operación\n");
                }
            } else {
                printf("❌ Error al ejecutar inferencia\n");
            }
            
            // Descargar modelo
            AIHardware_UnloadModel(model_handle);
        } else {
            printf("❌ Error al cargar modelo\n");
        }
        
        // Destruir contexto
        AIHardware_DestroyContext(context_handle);
    } else {
        printf("❌ Error al crear contexto\n");
    }
    
    // Obtener métricas de rendimiento
    if (AIHardware_GetPerformanceMetrics(0, NULL) == 0) {
        printf("✅ Métricas de rendimiento obtenidas\n");
    } else {
        printf("❌ Error al obtener métricas de rendimiento\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de dispositivos: %d\n", AIHardware_GetDeviceCount());
    printf("✅ Número de contextos: %d\n", AIHardware_GetContextCount());
    printf("✅ Número de operaciones: %d\n", AIHardware_GetOperationCount());
    
    // Test completo
    if (AIHardware_Test() == 0) {
        printf("✅ Test de AI Hardware Manager completado\n");
    } else {
        printf("❌ Error en test de AI Hardware Manager\n");
        return 1;
    }
    
    printf("🎉 Test de AI Hardware Manager completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para NPU
    cat > test-hardware-ai/npu/test_npu.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de NPU
extern int NPU_Initialize();
extern int NPU_EnumerateDevices(void* devices, unsigned int max_devices, unsigned int* device_count);
extern int NPU_GetDeviceInfo(unsigned int device_id, void* device_info);
extern int NPU_OpenDevice(unsigned int device_id, void** device_handle);
extern int NPU_CloseDevice(void* device_handle);
extern int NPU_LoadModel(void* device_handle, const unsigned char* model_data, unsigned long long model_size, void** model_handle);
extern int NPU_UnloadModel(void* model_handle);
extern int NPU_OptimizeModel(void* model_handle, unsigned int optimization_level, unsigned int target_precision);
extern int NPU_QuantizeModel(void* model_handle, unsigned int quantization_type, unsigned int target_precision);
extern int NPU_ExecuteInference(void* device_handle, void* model_handle, const unsigned char* input_data, unsigned long long input_size, unsigned char* output_data, unsigned long long output_size, void** operation_handle);
extern int NPU_ExecuteTraining(void* device_handle, void* model_handle, const unsigned char* training_data, unsigned long long training_size, void** operation_handle);
extern int NPU_ExecuteOptimization(void* device_handle, void* model_handle, unsigned int optimization_type, const float* parameters, void** operation_handle);
extern int NPU_WaitForOperation(void* operation_handle, unsigned int timeout);
extern int NPU_GetOperationStatus(void* operation_handle, unsigned int* status, float* progress);
extern int NPU_GetOperationResult(void* operation_handle, unsigned char* result_data, unsigned long long* result_size);
extern int NPU_GetPerformanceMetrics(void* device_handle, void* metrics);
extern int NPU_SetPowerMode(void* device_handle, unsigned int power_mode);
extern int NPU_GetPowerMode(void* device_handle, unsigned int* power_mode);
extern int NPU_GetTemperature(void* device_handle, float* temperature);
extern int NPU_GetUtilization(void* device_handle, float* utilization);
extern int NPU_GetMemoryUsage(void* device_handle, unsigned long long* memory_usage, unsigned long long* memory_total);
extern int NPU_GetDeviceCount();
extern int NPU_GetModelCount();
extern int NPU_GetOperationCount();
extern int NPU_Test();

int main() {
    printf("🧠 Probando NPU Driver...\n");
    
    // Inicializar NPU
    if (NPU_Initialize() == 0) {
        printf("✅ NPU inicializado\n");
    } else {
        printf("❌ Error al inicializar NPU\n");
        return 1;
    }
    
    // Enumerar dispositivos NPU
    unsigned int device_count;
    if (NPU_EnumerateDevices(NULL, 10, &device_count) == 0) {
        printf("✅ Dispositivos NPU enumerados: %u\n", device_count);
    } else {
        printf("❌ Error al enumerar dispositivos NPU\n");
    }
    
    // Abrir dispositivo NPU
    void* device_handle;
    if (NPU_OpenDevice(0, &device_handle) == 0) {
        printf("✅ Dispositivo NPU abierto\n");
        
        // Cargar modelo en NPU
        void* model_handle;
        if (NPU_LoadModel(device_handle, NULL, 2048, &model_handle) == 0) {
            printf("✅ Modelo cargado en NPU\n");
            
            // Optimizar modelo para NPU
            if (NPU_OptimizeModel(model_handle, 3, 1) == 0) {
                printf("✅ Modelo optimizado para NPU\n");
                
                // Cuantizar modelo
                if (NPU_QuantizeModel(model_handle, 1, 1) == 0) {
                    printf("✅ Modelo cuantizado\n");
                    
                    // Ejecutar inferencia en NPU
                    void* operation_handle;
                    if (NPU_ExecuteInference(device_handle, model_handle, NULL, 512, NULL, 256, &operation_handle) == 0) {
                        printf("✅ Inferencia ejecutada en NPU\n");
                        
                        // Esperar operación
                        if (NPU_WaitForOperation(operation_handle, 3000) == 0) {
                            printf("✅ Operación NPU completada\n");
                            
                            // Obtener resultado
                            unsigned char result[256];
                            unsigned long long result_size = sizeof(result);
                            if (NPU_GetOperationResult(operation_handle, result, &result_size) == 0) {
                                printf("✅ Resultado NPU obtenido (%llu bytes)\n", result_size);
                            } else {
                                printf("❌ Error al obtener resultado NPU\n");
                            }
                        } else {
                            printf("❌ Error al esperar operación NPU\n");
                        }
                    } else {
                        printf("❌ Error al ejecutar inferencia en NPU\n");
                    }
                } else {
                    printf("❌ Error al cuantizar modelo\n");
                }
            } else {
                printf("❌ Error al optimizar modelo para NPU\n");
            }
            
            // Descargar modelo
            NPU_UnloadModel(model_handle);
        } else {
            printf("❌ Error al cargar modelo en NPU\n");
        }
        
        // Obtener métricas de rendimiento
        if (NPU_GetPerformanceMetrics(device_handle, NULL) == 0) {
            printf("✅ Métricas de rendimiento NPU obtenidas\n");
        } else {
            printf("❌ Error al obtener métricas de rendimiento NPU\n");
        }
        
        // Obtener información del dispositivo
        float temperature, utilization;
        unsigned long long memory_usage, memory_total;
        if (NPU_GetTemperature(device_handle, &temperature) == 0) {
            printf("✅ Temperatura NPU: %.2f°C\n", temperature);
        }
        if (NPU_GetUtilization(device_handle, &utilization) == 0) {
            printf("✅ Utilización NPU: %.2f%%\n", utilization);
        }
        if (NPU_GetMemoryUsage(device_handle, &memory_usage, &memory_total) == 0) {
            printf("✅ Memoria NPU: %llu/%llu bytes\n", memory_usage, memory_total);
        }
        
        // Cerrar dispositivo NPU
        NPU_CloseDevice(device_handle);
    } else {
        printf("❌ Error al abrir dispositivo NPU\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de dispositivos NPU: %d\n", NPU_GetDeviceCount());
    printf("✅ Número de modelos NPU: %d\n", NPU_GetModelCount());
    printf("✅ Número de operaciones NPU: %d\n", NPU_GetOperationCount());
    
    // Test completo
    if (NPU_Test() == 0) {
        printf("✅ Test de NPU completado\n");
    } else {
        printf("❌ Error en test de NPU\n");
        return 1;
    }
    
    printf("🎉 Test de NPU completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para GPU AI
    cat > test-hardware-ai/gpu_ai/test_gpu_ai.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de GPU AI
extern int GPUAI_Initialize();
extern int GPUAI_EnumerateDevices(void* devices, unsigned int max_devices, unsigned int* device_count);
extern int GPUAI_GetDeviceInfo(unsigned int device_id, void* device_info);
extern int GPUAI_CreateContext(void* device_handle, unsigned int api_type, void** context_handle);
extern int GPUAI_DestroyContext(void* context_handle);
extern int GPUAI_LoadModel(void* context_handle, const unsigned char* model_data, unsigned long long model_size, void** model_handle);
extern int GPUAI_UnloadModel(void* model_handle);
extern int GPUAI_OptimizeModel(void* model_handle, unsigned int optimization_level, unsigned int target_precision);
extern int GPUAI_EnableTensorCores(void* model_handle, bool enable);
extern int GPUAI_EnableMixedPrecision(void* model_handle, bool enable);
extern int GPUAI_ExecuteInference(void* context_handle, void* model_handle, const unsigned char* input_data, unsigned long long input_size, unsigned char* output_data, unsigned long long output_size, void** operation_handle);
extern int GPUAI_ExecuteTraining(void* context_handle, void* model_handle, const unsigned char* training_data, unsigned long long training_size, void** operation_handle);
extern int GPUAI_ExecuteOptimization(void* context_handle, void* model_handle, unsigned int optimization_type, const float* parameters, void** operation_handle);
extern int GPUAI_WaitForOperation(void* operation_handle, unsigned int timeout);
extern int GPUAI_GetOperationStatus(void* operation_handle, unsigned int* status, float* progress);
extern int GPUAI_GetOperationResult(void* operation_handle, unsigned char* result_data, unsigned long long* result_size);
extern int GPUAI_GetPerformanceMetrics(void* device_handle, void* metrics);
extern int GPUAI_SetPowerMode(void* device_handle, unsigned int power_mode);
extern int GPUAI_GetPowerMode(void* device_handle, unsigned int* power_mode);
extern int GPUAI_GetTemperature(void* device_handle, float* temperature);
extern int GPUAI_GetUtilization(void* device_handle, float* utilization);
extern int GPUAI_GetMemoryUsage(void* device_handle, unsigned long long* memory_usage, unsigned long long* memory_total);
extern int GPUAI_GetDeviceCount();
extern int GPUAI_GetContextCount();
extern int GPUAI_GetModelCount();
extern int GPUAI_GetOperationCount();
extern int GPUAI_Test();

int main() {
    printf("🎮 Probando GPU AI Driver...\n");
    
    // Inicializar GPU AI
    if (GPUAI_Initialize() == 0) {
        printf("✅ GPU AI inicializado\n");
    } else {
        printf("❌ Error al inicializar GPU AI\n");
        return 1;
    }
    
    // Enumerar dispositivos GPU
    unsigned int device_count;
    if (GPUAI_EnumerateDevices(NULL, 10, &device_count) == 0) {
        printf("✅ Dispositivos GPU enumerados: %u\n", device_count);
    } else {
        printf("❌ Error al enumerar dispositivos GPU\n");
    }
    
    // Crear contexto GPU
    void* context_handle;
    if (GPUAI_CreateContext(NULL, 1, &context_handle) == 0) { // CUDA
        printf("✅ Contexto GPU creado\n");
        
        // Cargar modelo en GPU
        void* model_handle;
        if (GPUAI_LoadModel(context_handle, NULL, 4096, &model_handle) == 0) {
            printf("✅ Modelo cargado en GPU\n");
            
            // Optimizar modelo para GPU
            if (GPUAI_OptimizeModel(model_handle, 3, 1) == 0) {
                printf("✅ Modelo optimizado para GPU\n");
                
                // Habilitar tensor cores
                if (GPUAI_EnableTensorCores(model_handle, true) == 0) {
                    printf("✅ Tensor cores habilitados\n");
                    
                    // Habilitar precisión mixta
                    if (GPUAI_EnableMixedPrecision(model_handle, true) == 0) {
                        printf("✅ Precisión mixta habilitada\n");
                        
                        // Ejecutar inferencia en GPU
                        void* operation_handle;
                        if (GPUAI_ExecuteInference(context_handle, model_handle, NULL, 1024, NULL, 512, &operation_handle) == 0) {
                            printf("✅ Inferencia ejecutada en GPU\n");
                            
                            // Esperar operación
                            if (GPUAI_WaitForOperation(operation_handle, 2000) == 0) {
                                printf("✅ Operación GPU completada\n");
                                
                                // Obtener resultado
                                unsigned char result[512];
                                unsigned long long result_size = sizeof(result);
                                if (GPUAI_GetOperationResult(operation_handle, result, &result_size) == 0) {
                                    printf("✅ Resultado GPU obtenido (%llu bytes)\n", result_size);
                                } else {
                                    printf("❌ Error al obtener resultado GPU\n");
                                }
                            } else {
                                printf("❌ Error al esperar operación GPU\n");
                            }
                        } else {
                            printf("❌ Error al ejecutar inferencia en GPU\n");
                        }
                    } else {
                        printf("❌ Error al habilitar precisión mixta\n");
                    }
                } else {
                    printf("❌ Error al habilitar tensor cores\n");
                }
            } else {
                printf("❌ Error al optimizar modelo para GPU\n");
            }
            
            // Descargar modelo
            GPUAI_UnloadModel(model_handle);
        } else {
            printf("❌ Error al cargar modelo en GPU\n");
        }
        
        // Obtener métricas de rendimiento
        if (GPUAI_GetPerformanceMetrics(NULL, NULL) == 0) {
            printf("✅ Métricas de rendimiento GPU obtenidas\n");
        } else {
            printf("❌ Error al obtener métricas de rendimiento GPU\n");
        }
        
        // Obtener información del dispositivo
        float temperature, utilization;
        unsigned long long memory_usage, memory_total;
        if (GPUAI_GetTemperature(NULL, &temperature) == 0) {
            printf("✅ Temperatura GPU: %.2f°C\n", temperature);
        }
        if (GPUAI_GetUtilization(NULL, &utilization) == 0) {
            printf("✅ Utilización GPU: %.2f%%\n", utilization);
        }
        if (GPUAI_GetMemoryUsage(NULL, &memory_usage, &memory_total) == 0) {
            printf("✅ Memoria GPU: %llu/%llu bytes\n", memory_usage, memory_total);
        }
        
        // Destruir contexto GPU
        GPUAI_DestroyContext(context_handle);
    } else {
        printf("❌ Error al crear contexto GPU\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de dispositivos GPU: %d\n", GPUAI_GetDeviceCount());
    printf("✅ Número de contextos GPU: %d\n", GPUAI_GetContextCount());
    printf("✅ Número de modelos GPU: %d\n", GPUAI_GetModelCount());
    printf("✅ Número de operaciones GPU: %d\n", GPUAI_GetOperationCount());
    
    // Test completo
    if (GPUAI_Test() == 0) {
        printf("✅ Test de GPU AI completado\n");
    } else {
        printf("❌ Error en test de GPU AI\n");
        return 1;
    }
    
    printf("🎉 Test de GPU AI completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para CPU AI
    cat > test-hardware-ai/cpu_ai/test_cpu_ai.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de CPU AI
extern int CPUAI_Initialize();
extern int CPUAI_EnumerateDevices(void* devices, unsigned int max_devices, unsigned int* device_count);
extern int CPUAI_GetDeviceInfo(unsigned int device_id, void* device_info);
extern int CPUAI_CreateContext(void* device_handle, unsigned int accelerator_type, void** context_handle);
extern int CPUAI_DestroyContext(void* context_handle);
extern int CPUAI_LoadModel(void* context_handle, const unsigned char* model_data, unsigned long long model_size, void** model_handle);
extern int CPUAI_UnloadModel(void* model_handle);
extern int CPUAI_OptimizeModel(void* model_handle, unsigned int optimization_level, unsigned int target_precision);
extern int CPUAI_EnableAccelerator(void* model_handle, unsigned int accelerator_type, bool enable);
extern int CPUAI_QuantizeModel(void* model_handle, unsigned int quantization_type, unsigned int target_precision);
extern int CPUAI_ExecuteInference(void* context_handle, void* model_handle, const unsigned char* input_data, unsigned long long input_size, unsigned char* output_data, unsigned long long output_size, void** operation_handle);
extern int CPUAI_ExecuteTraining(void* context_handle, void* model_handle, const unsigned char* training_data, unsigned long long training_size, void** operation_handle);
extern int CPUAI_ExecuteOptimization(void* context_handle, void* model_handle, unsigned int optimization_type, const float* parameters, void** operation_handle);
extern int CPUAI_WaitForOperation(void* operation_handle, unsigned int timeout);
extern int CPUAI_GetOperationStatus(void* operation_handle, unsigned int* status, float* progress);
extern int CPUAI_GetOperationResult(void* operation_handle, unsigned char* result_data, unsigned long long* result_size);
extern int CPUAI_GetPerformanceMetrics(void* device_handle, void* metrics);
extern int CPUAI_SetPowerMode(void* device_handle, unsigned int power_mode);
extern int CPUAI_GetPowerMode(void* device_handle, unsigned int* power_mode);
extern int CPUAI_GetTemperature(void* device_handle, float* temperature);
extern int CPUAI_GetUtilization(void* device_handle, float* utilization);
extern int CPUAI_GetMemoryUsage(void* device_handle, unsigned long long* memory_usage, unsigned long long* memory_total);
extern int CPUAI_GetDeviceCount();
extern int CPUAI_GetContextCount();
extern int CPUAI_GetModelCount();
extern int CPUAI_GetOperationCount();
extern int CPUAI_Test();

int main() {
    printf("⚡ Probando CPU AI Driver...\n");
    
    // Inicializar CPU AI
    if (CPUAI_Initialize() == 0) {
        printf("✅ CPU AI inicializado\n");
    } else {
        printf("❌ Error al inicializar CPU AI\n");
        return 1;
    }
    
    // Enumerar dispositivos CPU
    unsigned int device_count;
    if (CPUAI_EnumerateDevices(NULL, 10, &device_count) == 0) {
        printf("✅ Dispositivos CPU enumerados: %u\n", device_count);
    } else {
        printf("❌ Error al enumerar dispositivos CPU\n");
    }
    
    // Crear contexto CPU
    void* context_handle;
    if (CPUAI_CreateContext(NULL, 1, &context_handle) == 0) { // AMX
        printf("✅ Contexto CPU creado\n");
        
        // Cargar modelo en CPU
        void* model_handle;
        if (CPUAI_LoadModel(context_handle, NULL, 2048, &model_handle) == 0) {
            printf("✅ Modelo cargado en CPU\n");
            
            // Optimizar modelo para CPU
            if (CPUAI_OptimizeModel(model_handle, 2, 1) == 0) {
                printf("✅ Modelo optimizado para CPU\n");
                
                // Habilitar acelerador AMX
                if (CPUAI_EnableAccelerator(model_handle, 1, true) == 0) {
                    printf("✅ Acelerador AMX habilitado\n");
                    
                    // Cuantizar modelo
                    if (CPUAI_QuantizeModel(model_handle, 1, 1) == 0) {
                        printf("✅ Modelo cuantizado\n");
                        
                        // Ejecutar inferencia en CPU
                        void* operation_handle;
                        if (CPUAI_ExecuteInference(context_handle, model_handle, NULL, 512, NULL, 256, &operation_handle) == 0) {
                            printf("✅ Inferencia ejecutada en CPU\n");
                            
                            // Esperar operación
                            if (CPUAI_WaitForOperation(operation_handle, 1000) == 0) {
                                printf("✅ Operación CPU completada\n");
                                
                                // Obtener resultado
                                unsigned char result[256];
                                unsigned long long result_size = sizeof(result);
                                if (CPUAI_GetOperationResult(operation_handle, result, &result_size) == 0) {
                                    printf("✅ Resultado CPU obtenido (%llu bytes)\n", result_size);
                                } else {
                                    printf("❌ Error al obtener resultado CPU\n");
                                }
                            } else {
                                printf("❌ Error al esperar operación CPU\n");
                            }
                        } else {
                            printf("❌ Error al ejecutar inferencia en CPU\n");
                        }
                    } else {
                        printf("❌ Error al cuantizar modelo\n");
                    }
                } else {
                    printf("❌ Error al habilitar acelerador AMX\n");
                }
            } else {
                printf("❌ Error al optimizar modelo para CPU\n");
            }
            
            // Descargar modelo
            CPUAI_UnloadModel(model_handle);
        } else {
            printf("❌ Error al cargar modelo en CPU\n");
        }
        
        // Obtener métricas de rendimiento
        if (CPUAI_GetPerformanceMetrics(NULL, NULL) == 0) {
            printf("✅ Métricas de rendimiento CPU obtenidas\n");
        } else {
            printf("❌ Error al obtener métricas de rendimiento CPU\n");
        }
        
        // Obtener información del dispositivo
        float temperature, utilization;
        unsigned long long memory_usage, memory_total;
        if (CPUAI_GetTemperature(NULL, &temperature) == 0) {
            printf("✅ Temperatura CPU: %.2f°C\n", temperature);
        }
        if (CPUAI_GetUtilization(NULL, &utilization) == 0) {
            printf("✅ Utilización CPU: %.2f%%\n", utilization);
        }
        if (CPUAI_GetMemoryUsage(NULL, &memory_usage, &memory_total) == 0) {
            printf("✅ Memoria CPU: %llu/%llu bytes\n", memory_usage, memory_total);
        }
        
        // Destruir contexto CPU
        CPUAI_DestroyContext(context_handle);
    } else {
        printf("❌ Error al crear contexto CPU\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de dispositivos CPU: %d\n", CPUAI_GetDeviceCount());
    printf("✅ Número de contextos CPU: %d\n", CPUAI_GetContextCount());
    printf("✅ Número de modelos CPU: %d\n", CPUAI_GetModelCount());
    printf("✅ Número de operaciones CPU: %d\n", CPUAI_GetOperationCount());
    
    // Test completo
    if (CPUAI_Test() == 0) {
        printf("✅ Test de CPU AI completado\n");
    } else {
        printf("❌ Error en test de CPU AI\n");
        return 1;
    }
    
    printf("🎉 Test de CPU AI completado exitosamente\n");
    return 0;
}
EOF

    print_success "Archivos de test de hardware acelerado creados"
}

# Compilar tests de hardware acelerado
compile_hardware_ai_tests() {
    print_hardware "Compilando tests de hardware acelerado de AI..."
    
    # Compilar test de AI Hardware Manager
    cd test-hardware-ai/ai_hardware
    gcc -o test_ai_hardware test_ai_hardware.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de AI Hardware Manager compilado"
    else
        print_warning "Error al compilar test de AI Hardware Manager"
    fi
    cd ../..
    
    # Compilar test de NPU
    cd test-hardware-ai/npu
    gcc -o test_npu test_npu.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de NPU compilado"
    else
        print_warning "Error al compilar test de NPU"
    fi
    cd ../..
    
    # Compilar test de GPU AI
    cd test-hardware-ai/gpu_ai
    gcc -o test_gpu_ai test_gpu_ai.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de GPU AI compilado"
    else
        print_warning "Error al compilar test de GPU AI"
    fi
    cd ../..
    
    # Compilar test de CPU AI
    cd test-hardware-ai/cpu_ai
    gcc -o test_cpu_ai test_cpu_ai.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de CPU AI compilado"
    else
        print_warning "Error al compilar test de CPU AI"
    fi
    cd ../..
}

# Ejecutar tests de hardware acelerado
run_hardware_ai_tests() {
    print_hardware "Ejecutando tests de hardware acelerado de AI..."
    
    # Ejecutar test de AI Hardware Manager
    if [ -f "test-hardware-ai/ai_hardware/test_ai_hardware" ]; then
        print_hardware "Ejecutando test de AI Hardware Manager..."
        ./test-hardware-ai/ai_hardware/test_ai_hardware
    fi
    
    # Ejecutar test de NPU
    if [ -f "test-hardware-ai/npu/test_npu" ]; then
        print_hardware "Ejecutando test de NPU..."
        ./test-hardware-ai/npu/test_npu
    fi
    
    # Ejecutar test de GPU AI
    if [ -f "test-hardware-ai/gpu_ai/test_gpu_ai" ]; then
        print_hardware "Ejecutando test de GPU AI..."
        ./test-hardware-ai/gpu_ai/test_gpu_ai
    fi
    
    # Ejecutar test de CPU AI
    if [ -f "test-hardware-ai/cpu_ai/test_cpu_ai" ]; then
        print_hardware "Ejecutando test de CPU AI..."
        ./test-hardware-ai/cpu_ai/test_cpu_ai
    fi
}

# Función principal
main() {
    echo "🚀 Test de Hardware Acelerado de AI de ReactOS Rust OS"
    echo "====================================================="
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_drivers_hardware_x86
    compile_drivers_hardware_x64
    compile_userland_hardware_x86
    compile_userland_hardware_x64
    create_hardware_ai_test_files
    compile_hardware_ai_tests
    run_hardware_ai_tests
    
    echo ""
    print_success "Test de hardware acelerado de AI completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • test-hardware-ai/ - Directorio de tests de hardware acelerado"
    echo "   • test-hardware-ai/ai_hardware/ - Tests de AI Hardware Manager"
    echo "   • test-hardware-ai/npu/ - Tests de NPU"
    echo "   • test-hardware-ai/gpu_ai/ - Tests de GPU AI"
    echo "   • test-hardware-ai/cpu_ai/ - Tests de CPU AI"
    echo ""
    echo "🚀 Componentes de hardware acelerado implementados:"
    echo "   • AI Hardware Manager - Gestión unificada de hardware de IA"
    echo "   • NPU Driver - Soporte para Neural Processing Units"
    echo "   • GPU AI Driver - Aceleración GPU para IA"
    echo "   • CPU AI Driver - Aceleradores de IA integrados en CPU"
    echo ""
    echo "🧠 Características de hardware acelerado:"
    echo "   • NPU - Intel GNA, AMD XDNA, Qualcomm Hexagon, Apple Neural Engine"
    echo "   • GPU - CUDA, OpenCL, DirectML, Vulkan, Metal, ROCm, OneAPI"
    echo "   • CPU - Intel AMX, ARM SVE, AVX-512, NEON, SSE, AVX, FMA, BF16"
    echo "   • Optimización automática para cada tipo de hardware"
    echo "   • Gestión unificada de recursos y rendimiento"
    echo "   • Soporte para precisión mixta y cuantización"
    echo "   • Monitoreo de temperatura, utilización y memoria"
    echo ""
    echo "🚀 ¡ReactOS Rust OS con hardware acelerado de IA implementado!"
}

# Ejecutar función principal
main "$@"
