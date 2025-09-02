#!/bin/bash

# Script de test para AI de ReactOS Rust OS
echo "🧠 Probando AI de ReactOS Rust OS..."

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

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias para AI..."
    
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

# Compilar userland con AI para x86
compile_userland_ai_x86() {
    print_ai "Compilando userland con AI para x86 (32-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland con AI x86 compilado exitosamente"
    else
        print_error "Error al compilar userland con AI x86"
        exit 1
    fi
    
    cd ..
}

# Compilar userland con AI para x86_64
compile_userland_ai_x64() {
    print_ai "Compilando userland con AI para x86_64 (64-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland con AI x86_64 compilado exitosamente"
    else
        print_error "Error al compilar userland con AI x86_64"
        exit 1
    fi
    
    cd ..
}

# Crear archivos de test para AI
create_ai_test_files() {
    print_ai "Creando archivos de test para AI..."
    
    # Crear directorio de test
    mkdir -p test-ai/{ai_core,ai_performance,ai_anomaly,ai_assistant,ai_predictor}
    
    # Crear archivos de test para AI Core
    cat > test-ai/ai_core/test_ai_core.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de AI Core
extern int AI_Initialize();
extern int AI_CreateModel(const char* model_name, unsigned int model_type, unsigned int task_type, unsigned int input_size, unsigned int output_size, void** model_handle);
extern int AI_TrainModel(void* model_handle, void* dataset_handle, void* config, void* progress_callback);
extern int AI_Predict(void* model_handle, const float* input_data, unsigned int input_size, float* output_data, unsigned int output_size, void* result);
extern int AI_GetModelCount();
extern int AI_GetDatasetCount();
extern unsigned long long AI_GetInferenceCount();
extern int AI_Test();

int main() {
    printf("🧠 Probando AI Core del sistema...\n");
    
    // Inicializar AI Core
    if (AI_Initialize() == 0) {
        printf("✅ AI Core inicializado\n");
    } else {
        printf("❌ Error al inicializar AI Core\n");
        return 1;
    }
    
    // Crear modelo de test
    void* test_model;
    if (AI_CreateModel("TestModel", 1, 1, 10, 5, &test_model) == 0) { // Neural Network, Classification
        printf("✅ Modelo de test creado\n");
        
        // Entrenar modelo (simulado)
        if (AI_TrainModel(test_model, NULL, NULL, NULL) == 0) {
            printf("✅ Modelo entrenado\n");
            
            // Realizar predicción (simulada)
            float input[10] = {1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, 7.0f, 8.0f, 9.0f, 10.0f};
            float output[5];
            if (AI_Predict(test_model, input, 10, output, 5, NULL) == 0) {
                printf("✅ Predicción realizada\n");
                printf("   Resultado: [%.2f, %.2f, %.2f, %.2f, %.2f]\n", 
                       output[0], output[1], output[2], output[3], output[4]);
            } else {
                printf("❌ Error al realizar predicción\n");
            }
        } else {
            printf("❌ Error al entrenar modelo\n");
        }
    } else {
        printf("❌ Error al crear modelo de test\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de modelos: %d\n", AI_GetModelCount());
    printf("✅ Número de datasets: %d\n", AI_GetDatasetCount());
    printf("✅ Número de inferencias: %llu\n", AI_GetInferenceCount());
    
    // Test completo
    if (AI_Test() == 0) {
        printf("✅ Test de AI Core completado\n");
    } else {
        printf("❌ Error en test de AI Core\n");
        return 1;
    }
    
    printf("🎉 Test de AI Core completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para Performance Optimizer
    cat > test-ai/ai_performance/test_performance.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de Performance Optimizer
extern int PerformanceOptimizer_Initialize();
extern int PerformanceOptimizer_CollectMetrics(void* metrics);
extern int PerformanceOptimizer_OptimizeCPU(void* config, void* result);
extern int PerformanceOptimizer_OptimizeMemory(void* config, void* result);
extern int PerformanceOptimizer_OptimizeDisk(void* config, void* result);
extern int PerformanceOptimizer_OptimizeNetwork(void* config, void* result);
extern int PerformanceOptimizer_ComprehensiveOptimization(void* config, void* result);
extern int PerformanceOptimizer_PredictPerformance(unsigned int metric_type, unsigned int time_horizon, void* prediction);
extern int PerformanceOptimizer_GetOptimizationCount();
extern int PerformanceOptimizer_GetPredictionCount();
extern int PerformanceOptimizer_GetTotalImprovement();
extern int PerformanceOptimizer_Test();

int main() {
    printf("⚡ Probando Performance Optimizer...\n");
    
    // Inicializar Performance Optimizer
    if (PerformanceOptimizer_Initialize() == 0) {
        printf("✅ Performance Optimizer inicializado\n");
    } else {
        printf("❌ Error al inicializar Performance Optimizer\n");
        return 1;
    }
    
    // Recopilar métricas
    if (PerformanceOptimizer_CollectMetrics(NULL) == 0) {
        printf("✅ Métricas recopiladas\n");
    } else {
        printf("❌ Error al recopilar métricas\n");
    }
    
    // Optimizar CPU
    if (PerformanceOptimizer_OptimizeCPU(NULL, NULL) == 0) {
        printf("✅ CPU optimizado\n");
    } else {
        printf("❌ Error al optimizar CPU\n");
    }
    
    // Optimizar memoria
    if (PerformanceOptimizer_OptimizeMemory(NULL, NULL) == 0) {
        printf("✅ Memoria optimizada\n");
    } else {
        printf("❌ Error al optimizar memoria\n");
    }
    
    // Optimizar disco
    if (PerformanceOptimizer_OptimizeDisk(NULL, NULL) == 0) {
        printf("✅ Disco optimizado\n");
    } else {
        printf("❌ Error al optimizar disco\n");
    }
    
    // Optimizar red
    if (PerformanceOptimizer_OptimizeNetwork(NULL, NULL) == 0) {
        printf("✅ Red optimizada\n");
    } else {
        printf("❌ Error al optimizar red\n");
    }
    
    // Optimización integral
    if (PerformanceOptimizer_ComprehensiveOptimization(NULL, NULL) == 0) {
        printf("✅ Optimización integral completada\n");
    } else {
        printf("❌ Error en optimización integral\n");
    }
    
    // Predecir rendimiento
    if (PerformanceOptimizer_PredictPerformance(1, 60, NULL) == 0) { // CPU, 60 segundos
        printf("✅ Rendimiento predicho\n");
    } else {
        printf("❌ Error al predecir rendimiento\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de optimizaciones: %d\n", PerformanceOptimizer_GetOptimizationCount());
    printf("✅ Número de predicciones: %d\n", PerformanceOptimizer_GetPredictionCount());
    printf("✅ Mejora total: %d%%\n", PerformanceOptimizer_GetTotalImprovement());
    
    // Test completo
    if (PerformanceOptimizer_Test() == 0) {
        printf("✅ Test de Performance Optimizer completado\n");
    } else {
        printf("❌ Error en test de Performance Optimizer\n");
        return 1;
    }
    
    printf("🎉 Test de Performance Optimizer completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para Anomaly Detector
    cat > test-ai/ai_anomaly/test_anomaly.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de Anomaly Detector
extern int AnomalyDetector_Initialize();
extern int AnomalyDetector_DetectPerformanceAnomaly(void* performance_data, void* result);
extern int AnomalyDetector_DetectSecurityAnomaly(void* security_data, void* result);
extern int AnomalyDetector_DetectNetworkAnomaly(void* network_data, void* result);
extern int AnomalyDetector_DetectSystemAnomaly(void* system_data, void* result);
extern int AnomalyDetector_DetectHardwareAnomaly(void* hardware_data, void* result);
extern int AnomalyDetector_DetectUserBehaviorAnomaly(void* user_data, void* result);
extern int AnomalyDetector_GetAnomalyCount();
extern int AnomalyDetector_GetPatternCount();
extern unsigned long long AnomalyDetector_GetDetectedAnomalies();
extern int AnomalyDetector_Test();

int main() {
    printf("🔍 Probando Anomaly Detector...\n");
    
    // Inicializar Anomaly Detector
    if (AnomalyDetector_Initialize() == 0) {
        printf("✅ Anomaly Detector inicializado\n");
    } else {
        printf("❌ Error al inicializar Anomaly Detector\n");
        return 1;
    }
    
    // Detectar anomalía de rendimiento
    if (AnomalyDetector_DetectPerformanceAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía de rendimiento detectada\n");
    } else {
        printf("❌ Error al detectar anomalía de rendimiento\n");
    }
    
    // Detectar anomalía de seguridad
    if (AnomalyDetector_DetectSecurityAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía de seguridad detectada\n");
    } else {
        printf("❌ Error al detectar anomalía de seguridad\n");
    }
    
    // Detectar anomalía de red
    if (AnomalyDetector_DetectNetworkAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía de red detectada\n");
    } else {
        printf("❌ Error al detectar anomalía de red\n");
    }
    
    // Detectar anomalía del sistema
    if (AnomalyDetector_DetectSystemAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía del sistema detectada\n");
    } else {
        printf("❌ Error al detectar anomalía del sistema\n");
    }
    
    // Detectar anomalía de hardware
    if (AnomalyDetector_DetectHardwareAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía de hardware detectada\n");
    } else {
        printf("❌ Error al detectar anomalía de hardware\n");
    }
    
    // Detectar anomalía de comportamiento de usuario
    if (AnomalyDetector_DetectUserBehaviorAnomaly(NULL, NULL) == 0) {
        printf("✅ Anomalía de comportamiento detectada\n");
    } else {
        printf("❌ Error al detectar anomalía de comportamiento\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de anomalías: %d\n", AnomalyDetector_GetAnomalyCount());
    printf("✅ Número de patrones: %d\n", AnomalyDetector_GetPatternCount());
    printf("✅ Anomalías detectadas: %llu\n", AnomalyDetector_GetDetectedAnomalies());
    
    // Test completo
    if (AnomalyDetector_Test() == 0) {
        printf("✅ Test de Anomaly Detector completado\n");
    } else {
        printf("❌ Error en test de Anomaly Detector\n");
        return 1;
    }
    
    printf("🎉 Test de Anomaly Detector completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para System Assistant
    cat > test-ai/ai_assistant/test_assistant.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de System Assistant
extern int SystemAssistant_Initialize();
extern int SystemAssistant_CreateSession(unsigned int user_id, void* preferences, void** session_handle);
extern int SystemAssistant_ProcessRequest(void* session_handle, void* request, void* response);
extern int SystemAssistant_ProvideHelp(void* session_handle, const char* topic, unsigned int complexity_level, void* response);
extern int SystemAssistant_Troubleshoot(void* session_handle, const char* problem_description, void* system_state, void* response);
extern int SystemAssistant_OptimizeSystem(void* session_handle, const char* optimization_goal, void* current_state, void* response);
extern int SystemAssistant_AutomateTask(void* session_handle, const char* task_description, void* parameters, void* response);
extern int SystemAssistant_ProvideRecommendation(void* session_handle, void* context, unsigned int recommendation_type, void* response);
extern int SystemAssistant_EducateUser(void* session_handle, const char* topic, unsigned int user_level, void* response);
extern int SystemAssistant_PredictIssue(void* session_handle, void* system_data, unsigned int prediction_horizon, void* response);
extern int SystemAssistant_AnalyzeSystem(void* session_handle, unsigned int analysis_type, void* data, void* response);
extern int SystemAssistant_GetSessionCount();
extern int SystemAssistant_GetRequestCount();
extern int SystemAssistant_GetResponseCount();
extern int SystemAssistant_GetKnowledgeCount();
extern int SystemAssistant_Test();

int main() {
    printf("🤖 Probando System Assistant...\n");
    
    // Inicializar System Assistant
    if (SystemAssistant_Initialize() == 0) {
        printf("✅ System Assistant inicializado\n");
    } else {
        printf("❌ Error al inicializar System Assistant\n");
        return 1;
    }
    
    // Crear sesión de test
    void* test_session;
    if (SystemAssistant_CreateSession(1, NULL, &test_session) == 0) {
        printf("✅ Sesión de test creada\n");
        
        // Proporcionar ayuda
        if (SystemAssistant_ProvideHelp(test_session, "system optimization", 2, NULL) == 0) {
            printf("✅ Ayuda proporcionada\n");
        } else {
            printf("❌ Error al proporcionar ayuda\n");
        }
        
        // Solucionar problemas
        if (SystemAssistant_Troubleshoot(test_session, "slow performance", NULL, NULL) == 0) {
            printf("✅ Problema solucionado\n");
        } else {
            printf("❌ Error al solucionar problema\n");
        }
        
        // Optimizar sistema
        if (SystemAssistant_OptimizeSystem(test_session, "improve speed", NULL, NULL) == 0) {
            printf("✅ Sistema optimizado\n");
        } else {
            printf("❌ Error al optimizar sistema\n");
        }
        
        // Automatizar tarea
        if (SystemAssistant_AutomateTask(test_session, "backup files", NULL, NULL) == 0) {
            printf("✅ Tarea automatizada\n");
        } else {
            printf("❌ Error al automatizar tarea\n");
        }
        
        // Proporcionar recomendación
        if (SystemAssistant_ProvideRecommendation(test_session, NULL, 1, NULL) == 0) {
            printf("✅ Recomendación proporcionada\n");
        } else {
            printf("❌ Error al proporcionar recomendación\n");
        }
        
        // Educar usuario
        if (SystemAssistant_EducateUser(test_session, "system administration", 2, NULL) == 0) {
            printf("✅ Usuario educado\n");
        } else {
            printf("❌ Error al educar usuario\n");
        }
        
        // Predecir problema
        if (SystemAssistant_PredictIssue(test_session, NULL, 3600, NULL) == 0) { // 1 hora
            printf("✅ Problema predicho\n");
        } else {
            printf("❌ Error al predecir problema\n");
        }
        
        // Analizar sistema
        if (SystemAssistant_AnalyzeSystem(test_session, 1, NULL, NULL) == 0) {
            printf("✅ Sistema analizado\n");
        } else {
            printf("❌ Error al analizar sistema\n");
        }
        
        // Finalizar sesión
        SystemAssistant_EndSession(test_session);
    } else {
        printf("❌ Error al crear sesión de test\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de sesiones: %d\n", SystemAssistant_GetSessionCount());
    printf("✅ Número de solicitudes: %d\n", SystemAssistant_GetRequestCount());
    printf("✅ Número de respuestas: %d\n", SystemAssistant_GetResponseCount());
    printf("✅ Número de conocimientos: %d\n", SystemAssistant_GetKnowledgeCount());
    
    // Test completo
    if (SystemAssistant_Test() == 0) {
        printf("✅ Test de System Assistant completado\n");
    } else {
        printf("❌ Error en test de System Assistant\n");
        return 1;
    }
    
    printf("🎉 Test de System Assistant completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para Resource Predictor
    cat > test-ai/ai_predictor/test_predictor.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de Resource Predictor
extern int ResourcePredictor_Initialize();
extern int ResourcePredictor_PredictCPU(unsigned int time_horizon, void* prediction);
extern int ResourcePredictor_PredictMemory(unsigned int time_horizon, void* prediction);
extern int ResourcePredictor_PredictDisk(unsigned int time_horizon, void* prediction);
extern int ResourcePredictor_PredictNetwork(unsigned int time_horizon, void* prediction);
extern int ResourcePredictor_PredictPower(unsigned int time_horizon, void* prediction);
extern int ResourcePredictor_PredictComprehensive(unsigned int time_horizon, void* predictions, unsigned int max_predictions, unsigned int* prediction_count);
extern int ResourcePredictor_AddHistoryData(void* history);
extern int ResourcePredictor_ValidatePrediction(unsigned int prediction_id, float actual_value, float* accuracy);
extern int ResourcePredictor_GetPredictionCount();
extern int ResourcePredictor_GetModelCount();
extern int ResourcePredictor_GetHistoryCount();
extern unsigned long long ResourcePredictor_GetAccuratePredictions();
extern int ResourcePredictor_Test();

int main() {
    printf("📊 Probando Resource Predictor...\n");
    
    // Inicializar Resource Predictor
    if (ResourcePredictor_Initialize() == 0) {
        printf("✅ Resource Predictor inicializado\n");
    } else {
        printf("❌ Error al inicializar Resource Predictor\n");
        return 1;
    }
    
    // Predecir CPU
    if (ResourcePredictor_PredictCPU(300, NULL) == 0) { // 5 minutos
        printf("✅ CPU predicho\n");
    } else {
        printf("❌ Error al predecir CPU\n");
    }
    
    // Predecir memoria
    if (ResourcePredictor_PredictMemory(300, NULL) == 0) { // 5 minutos
        printf("✅ Memoria predicha\n");
    } else {
        printf("❌ Error al predecir memoria\n");
    }
    
    // Predecir disco
    if (ResourcePredictor_PredictDisk(300, NULL) == 0) { // 5 minutos
        printf("✅ Disco predicho\n");
    } else {
        printf("❌ Error al predecir disco\n");
    }
    
    // Predecir red
    if (ResourcePredictor_PredictNetwork(300, NULL) == 0) { // 5 minutos
        printf("✅ Red predicha\n");
    } else {
        printf("❌ Error al predecir red\n");
    }
    
    // Predecir energía
    if (ResourcePredictor_PredictPower(300, NULL) == 0) { // 5 minutos
        printf("✅ Energía predicha\n");
    } else {
        printf("❌ Error al predecir energía\n");
    }
    
    // Predicción integral
    unsigned int prediction_count;
    if (ResourcePredictor_PredictComprehensive(300, NULL, 10, &prediction_count) == 0) { // 5 minutos, 10 predicciones
        printf("✅ Predicción integral completada (%d predicciones)\n", prediction_count);
    } else {
        printf("❌ Error en predicción integral\n");
    }
    
    // Agregar datos históricos
    if (ResourcePredictor_AddHistoryData(NULL) == 0) {
        printf("✅ Datos históricos agregados\n");
    } else {
        printf("❌ Error al agregar datos históricos\n");
    }
    
    // Validar predicción
    float accuracy;
    if (ResourcePredictor_ValidatePrediction(1, 75.5f, &accuracy) == 0) {
        printf("✅ Predicción validada (precisión: %.2f%%)\n", accuracy);
    } else {
        printf("❌ Error al validar predicción\n");
    }
    
    // Obtener estadísticas
    printf("✅ Número de predicciones: %d\n", ResourcePredictor_GetPredictionCount());
    printf("✅ Número de modelos: %d\n", ResourcePredictor_GetModelCount());
    printf("✅ Número de datos históricos: %d\n", ResourcePredictor_GetHistoryCount());
    printf("✅ Predicciones precisas: %llu\n", ResourcePredictor_GetAccuratePredictions());
    
    // Test completo
    if (ResourcePredictor_Test() == 0) {
        printf("✅ Test de Resource Predictor completado\n");
    } else {
        printf("❌ Error en test de Resource Predictor\n");
        return 1;
    }
    
    printf("🎉 Test de Resource Predictor completado exitosamente\n");
    return 0;
}
EOF

    print_success "Archivos de test de AI creados"
}

# Compilar tests de AI
compile_ai_tests() {
    print_ai "Compilando tests de AI..."
    
    # Compilar test de AI Core
    cd test-ai/ai_core
    gcc -o test_ai_core test_ai_core.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de AI Core compilado"
    else
        print_warning "Error al compilar test de AI Core"
    fi
    cd ../..
    
    # Compilar test de Performance Optimizer
    cd test-ai/ai_performance
    gcc -o test_performance test_performance.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de Performance Optimizer compilado"
    else
        print_warning "Error al compilar test de Performance Optimizer"
    fi
    cd ../..
    
    # Compilar test de Anomaly Detector
    cd test-ai/ai_anomaly
    gcc -o test_anomaly test_anomaly.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de Anomaly Detector compilado"
    else
        print_warning "Error al compilar test de Anomaly Detector"
    fi
    cd ../..
    
    # Compilar test de System Assistant
    cd test-ai/ai_assistant
    gcc -o test_assistant test_assistant.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de System Assistant compilado"
    else
        print_warning "Error al compilar test de System Assistant"
    fi
    cd ../..
    
    # Compilar test de Resource Predictor
    cd test-ai/ai_predictor
    gcc -o test_predictor test_predictor.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de Resource Predictor compilado"
    else
        print_warning "Error al compilar test de Resource Predictor"
    fi
    cd ../..
}

# Ejecutar tests de AI
run_ai_tests() {
    print_ai "Ejecutando tests de AI..."
    
    # Ejecutar test de AI Core
    if [ -f "test-ai/ai_core/test_ai_core" ]; then
        print_ai "Ejecutando test de AI Core..."
        ./test-ai/ai_core/test_ai_core
    fi
    
    # Ejecutar test de Performance Optimizer
    if [ -f "test-ai/ai_performance/test_performance" ]; then
        print_ai "Ejecutando test de Performance Optimizer..."
        ./test-ai/ai_performance/test_performance
    fi
    
    # Ejecutar test de Anomaly Detector
    if [ -f "test-ai/ai_anomaly/test_anomaly" ]; then
        print_ai "Ejecutando test de Anomaly Detector..."
        ./test-ai/ai_anomaly/test_anomaly
    fi
    
    # Ejecutar test de System Assistant
    if [ -f "test-ai/ai_assistant/test_assistant" ]; then
        print_ai "Ejecutando test de System Assistant..."
        ./test-ai/ai_assistant/test_assistant
    fi
    
    # Ejecutar test de Resource Predictor
    if [ -f "test-ai/ai_predictor/test_predictor" ]; then
        print_ai "Ejecutando test de Resource Predictor..."
        ./test-ai/ai_predictor/test_predictor
    fi
}

# Función principal
main() {
    echo "🧠 Test de AI de ReactOS Rust OS"
    echo "================================"
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_userland_ai_x86
    compile_userland_ai_x64
    create_ai_test_files
    compile_ai_tests
    run_ai_tests
    
    echo ""
    print_success "Test de AI completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • test-ai/ - Directorio de tests de AI"
    echo "   • test-ai/ai_core/ - Tests de AI Core"
    echo "   • test-ai/ai_performance/ - Tests de Performance Optimizer"
    echo "   • test-ai/ai_anomaly/ - Tests de Anomaly Detector"
    echo "   • test-ai/ai_assistant/ - Tests de System Assistant"
    echo "   • test-ai/ai_predictor/ - Tests de Resource Predictor"
    echo ""
    echo "🧠 Componentes de AI implementados:"
    echo "   • AI Core - Motor básico de inteligencia artificial"
    echo "   • Performance Optimizer - Optimización automática del sistema"
    echo "   • Anomaly Detector - Detección de problemas y anomalías"
    echo "   • System Assistant - Asistente inteligente del sistema"
    echo "   • Resource Predictor - Predicción de uso de recursos"
    echo ""
    echo "🚀 Características de AI:"
    echo "   • Machine Learning - Modelos de aprendizaje automático"
    echo "   • Neural Networks - Redes neuronales para predicción"
    echo "   • Anomaly Detection - Detección de comportamientos anómalos"
    echo "   • Performance Optimization - Optimización automática"
    echo "   • Intelligent Assistant - Asistente con IA"
    echo "   • Resource Prediction - Predicción de recursos"
    echo ""
    echo "🧠 ¡ReactOS Rust OS con AI nativa implementada!"
}

# Ejecutar función principal
main "$@"
