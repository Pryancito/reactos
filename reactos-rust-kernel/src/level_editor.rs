//! Editor de Niveles 3D Integrado
//!
//! Sistema completo de edición de mundos virtuales que se integra
//! con el motor 3D y sistema de física

use alloc::{vec::Vec, string::{String, ToString}, format, collections::BTreeMap};

/// Editor de niveles principal
pub struct LevelEditor {
    pub current_level: Level,
    pub scene_graph: SceneGraph,
    pub object_manager: ObjectManager,
    pub camera_controller: CameraController,
    pub selection_system: SelectionSystem,
    pub transform_tools: TransformTools,
    pub material_editor: MaterialEditor,
    pub lighting_editor: LightingEditor,
    pub physics_editor: PhysicsEditor,
    pub undo_redo: UndoRedoSystem,
    pub is_active: bool,
    pub viewport_mode: ViewportMode,
    pub grid_enabled: bool,
    pub snap_enabled: bool,
}

/// Nivel/escena
#[derive(Debug, Clone)]
pub struct Level {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub created_date: String,
    pub modified_date: String,
    pub settings: LevelSettings,
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
    pub cameras: Vec<Camera>,
    pub materials: Vec<Material>,
    pub physics_settings: PhysicsSettings,
}

/// Configuración del nivel
#[derive(Debug, Clone)]
pub struct LevelSettings {
    pub gravity: Vector3,
    pub ambient_light: Vector3,
    pub fog_enabled: bool,
    pub fog_color: Vector3,
    pub fog_density: f32,
    pub skybox_enabled: bool,
    pub skybox_texture: String,
    pub time_of_day: f32,
    pub weather: WeatherType,
}

/// Vector 3D
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Tipo de clima
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rainy,
    Snowy,
    Foggy,
    Stormy,
}

/// Grafo de escena
#[derive(Debug, Clone)]
pub struct SceneGraph {
    pub root_node: SceneNode,
    pub nodes: BTreeMap<String, SceneNode>,
    pub hierarchy: BTreeMap<String, Vec<String>>,
}

/// Nodo de escena
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
    pub transform: Transform,
    pub object_id: Option<String>,
    pub visible: bool,
    pub enabled: bool,
}

/// Transformación
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub matrix: Matrix4x4,
}

/// Cuaternión
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Matriz 4x4
#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

/// Objeto de escena
#[derive(Debug, Clone)]
pub struct SceneObject {
    pub id: String,
    pub name: String,
    pub object_type: ObjectType,
    pub mesh_id: String,
    pub material_id: String,
    pub physics_body_id: Option<String>,
    pub transform: Transform,
    pub properties: BTreeMap<String, String>,
    pub visible: bool,
    pub cast_shadows: bool,
    pub receive_shadows: bool,
}

/// Tipo de objeto
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectType {
    Static,
    Dynamic,
    Kinematic,
    Trigger,
    Decoration,
    Light,
    Camera,
    ParticleSystem,
    AudioSource,
    Custom,
}

/// Gestor de objetos
#[derive(Debug, Clone)]
pub struct ObjectManager {
    pub objects: BTreeMap<String, SceneObject>,
    pub meshes: BTreeMap<String, Mesh>,
    pub materials: BTreeMap<String, Material>,
    pub prefabs: BTreeMap<String, Prefab>,
    pub object_counter: u32,
}

/// Malla
#[derive(Debug, Clone)]
pub struct Mesh {
    pub id: String,
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub bounding_box: BoundingBox,
    pub vertex_count: u32,
    pub triangle_count: u32,
}

/// Vértice
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tangent: Vector3,
    pub texcoord: Vector2,
    pub color: Vector4,
}

/// Vector 2D
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// Vector 4D
#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Caja delimitadora
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
    pub center: Vector3,
    pub size: Vector3,
}

/// Material
#[derive(Debug, Clone)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub shader_id: String,
    pub diffuse_color: Vector4,
    pub specular_color: Vector4,
    pub emissive_color: Vector4,
    pub metallic: f32,
    pub roughness: f32,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub textures: BTreeMap<String, String>,
    pub properties: BTreeMap<String, f32>,
}

/// Prefab
#[derive(Debug, Clone)]
pub struct Prefab {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objects: Vec<SceneObject>,
    pub materials: Vec<Material>,
    pub created_date: String,
    pub category: String,
}

/// Controlador de cámara
#[derive(Debug, Clone)]
pub struct CameraController {
    pub active_camera: String,
    pub cameras: BTreeMap<String, Camera>,
    pub viewport: Viewport,
    pub projection_mode: ProjectionMode,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
}

/// Cámara
#[derive(Debug, Clone)]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub transform: Transform,
    pub projection_mode: ProjectionMode,
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect_ratio: f32,
    pub ortho_size: f32,
    pub view_matrix: Matrix4x4,
    pub projection_matrix: Matrix4x4,
}

/// Modo de proyección
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
}

/// Viewport
#[derive(Debug, Clone)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub clear_color: Vector4,
    pub wireframe: bool,
    pub show_grid: bool,
    pub show_axes: bool,
    pub show_bounds: bool,
}

/// Sistema de selección
#[derive(Debug, Clone)]
pub struct SelectionSystem {
    pub selected_objects: Vec<String>,
    pub selection_mode: SelectionMode,
    pub selection_tool: SelectionTool,
    pub multi_select: bool,
    pub selection_box: BoundingBox,
}

/// Modo de selección
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionMode {
    Object,
    Vertex,
    Edge,
    Face,
    Material,
}

/// Herramienta de selección
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionTool {
    Select,
    BoxSelect,
    LassoSelect,
    PaintSelect,
}

/// Herramientas de transformación
#[derive(Debug, Clone)]
pub struct TransformTools {
    pub active_tool: TransformTool,
    pub gizmo_mode: GizmoMode,
    pub gizmo_space: GizmoSpace,
    pub snap_settings: SnapSettings,
    pub pivot_mode: PivotMode,
}

/// Herramienta de transformación
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransformTool {
    Move,
    Rotate,
    Scale,
    Universal,
}

/// Modo de gizmo
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GizmoMode {
    Local,
    Global,
    View,
    Normal,
}

/// Espacio del gizmo
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GizmoSpace {
    World,
    Local,
    View,
}

/// Configuración de snap
#[derive(Debug, Clone)]
pub struct SnapSettings {
    pub enabled: bool,
    pub snap_to_grid: bool,
    pub snap_to_vertex: bool,
    pub snap_to_edge: bool,
    pub snap_to_face: bool,
    pub grid_size: f32,
    pub angle_snap: f32,
    pub scale_snap: f32,
}

/// Modo de pivote
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PivotMode {
    Center,
    Individual,
    Active,
    Cursor,
}

/// Editor de materiales
#[derive(Debug, Clone)]
pub struct MaterialEditor {
    pub active_material: Option<String>,
    pub material_preview: bool,
    pub shader_nodes: Vec<ShaderNode>,
    pub node_connections: Vec<NodeConnection>,
}

/// Nodo de shader
#[derive(Debug, Clone)]
pub struct ShaderNode {
    pub id: String,
    pub node_type: ShaderNodeType,
    pub position: Vector2,
    pub inputs: Vec<NodeInput>,
    pub outputs: Vec<NodeOutput>,
    pub properties: BTreeMap<String, f32>,
}

/// Tipo de nodo de shader
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderNodeType {
    Input,
    Output,
    Math,
    Vector,
    Color,
    Texture,
    Geometry,
    Light,
    Custom,
}

/// Entrada de nodo
#[derive(Debug, Clone)]
pub struct NodeInput {
    pub name: String,
    pub data_type: NodeDataType,
    pub value: NodeValue,
    pub connected: bool,
}

/// Salida de nodo
#[derive(Debug, Clone)]
pub struct NodeOutput {
    pub name: String,
    pub data_type: NodeDataType,
    pub value: NodeValue,
}

/// Tipo de dato de nodo
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeDataType {
    Float,
    Vector2,
    Vector3,
    Vector4,
    Color,
    Texture,
    Geometry,
}

/// Valor de nodo
#[derive(Debug, Clone)]
pub enum NodeValue {
    Float(f32),
    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),
    Color(Vector4),
    Texture(String),
    Geometry(String),
}

/// Conexión de nodos
#[derive(Debug, Clone)]
pub struct NodeConnection {
    pub from_node: String,
    pub from_output: String,
    pub to_node: String,
    pub to_input: String,
}

/// Editor de iluminación
#[derive(Debug, Clone)]
pub struct LightingEditor {
    pub lights: BTreeMap<String, Light>,
    pub light_baking: bool,
    pub global_illumination: bool,
    pub ambient_occlusion: bool,
    pub shadow_quality: ShadowQuality,
}

/// Luz
#[derive(Debug, Clone)]
pub struct Light {
    pub id: String,
    pub name: String,
    pub light_type: LightType,
    pub transform: Transform,
    pub color: Vector3,
    pub intensity: f32,
    pub range: f32,
    pub spot_angle: f32,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
    pub casts_shadows: bool,
    pub shadow_resolution: u32,
    pub shadow_bias: f32,
    pub shadow_normal_bias: f32,
}

/// Tipo de luz
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LightType {
    Directional,
    Point,
    Spot,
    Area,
}

/// Calidad de sombras
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Editor de física
#[derive(Debug, Clone)]
pub struct PhysicsEditor {
    pub physics_bodies: BTreeMap<String, PhysicsBody>,
    pub collision_shapes: BTreeMap<String, CollisionShape>,
    pub materials: BTreeMap<String, PhysicsMaterial>,
    pub constraints: BTreeMap<String, Constraint>,
    pub debug_draw: bool,
}

/// Cuerpo de física
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub id: String,
    pub name: String,
    pub body_type: BodyType,
    pub mass: f32,
    pub inertia: Vector3,
    pub center_of_mass: Vector3,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub restitution: f32,
    pub friction: f32,
    pub rolling_friction: f32,
    pub spinning_friction: f32,
}

/// Tipo de cuerpo
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

/// Forma de colisión
#[derive(Debug, Clone)]
pub struct CollisionShape {
    pub id: String,
    pub name: String,
    pub shape_type: ShapeType,
    pub dimensions: Vector3,
    pub radius: f32,
    pub height: f32,
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
}

/// Material de física
#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    pub id: String,
    pub name: String,
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub rolling_friction: f32,
    pub spinning_friction: f32,
}

/// Restricción
#[derive(Debug, Clone)]
pub struct Constraint {
    pub id: String,
    pub name: String,
    pub constraint_type: ConstraintType,
    pub body_a: String,
    pub body_b: String,
    pub pivot_a: Vector3,
    pub pivot_b: Vector3,
    pub limits: ConstraintLimits,
}

/// Tipo de restricción
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintType {
    PointToPoint,
    Hinge,
    Slider,
    ConeTwist,
    Generic6DOF,
    Fixed,
}

/// Límites de restricción
#[derive(Debug, Clone)]
pub struct ConstraintLimits {
    pub linear_lower: Vector3,
    pub linear_upper: Vector3,
    pub angular_lower: Vector3,
    pub angular_upper: Vector3,
    pub motor_enabled: bool,
    pub motor_target_velocity: Vector3,
    pub motor_max_force: Vector3,
}

/// Sistema de deshacer/rehacer
#[derive(Debug, Clone)]
pub struct UndoRedoSystem {
    pub undo_stack: Vec<EditorAction>,
    pub redo_stack: Vec<EditorAction>,
    pub max_history: usize,
    pub current_action: Option<EditorAction>,
}

/// Acción del editor
#[derive(Debug, Clone)]
pub struct EditorAction {
    pub id: String,
    pub action_type: ActionType,
    pub description: String,
    pub timestamp: String,
    pub data: ActionData,
}

/// Tipo de acción
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionType {
    CreateObject,
    DeleteObject,
    MoveObject,
    RotateObject,
    ScaleObject,
    ChangeMaterial,
    ChangeProperties,
    CreateLight,
    DeleteLight,
    ChangeLightSettings,
    CreateCamera,
    DeleteCamera,
    ChangeCameraSettings,
    CreateMaterial,
    DeleteMaterial,
    ChangeMaterialProperties,
    CreatePhysicsBody,
    DeletePhysicsBody,
    ChangePhysicsProperties,
}

/// Datos de acción
#[derive(Debug, Clone)]
pub enum ActionData {
    ObjectData(SceneObject),
    LightData(Light),
    CameraData(Camera),
    MaterialData(Material),
    PhysicsData(PhysicsBody),
    TransformData(Transform),
    PropertiesData(BTreeMap<String, String>),
}

/// Modo de viewport
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViewportMode {
    Perspective,
    Orthographic,
    Wireframe,
    Shaded,
    Material,
    Lighting,
    Physics,
    Navigation,
}

/// Configuración de física
#[derive(Debug, Clone)]
pub struct PhysicsSettings {
    pub gravity: Vector3,
    pub air_density: f32,
    pub wind_velocity: Vector3,
    pub time_scale: f32,
    pub solver_iterations: u32,
    pub solver_mode: SolverMode,
}

/// Modo del solucionador
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SolverMode {
    SequentialImpulse,
    ProjectedGaussSeidel,
    DirectSolver,
    MultiBody,
}

impl LevelEditor {
    pub fn new() -> Self {
        Self {
            current_level: Level::new(),
            scene_graph: SceneGraph::new(),
            object_manager: ObjectManager::new(),
            camera_controller: CameraController::new(),
            selection_system: SelectionSystem::new(),
            transform_tools: TransformTools::new(),
            material_editor: MaterialEditor::new(),
            lighting_editor: LightingEditor::new(),
            physics_editor: PhysicsEditor::new(),
            undo_redo: UndoRedoSystem::new(),
            is_active: false,
            viewport_mode: ViewportMode::Perspective,
            grid_enabled: true,
            snap_enabled: true,
        }
    }

    /// Inicializar el editor de niveles
    pub fn initialize(&mut self) -> bool {
        // Crear nivel por defecto
        self.current_level = Level {
            id: "default_level".to_string(),
            name: "Nivel por Defecto".to_string(),
            description: "Nivel creado automáticamente".to_string(),
            version: "1.0".to_string(),
            author: "Sistema".to_string(),
            created_date: "2024-01-01".to_string(),
            modified_date: "2024-01-01".to_string(),
            settings: LevelSettings::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            materials: Vec::new(),
            physics_settings: PhysicsSettings::new(),
        };

        // Crear cámara por defecto
        let default_camera = Camera {
            id: "default_camera".to_string(),
            name: "Cámara Principal".to_string(),
            transform: Transform::new(),
            projection_mode: ProjectionMode::Perspective,
            fov: 60.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            aspect_ratio: 16.0 / 9.0,
            ortho_size: 10.0,
            view_matrix: Matrix4x4::identity(),
            projection_matrix: Matrix4x4::identity(),
        };

        self.camera_controller.cameras.insert("default_camera".to_string(), default_camera);
        self.camera_controller.active_camera = "default_camera".to_string();

        // Crear luz direccional por defecto
        let default_light = Light {
            id: "default_light".to_string(),
            name: "Luz del Sol".to_string(),
            light_type: LightType::Directional,
            transform: Transform::new(),
            color: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            intensity: 1.0,
            range: 0.0,
            spot_angle: 0.0,
            inner_cone_angle: 0.0,
            outer_cone_angle: 0.0,
            casts_shadows: true,
            shadow_resolution: 2048,
            shadow_bias: 0.001,
            shadow_normal_bias: 0.1,
        };

        self.lighting_editor.lights.insert("default_light".to_string(), default_light);

        // Crear material por defecto
        let default_material = Material {
            id: "default_material".to_string(),
            name: "Material por Defecto".to_string(),
            shader_id: "pbr".to_string(),
            diffuse_color: Vector4 { x: 0.8, y: 0.8, z: 0.8, w: 1.0 },
            specular_color: Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
            emissive_color: Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            metallic: 0.0,
            roughness: 0.5,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            textures: BTreeMap::new(),
            properties: BTreeMap::new(),
        };

        self.object_manager.materials.insert("default_material".to_string(), default_material);

        self.is_active = true;
        true
    }

    /// Crear objeto en la escena
    pub fn create_object(&mut self, name: &str, object_type: ObjectType, mesh_id: &str) -> Option<String> {
        let object_id = format!("obj_{}", self.object_manager.object_counter);
        self.object_manager.object_counter += 1;

        let object = SceneObject {
            id: object_id.clone(),
            name: name.to_string(),
            object_type,
            mesh_id: mesh_id.to_string(),
            material_id: "default_material".to_string(),
            physics_body_id: None,
            transform: Transform::new(),
            properties: BTreeMap::new(),
            visible: true,
            cast_shadows: true,
            receive_shadows: true,
        };

        self.object_manager.objects.insert(object_id.clone(), object.clone());
        self.current_level.objects.push(object);
        
        Some(object_id)
    }

    /// Seleccionar objeto
    pub fn select_object(&mut self, object_id: &str) -> bool {
        if self.object_manager.objects.contains_key(object_id) {
            self.selection_system.selected_objects.clear();
            self.selection_system.selected_objects.push(object_id.to_string());
            true
        } else {
            false
        }
    }

    /// Mover objeto seleccionado
    pub fn move_selected_object(&mut self, delta: Vector3) -> bool {
        if let Some(object_id) = self.selection_system.selected_objects.first() {
            if let Some(object) = self.object_manager.objects.get_mut(object_id) {
                object.transform.position.x += delta.x;
                object.transform.position.y += delta.y;
                object.transform.position.z += delta.z;
                return true;
            }
        }
        false
    }

    /// Rotar objeto seleccionado
    pub fn rotate_selected_object(&mut self, delta: Quaternion) -> bool {
        if let Some(object_id) = self.selection_system.selected_objects.first() {
            if let Some(object) = self.object_manager.objects.get_mut(object_id) {
                // Multiplicar cuaterniones (simplificado)
                object.transform.rotation = delta;
                return true;
            }
        }
        false
    }

    /// Escalar objeto seleccionado
    pub fn scale_selected_object(&mut self, delta: Vector3) -> bool {
        if let Some(object_id) = self.selection_system.selected_objects.first() {
            if let Some(object) = self.object_manager.objects.get_mut(object_id) {
                object.transform.scale.x *= delta.x;
                object.transform.scale.y *= delta.y;
                object.transform.scale.z *= delta.z;
                return true;
            }
        }
        false
    }

    /// Crear luz
    pub fn create_light(&mut self, name: &str, light_type: LightType) -> Option<String> {
        let light_id = format!("light_{}", self.lighting_editor.lights.len());
        
        let light = Light {
            id: light_id.clone(),
            name: name.to_string(),
            light_type,
            transform: Transform::new(),
            color: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            intensity: 1.0,
            range: 10.0,
            spot_angle: 45.0,
            inner_cone_angle: 30.0,
            outer_cone_angle: 45.0,
            casts_shadows: true,
            shadow_resolution: 1024,
            shadow_bias: 0.001,
            shadow_normal_bias: 0.1,
        };

        self.lighting_editor.lights.insert(light_id.clone(), light.clone());
        self.current_level.lights.push(light);
        
        Some(light_id)
    }

    /// Crear cámara
    pub fn create_camera(&mut self, name: &str) -> Option<String> {
        let camera_id = format!("camera_{}", self.camera_controller.cameras.len());
        
        let camera = Camera {
            id: camera_id.clone(),
            name: name.to_string(),
            transform: Transform::new(),
            projection_mode: ProjectionMode::Perspective,
            fov: 60.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            aspect_ratio: 16.0 / 9.0,
            ortho_size: 10.0,
            view_matrix: Matrix4x4::identity(),
            projection_matrix: Matrix4x4::identity(),
        };

        self.camera_controller.cameras.insert(camera_id.clone(), camera.clone());
        self.current_level.cameras.push(camera);
        
        Some(camera_id)
    }

    /// Guardar nivel
    pub fn save_level(&mut self, filename: &str) -> bool {
        // Simular guardado
        self.current_level.modified_date = "2024-01-01".to_string();
        true
    }

    /// Cargar nivel
    pub fn load_level(&mut self, filename: &str) -> bool {
        // Simular carga
        self.current_level.name = filename.to_string();
        true
    }

    /// Obtener información del editor
    pub fn get_editor_info(&self) -> String {
        format!(
            "Editor: {} | Objetos: {} | Luces: {} | Cámaras: {} | Materiales: {}",
            if self.is_active { "Activo" } else { "Inactivo" },
            self.object_manager.objects.len(),
            self.lighting_editor.lights.len(),
            self.camera_controller.cameras.len(),
            self.object_manager.materials.len()
        )
    }

    /// Obtener estadísticas detalladas
    pub fn get_detailed_stats(&self) -> String {
        format!(
            "Estadísticas del Editor de Niveles:\n\
            ===================================\n\
            Estado: {}\n\
            Nivel: {}\n\
            Objetos: {}\n\
            Luces: {}\n\
            Cámaras: {}\n\
            Materiales: {}\n\
            Prefabs: {}\n\
            Objetos Seleccionados: {}\n\
            Modo de Viewport: {:?}\n\
            Grid: {}\n\
            Snap: {}\n\
            Herramienta Activa: {:?}\n\
            Modo de Gizmo: {:?}\n\
            Acciones Deshacer: {}\n\
            Acciones Rehacer: {}\n\
            \n\
            Configuración del Nivel:\n\
            - Gravedad: ({:.2}, {:.2}, {:.2})\n\
            - Luz Ambiental: ({:.2}, {:.2}, {:.2})\n\
            - Niebla: {}\n\
            - Skybox: {}\n\
            - Hora del Día: {:.1}\n\
            - Clima: {:?}",
            if self.is_active { "Activo" } else { "Inactivo" },
            self.current_level.name,
            self.object_manager.objects.len(),
            self.lighting_editor.lights.len(),
            self.camera_controller.cameras.len(),
            self.object_manager.materials.len(),
            self.object_manager.prefabs.len(),
            self.selection_system.selected_objects.len(),
            self.viewport_mode,
            if self.grid_enabled { "Habilitado" } else { "Deshabilitado" },
            if self.snap_enabled { "Habilitado" } else { "Deshabilitado" },
            self.transform_tools.active_tool,
            self.transform_tools.gizmo_mode,
            self.undo_redo.undo_stack.len(),
            self.undo_redo.redo_stack.len(),
            self.current_level.settings.gravity.x,
            self.current_level.settings.gravity.y,
            self.current_level.settings.gravity.z,
            self.current_level.settings.ambient_light.x,
            self.current_level.settings.ambient_light.y,
            self.current_level.settings.ambient_light.z,
            if self.current_level.settings.fog_enabled { "Habilitada" } else { "Deshabilitada" },
            if self.current_level.settings.skybox_enabled { "Habilitado" } else { "Deshabilitado" },
            self.current_level.settings.time_of_day,
            self.current_level.settings.weather
        )
    }
}

// Implementaciones por defecto
impl Level {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            version: String::new(),
            author: String::new(),
            created_date: String::new(),
            modified_date: String::new(),
            settings: LevelSettings::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            cameras: Vec::new(),
            materials: Vec::new(),
            physics_settings: PhysicsSettings::new(),
        }
    }
}

impl LevelSettings {
    pub fn new() -> Self {
        Self {
            gravity: Vector3 { x: 0.0, y: -9.81, z: 0.0 },
            ambient_light: Vector3 { x: 0.2, y: 0.2, z: 0.2 },
            fog_enabled: false,
            fog_color: Vector3 { x: 0.5, y: 0.5, z: 0.5 },
            fog_density: 0.01,
            skybox_enabled: true,
            skybox_texture: "default_skybox".to_string(),
            time_of_day: 12.0,
            weather: WeatherType::Clear,
        }
    }
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            root_node: SceneNode::new("root".to_string()),
            nodes: BTreeMap::new(),
            hierarchy: BTreeMap::new(),
        }
    }
}

impl SceneNode {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
            parent_id: None,
            children_ids: Vec::new(),
            transform: Transform::new(),
            object_id: None,
            visible: true,
            enabled: true,
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            scale: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            matrix: Matrix4x4::identity(),
        }
    }
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            meshes: BTreeMap::new(),
            materials: BTreeMap::new(),
            prefabs: BTreeMap::new(),
            object_counter: 0,
        }
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            active_camera: String::new(),
            cameras: BTreeMap::new(),
            viewport: Viewport::new(),
            projection_mode: ProjectionMode::Perspective,
            movement_speed: 5.0,
            rotation_speed: 2.0,
            zoom_speed: 1.0,
        }
    }
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            clear_color: Vector4 { x: 0.2, y: 0.2, z: 0.2, w: 1.0 },
            wireframe: false,
            show_grid: true,
            show_axes: true,
            show_bounds: false,
        }
    }
}

impl SelectionSystem {
    pub fn new() -> Self {
        Self {
            selected_objects: Vec::new(),
            selection_mode: SelectionMode::Object,
            selection_tool: SelectionTool::Select,
            multi_select: false,
            selection_box: BoundingBox {
                min: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                max: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                center: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                size: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
        }
    }
}

impl TransformTools {
    pub fn new() -> Self {
        Self {
            active_tool: TransformTool::Move,
            gizmo_mode: GizmoMode::Local,
            gizmo_space: GizmoSpace::World,
            snap_settings: SnapSettings::new(),
            pivot_mode: PivotMode::Center,
        }
    }
}

impl SnapSettings {
    pub fn new() -> Self {
        Self {
            enabled: true,
            snap_to_grid: true,
            snap_to_vertex: false,
            snap_to_edge: false,
            snap_to_face: false,
            grid_size: 1.0,
            angle_snap: 15.0,
            scale_snap: 0.1,
        }
    }
}

impl MaterialEditor {
    pub fn new() -> Self {
        Self {
            active_material: None,
            material_preview: true,
            shader_nodes: Vec::new(),
            node_connections: Vec::new(),
        }
    }
}

impl LightingEditor {
    pub fn new() -> Self {
        Self {
            lights: BTreeMap::new(),
            light_baking: false,
            global_illumination: true,
            ambient_occlusion: true,
            shadow_quality: ShadowQuality::High,
        }
    }
}

impl PhysicsEditor {
    pub fn new() -> Self {
        Self {
            physics_bodies: BTreeMap::new(),
            collision_shapes: BTreeMap::new(),
            materials: BTreeMap::new(),
            constraints: BTreeMap::new(),
            debug_draw: false,
        }
    }
}

impl UndoRedoSystem {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 100,
            current_action: None,
        }
    }
}

impl PhysicsSettings {
    pub fn new() -> Self {
        Self {
            gravity: Vector3 { x: 0.0, y: -9.81, z: 0.0 },
            air_density: 1.225,
            wind_velocity: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            time_scale: 1.0,
            solver_iterations: 10,
            solver_mode: SolverMode::SequentialImpulse,
        }
    }
}

// Gestor global del editor de niveles
use spin::Mutex;

pub static LEVEL_EDITOR: Mutex<Option<LevelEditor>> = Mutex::new(None);

/// Inicializar el editor de niveles
pub fn init_level_editor() {
    let mut editor = LEVEL_EDITOR.lock();
    *editor = Some(LevelEditor::new());
    if let Some(ref mut e) = *editor {
        e.initialize();
    }
    crate::logging::info("level_editor", "Editor de niveles inicializado");
}

/// Obtener información del editor
pub fn get_editor_info() -> String {
    let editor = LEVEL_EDITOR.lock();
    if let Some(ref e) = *editor {
        e.get_editor_info()
    } else {
        String::from("Editor de niveles no inicializado")
    }
}

/// Mover objeto seleccionado
pub fn move_selected_object(delta: Vector3) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut ed) = *editor {
        if let Some(selected_id) = ed.selection_system.selected_objects.first() {
            if let Some(obj) = ed.object_manager.objects.get_mut(selected_id.as_str()) {
                obj.transform.position.x += delta.x;
                obj.transform.position.y += delta.y;
                obj.transform.position.z += delta.z;
                return true;
            }
        }
    }
    false
}

/// Rotar objeto seleccionado
pub fn rotate_selected_object(rotation: Quaternion) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut ed) = *editor {
        if let Some(selected_id) = ed.selection_system.selected_objects.first() {
            if let Some(obj) = ed.object_manager.objects.get_mut(selected_id.as_str()) {
                obj.transform.rotation = rotation;
                return true;
            }
        }
    }
    false
}

/// Escalar objeto seleccionado
pub fn scale_selected_object(scale: Vector3) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut ed) = *editor {
        if let Some(selected_id) = ed.selection_system.selected_objects.first() {
            if let Some(obj) = ed.object_manager.objects.get_mut(selected_id.as_str()) {
                obj.transform.scale = scale;
                return true;
            }
        }
    }
    false
}

/// Obtener estadísticas detalladas
pub fn get_editor_detailed_stats() -> String {
    let editor = LEVEL_EDITOR.lock();
    if let Some(ref e) = *editor {
        e.get_detailed_stats()
    } else {
        String::from("Editor de niveles no inicializado")
    }
}

/// Crear objeto
pub fn create_object(name: &str, object_type: ObjectType, mesh_id: &str) -> Option<String> {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.create_object(name, object_type, mesh_id)
    } else {
        None
    }
}

/// Seleccionar objeto
pub fn select_object(object_id: &str) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.select_object(object_id)
    } else {
        false
    }
}

/// Crear luz
pub fn create_light(name: &str, light_type: LightType) -> Option<String> {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.create_light(name, light_type)
    } else {
        None
    }
}

/// Crear cámara
pub fn create_camera(name: &str) -> Option<String> {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.create_camera(name)
    } else {
        None
    }
}

/// Guardar nivel
pub fn save_level(filename: &str) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.save_level(filename)
    } else {
        false
    }
}

/// Cargar nivel
pub fn load_level(filename: &str) -> bool {
    let mut editor = LEVEL_EDITOR.lock();
    if let Some(ref mut e) = *editor {
        e.load_level(filename)
    } else {
        false
    }
}
