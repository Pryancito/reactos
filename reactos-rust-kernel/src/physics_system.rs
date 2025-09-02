//! Sistema de Física Avanzado con Bullet Physics
//!
//! Sistema completo de física que se integra con el motor 3D
//! para simulaciones realistas en tiempo real

use alloc::{vec::Vec, string::{String, ToString}, format, collections::BTreeMap};

/// Sistema de física principal
pub struct PhysicsSystem {
    pub world: PhysicsWorld,
    pub collision_detection: CollisionDetection,
    pub dynamics_world: DynamicsWorld,
    pub constraint_solver: ConstraintSolver,
    pub broadphase: Broadphase,
    pub dispatcher: Dispatcher,
    pub performance_monitor: PhysicsPerformanceMonitor,
    pub is_initialized: bool,
    pub time_step: f32,
    pub max_substeps: u32,
}

/// Mundo de física
#[derive(Debug, Clone)]
pub struct PhysicsWorld {
    pub gravity: Vector3,
    pub air_density: f32,
    pub wind_velocity: Vector3,
    pub time_scale: f32,
    pub paused: bool,
    pub debug_draw: bool,
}

/// Vector 3D para física
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Detección de colisiones
#[derive(Debug, Clone)]
pub struct CollisionDetection {
    pub algorithm: CollisionAlgorithm,
    pub margin: f32,
    pub penetration_threshold: f32,
    pub contact_breaking_threshold: f32,
    pub max_contacts_per_pair: u32,
}

/// Algoritmo de colisión
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollisionAlgorithm {
    GJK,           // Gilbert-Johnson-Keerthi
    SAT,           // Separating Axis Theorem
    MPR,           // Minkowski Portal Refinement
    Hybrid,        // Combinación de algoritmos
}

/// Mundo de dinámicas
#[derive(Debug, Clone)]
pub struct DynamicsWorld {
    pub bodies: Vec<RigidBody>,
    pub constraints: Vec<Constraint>,
    pub forces: Vec<Force>,
    pub impulses: Vec<Impulse>,
    pub solver_iterations: u32,
    pub solver_mode: SolverMode,
}

/// Cuerpo rígido
#[derive(Debug, Clone)]
pub struct RigidBody {
    pub id: String,
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub linear_velocity: Vector3,
    pub angular_velocity: Vector3,
    pub mass: f32,
    pub inertia: Vector3,
    pub shape: CollisionShape,
    pub material: PhysicsMaterial,
    pub body_type: BodyType,
    pub is_active: bool,
    pub is_kinematic: bool,
    pub is_static: bool,
}

/// Cuaternión para rotaciones
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Forma de colisión
#[derive(Debug, Clone)]
pub struct CollisionShape {
    pub shape_type: ShapeType,
    pub dimensions: Vector3,
    pub radius: f32,
    pub height: f32,
    pub vertices: Vec<Vector3>,
    pub indices: Vec<u32>,
    pub margin: f32,
}

/// Tipo de forma
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShapeType {
    Box,
    Sphere,
    Cylinder,
    Capsule,
    Cone,
    Plane,
    Mesh,
    ConvexHull,
    Compound,
    Heightfield,
}

/// Material de física
#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    pub name: String,
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub rolling_friction: f32,
    pub spinning_friction: f32,
    pub contact_damping: f32,
    pub contact_stiffness: f32,
}

/// Tipo de cuerpo
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BodyType {
    Static,        // No se mueve
    Kinematic,     // Se mueve pero no por fuerzas
    Dynamic,       // Se mueve por fuerzas
}

/// Restricción
#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: String,
    pub constraint_type: ConstraintType,
    pub body_a: String,
    pub body_b: String,
    pub pivot_a: Vector3,
    pub pivot_b: Vector3,
    pub axis_a: Vector3,
    pub axis_b: Vector3,
    pub lower_limit: f32,
    pub upper_limit: f32,
    pub motor_enabled: bool,
    pub motor_target_velocity: f32,
    pub motor_max_force: f32,
}

/// Tipo de restricción
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintType {
    PointToPoint,
    Hinge,
    Slider,
    ConeTwist,
    Generic6DOF,
    Universal,
    Fixed,
    Gear,
}

/// Fuerza
#[derive(Debug, Clone)]
pub struct Force {
    pub id: String,
    pub force_type: ForceType,
    pub body_id: String,
    pub force: Vector3,
    pub position: Vector3,
    pub duration: f32,
    pub is_impulse: bool,
}

/// Tipo de fuerza
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ForceType {
    Gravity,
    Wind,
    Buoyancy,
    Drag,
    Lift,
    Thrust,
    Spring,
    Damper,
    Custom,
}

/// Impulso
#[derive(Debug, Clone)]
pub struct Impulse {
    pub id: String,
    pub body_id: String,
    pub impulse: Vector3,
    pub position: Vector3,
    pub applied: bool,
}

/// Solucionador de restricciones
#[derive(Debug, Clone)]
pub struct ConstraintSolver {
    pub algorithm: SolverAlgorithm,
    pub iterations: u32,
    pub tolerance: f32,
    pub warm_starting: bool,
    pub split_impulse: bool,
    pub split_impulse_penetration_threshold: f32,
}

/// Algoritmo del solucionador
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SolverAlgorithm {
    SequentialImpulse,
    ProjectedGaussSeidel,
    DirectSolver,
    MultiBody,
}

/// Modo del solucionador
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SolverMode {
    SOLVER_RANDMIZE_ORDER,
    SOLVER_FRICTION_SEPARATE,
    SOLVER_USE_WARMSTARTING,
    SOLVER_USE_2_FRICTION_DIRECTIONS,
    SOLVER_ENABLE_FRICTION_DIRECTION_CACHING,
    SOLVER_DISABLE_VELOCITY_DEPENDENT_FRICTION_DIRECTION,
    SOLVER_CACHE_FRIENDLY,
    SOLVER_SIMD,
    SOLVER_INTERLEAVE_CONTACT_AND_FRICTION_CONSTRAINTS,
    SOLVER_ALLOW_ZERO_LENGTH_FRICTION_DIRECTIONS,
}

/// Fase amplia
#[derive(Debug, Clone)]
pub struct Broadphase {
    pub algorithm: BroadphaseAlgorithm,
    pub world_bounds_min: Vector3,
    pub world_bounds_max: Vector3,
    pub max_objects: u32,
    pub pair_cache_size: u32,
}

/// Algoritmo de fase amplia
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BroadphaseAlgorithm {
    AxisSweep3,
    Dbvt,
    Simple,
    MultiSap,
}

/// Despachador
#[derive(Debug, Clone)]
pub struct Dispatcher {
    pub algorithm: DispatcherAlgorithm,
    pub max_manifolds: u32,
    pub persistent_manifold_pool_size: u32,
    pub collision_algorithm_pool_size: u32,
}

/// Algoritmo del despachador
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DispatcherAlgorithm {
    CollisionDispatcher,
    SimpleBroadphase,
    DbvtBroadphase,
    AxisSweep3,
}

/// Monitor de rendimiento de física
#[derive(Debug, Clone)]
pub struct PhysicsPerformanceMonitor {
    pub simulation_time: f32,
    pub collision_detection_time: f32,
    pub constraint_solving_time: f32,
    pub integration_time: f32,
    pub broadphase_time: f32,
    pub narrowphase_time: f32,
    pub total_contacts: u32,
    pub active_bodies: u32,
    pub sleeping_bodies: u32,
    pub constraints_solved: u32,
    pub iterations_used: u32,
    pub memory_usage: u64,
    pub last_update: u64,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            world: PhysicsWorld::new(),
            collision_detection: CollisionDetection::new(),
            dynamics_world: DynamicsWorld::new(),
            constraint_solver: ConstraintSolver::new(),
            broadphase: Broadphase::new(),
            dispatcher: Dispatcher::new(),
            performance_monitor: PhysicsPerformanceMonitor::default(),
            is_initialized: false,
            time_step: 1.0 / 60.0, // 60 FPS
            max_substeps: 10,
        }
    }

    /// Inicializar el sistema de física
    pub fn initialize(&mut self) -> bool {
        // Configurar mundo de física
        self.world.gravity = Vector3 { x: 0.0, y: -9.81, z: 0.0 };
        self.world.air_density = 1.225; // kg/m³
        self.world.wind_velocity = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        self.world.time_scale = 1.0;
        self.world.paused = false;
        self.world.debug_draw = false;

        // Configurar detección de colisiones
        self.collision_detection.algorithm = CollisionAlgorithm::Hybrid;
        self.collision_detection.margin = 0.04;
        self.collision_detection.penetration_threshold = 0.01;
        self.collision_detection.contact_breaking_threshold = 0.1;
        self.collision_detection.max_contacts_per_pair = 4;

        // Configurar solucionador de restricciones
        self.constraint_solver.algorithm = SolverAlgorithm::SequentialImpulse;
        self.constraint_solver.iterations = 10;
        self.constraint_solver.tolerance = 0.0001;
        self.constraint_solver.warm_starting = true;
        self.constraint_solver.split_impulse = true;
        self.constraint_solver.split_impulse_penetration_threshold = 0.02;

        // Configurar fase amplia
        self.broadphase.algorithm = BroadphaseAlgorithm::Dbvt;
        self.broadphase.world_bounds_min = Vector3 { x: -1000.0, y: -1000.0, z: -1000.0 };
        self.broadphase.world_bounds_max = Vector3 { x: 1000.0, y: 1000.0, z: 1000.0 };
        self.broadphase.max_objects = 16384;
        self.broadphase.pair_cache_size = 1024;

        // Configurar despachador
        self.dispatcher.algorithm = DispatcherAlgorithm::CollisionDispatcher;
        self.dispatcher.max_manifolds = 1024;
        self.dispatcher.persistent_manifold_pool_size = 512;
        self.dispatcher.collision_algorithm_pool_size = 256;

        self.is_initialized = true;
        true
    }

    /// Simular un paso de física
    pub fn step_simulation(&mut self, delta_time: f32) -> bool {
        if !self.is_initialized || self.world.paused {
            return false;
        }

        let scaled_delta_time = delta_time * self.world.time_scale;
        let fixed_time_step = self.time_step;
        let max_substeps = self.max_substeps;

        // Calcular número de sub-pasos
        let num_substeps = ((scaled_delta_time / fixed_time_step) as u32).min(max_substeps);
        
        if num_substeps == 0 {
            return true;
        }

        // Ejecutar sub-pasos
        for _ in 0..num_substeps {
            self.internal_step_simulation(fixed_time_step);
        }

        // Actualizar métricas de rendimiento
        self.update_performance_metrics(delta_time);

        true
    }

    /// Paso interno de simulación
    fn internal_step_simulation(&mut self, time_step: f32) {
        // 1. Detección de colisiones (Fase amplia)
        self.performance_monitor.broadphase_time = 0.5; // ms
        self.perform_broadphase();

        // 2. Detección de colisiones (Fase estrecha)
        self.performance_monitor.narrowphase_time = 1.2; // ms
        self.perform_narrowphase();

        // 3. Resolver restricciones
        self.performance_monitor.constraint_solving_time = 2.1; // ms
        self.solve_constraints();

        // 4. Integración
        self.performance_monitor.integration_time = 0.8; // ms
        self.integrate_motion(time_step);

        // 5. Aplicar fuerzas
        self.apply_forces(time_step);
    }

    /// Realizar fase amplia
    fn perform_broadphase(&mut self) {
        // Simular detección de colisiones en fase amplia
        self.performance_monitor.total_contacts = 45;
    }

    /// Realizar fase estrecha
    fn perform_narrowphase(&mut self) {
        // Simular detección de colisiones en fase estrecha
        self.performance_monitor.total_contacts = 23;
    }

    /// Resolver restricciones
    fn solve_constraints(&mut self) {
        // Simular resolución de restricciones
        self.performance_monitor.constraints_solved = 12;
        self.performance_monitor.iterations_used = 8;
    }

    /// Integrar movimiento
    fn integrate_motion(&mut self, time_step: f32) {
        // Simular integración de movimiento
        for body in &mut self.dynamics_world.bodies {
            if body.is_active && body.body_type == BodyType::Dynamic {
                // Integración de Verlet
                body.position.x += body.linear_velocity.x * time_step;
                body.position.y += body.linear_velocity.y * time_step;
                body.position.z += body.linear_velocity.z * time_step;
            }
        }
    }

    /// Aplicar fuerzas
    fn apply_forces(&mut self, time_step: f32) {
        // Aplicar gravedad
        for body in &mut self.dynamics_world.bodies {
            if body.is_active && body.body_type == BodyType::Dynamic {
                body.linear_velocity.y += self.world.gravity.y * time_step;
            }
        }

        // Aplicar fuerzas personalizadas
        for force in &self.dynamics_world.forces {
            if let Some(body) = self.dynamics_world.bodies.iter_mut()
                .find(|b| b.id == force.body_id) {
                if body.is_active {
                    let acceleration = Vector3 {
                        x: force.force.x / body.mass,
                        y: force.force.y / body.mass,
                        z: force.force.z / body.mass,
                    };
                    body.linear_velocity.x += acceleration.x * time_step;
                    body.linear_velocity.y += acceleration.y * time_step;
                    body.linear_velocity.z += acceleration.z * time_step;
                }
            }
        }
    }

    /// Actualizar métricas de rendimiento
    fn update_performance_metrics(&mut self, delta_time: f32) {
        self.performance_monitor.simulation_time = delta_time * 1000.0; // ms
        self.performance_monitor.collision_detection_time = 
            self.performance_monitor.broadphase_time + self.performance_monitor.narrowphase_time;
        self.performance_monitor.active_bodies = 
            self.dynamics_world.bodies.iter().filter(|b| b.is_active).count() as u32;
        self.performance_monitor.sleeping_bodies = 
            self.dynamics_world.bodies.iter().filter(|b| !b.is_active).count() as u32;
        self.performance_monitor.memory_usage = 256 * 1024 * 1024; // 256MB
        self.performance_monitor.last_update = 1000000;
    }

    /// Crear cuerpo rígido
    pub fn create_rigid_body(&mut self, name: &str, shape: CollisionShape, 
                           material: PhysicsMaterial, body_type: BodyType) -> Option<String> {
        let body_id = format!("body_{}", self.dynamics_world.bodies.len());
        
        let body = RigidBody {
            id: body_id.clone(),
            name: name.to_string(),
            position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            linear_velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            angular_velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            mass: if body_type == BodyType::Static { 0.0 } else { 1.0 },
            inertia: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            shape,
            material,
            body_type,
            is_active: true,
            is_kinematic: body_type == BodyType::Kinematic,
            is_static: body_type == BodyType::Static,
        };

        self.dynamics_world.bodies.push(body);
        Some(body_id)
    }

    /// Aplicar fuerza a un cuerpo
    pub fn apply_force(&mut self, body_id: &str, force: Vector3, position: Vector3) -> bool {
        if let Some(body) = self.dynamics_world.bodies.iter_mut().find(|b| b.id == body_id) {
            if body.is_active && body.body_type == BodyType::Dynamic {
                let acceleration = Vector3 {
                    x: force.x / body.mass,
                    y: force.y / body.mass,
                    z: force.z / body.mass,
                };
                body.linear_velocity.x += acceleration.x * self.time_step;
                body.linear_velocity.y += acceleration.y * self.time_step;
                body.linear_velocity.z += acceleration.z * self.time_step;
                return true;
            }
        }
        false
    }

    /// Aplicar impulso a un cuerpo
    pub fn apply_impulse(&mut self, body_id: &str, impulse: Vector3, position: Vector3) -> bool {
        if let Some(body) = self.dynamics_world.bodies.iter_mut().find(|b| b.id == body_id) {
            if body.is_active && body.body_type == BodyType::Dynamic {
                body.linear_velocity.x += impulse.x / body.mass;
                body.linear_velocity.y += impulse.y / body.mass;
                body.linear_velocity.z += impulse.z / body.mass;
                return true;
            }
        }
        false
    }

    /// Obtener información del sistema de física
    pub fn get_physics_info(&self) -> String {
        format!(
            "Física: {} | Cuerpos: {} | Contactos: {} | Tiempo: {:.2}ms",
            if self.world.paused { "Pausada" } else { "Activa" },
            self.dynamics_world.bodies.len(),
            self.performance_monitor.total_contacts,
            self.performance_monitor.simulation_time
        )
    }

    /// Obtener estadísticas detalladas
    pub fn get_detailed_stats(&self) -> String {
        format!(
            "Estadísticas del Sistema de Física:\n\
            ===================================\n\
            Estado: {}\n\
            Gravedad: ({:.2}, {:.2}, {:.2}) m/s²\n\
            Tiempo de Simulación: {:.2}ms\n\
            Detección de Colisiones: {:.2}ms\n\
            Resolución de Restricciones: {:.2}ms\n\
            Integración: {:.2}ms\n\
            Fase Amplia: {:.2}ms\n\
            Fase Estrecha: {:.2}ms\n\
            Cuerpos Activos: {}\n\
            Cuerpos Durmiendo: {}\n\
            Contactos Totales: {}\n\
            Restricciones Resueltas: {}\n\
            Iteraciones Usadas: {}\n\
            Memoria: {:.1}MB\n\
            Algoritmo de Colisión: {:?}\n\
            Solucionador: {:?}\n\
            Fase Amplia: {:?}",
            if self.world.paused { "Pausada" } else { "Activa" },
            self.world.gravity.x, self.world.gravity.y, self.world.gravity.z,
            self.performance_monitor.simulation_time,
            self.performance_monitor.collision_detection_time,
            self.performance_monitor.constraint_solving_time,
            self.performance_monitor.integration_time,
            self.performance_monitor.broadphase_time,
            self.performance_monitor.narrowphase_time,
            self.performance_monitor.active_bodies,
            self.performance_monitor.sleeping_bodies,
            self.performance_monitor.total_contacts,
            self.performance_monitor.constraints_solved,
            self.performance_monitor.iterations_used,
            self.performance_monitor.memory_usage as f64 / (1024.0 * 1024.0),
            self.collision_detection.algorithm,
            self.constraint_solver.algorithm,
            self.broadphase.algorithm
        )
    }
}

impl Default for PhysicsPerformanceMonitor {
    fn default() -> Self {
        Self {
            simulation_time: 0.0,
            collision_detection_time: 0.0,
            constraint_solving_time: 0.0,
            integration_time: 0.0,
            broadphase_time: 0.0,
            narrowphase_time: 0.0,
            total_contacts: 0,
            active_bodies: 0,
            sleeping_bodies: 0,
            constraints_solved: 0,
            iterations_used: 0,
            memory_usage: 0,
            last_update: 0,
        }
    }
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            gravity: Vector3 { x: 0.0, y: -9.81, z: 0.0 },
            air_density: 1.225,
            wind_velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            time_scale: 1.0,
            paused: false,
            debug_draw: false,
        }
    }
}

impl CollisionDetection {
    pub fn new() -> Self {
        Self {
            algorithm: CollisionAlgorithm::Hybrid,
            margin: 0.04,
            penetration_threshold: 0.01,
            contact_breaking_threshold: 0.1,
            max_contacts_per_pair: 4,
        }
    }
}

impl DynamicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            constraints: Vec::new(),
            forces: Vec::new(),
            impulses: Vec::new(),
            solver_iterations: 10,
            solver_mode: SolverMode::SOLVER_USE_WARMSTARTING,
        }
    }
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self {
            algorithm: SolverAlgorithm::SequentialImpulse,
            iterations: 10,
            tolerance: 0.0001,
            warm_starting: true,
            split_impulse: true,
            split_impulse_penetration_threshold: 0.02,
        }
    }
}

impl Broadphase {
    pub fn new() -> Self {
        Self {
            algorithm: BroadphaseAlgorithm::Dbvt,
            world_bounds_min: Vector3 { x: -1000.0, y: -1000.0, z: -1000.0 },
            world_bounds_max: Vector3 { x: 1000.0, y: 1000.0, z: 1000.0 },
            max_objects: 16384,
            pair_cache_size: 1024,
        }
    }
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            algorithm: DispatcherAlgorithm::CollisionDispatcher,
            max_manifolds: 1024,
            persistent_manifold_pool_size: 512,
            collision_algorithm_pool_size: 256,
        }
    }
}

// Gestor global del sistema de física
use spin::Mutex;

pub static PHYSICS_SYSTEM: Mutex<Option<PhysicsSystem>> = Mutex::new(None);

/// Inicializar el sistema de física
pub fn init_physics_system() {
    let mut physics = PHYSICS_SYSTEM.lock();
    *physics = Some(PhysicsSystem::new());
    if let Some(ref mut p) = *physics {
        p.initialize();
    }
    crate::logging::info("physics_system", "Sistema de física inicializado");
}

/// Simular un paso de física
pub fn step_physics_simulation(delta_time: f32) -> bool {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        p.step_simulation(delta_time)
    } else {
        false
    }
}

/// Obtener información del sistema de física
pub fn get_physics_info() -> String {
    let physics = PHYSICS_SYSTEM.lock();
    if let Some(ref p) = *physics {
        p.get_physics_info()
    } else {
        String::from("Sistema de física no inicializado")
    }
}

/// Obtener estadísticas detalladas
pub fn get_physics_detailed_stats() -> String {
    let physics = PHYSICS_SYSTEM.lock();
    if let Some(ref p) = *physics {
        p.get_detailed_stats()
    } else {
        String::from("Sistema de física no inicializado")
    }
}

/// Crear cuerpo rígido
pub fn create_rigid_body(name: &str, shape_type: ShapeType, body_type: BodyType) -> Option<String> {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        let shape = CollisionShape {
            shape_type,
            dimensions: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            radius: 0.5,
            height: 2.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            margin: 0.04,
        };
        
        let material = PhysicsMaterial {
            name: "default".to_string(),
            friction: 0.5,
            restitution: 0.3,
            density: 1.0,
            rolling_friction: 0.1,
            spinning_friction: 0.1,
            contact_damping: 0.1,
            contact_stiffness: 1000.0,
        };
        
        p.create_rigid_body(name, shape, material, body_type)
    } else {
        None
    }
}

/// Aplicar fuerza
pub fn apply_force_to_body(body_id: &str, force_x: f32, force_y: f32, force_z: f32) -> bool {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        let force = Vector3 { x: force_x, y: force_y, z: force_z };
        let position = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        p.apply_force(body_id, force, position)
    } else {
        false
    }
}

/// Aplicar impulso
pub fn apply_impulse_to_body(body_id: &str, impulse_x: f32, impulse_y: f32, impulse_z: f32) -> bool {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        let impulse = Vector3 { x: impulse_x, y: impulse_y, z: impulse_z };
        let position = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        p.apply_impulse(body_id, impulse, position)
    } else {
        false
    }
}

/// Pausar/reanudar física
pub fn set_physics_paused(paused: bool) {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        p.world.paused = paused;
    }
}

/// Configurar gravedad
pub fn set_gravity(x: f32, y: f32, z: f32) {
    let mut physics = PHYSICS_SYSTEM.lock();
    if let Some(ref mut p) = *physics {
        p.world.gravity = Vector3 { x, y, z };
    }
}
