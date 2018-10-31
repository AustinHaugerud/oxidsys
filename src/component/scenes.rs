pub struct Scene {
    id: String,
    flags: Vec<String>,
    mesh_name: Option<String>,
    body_name: Option<String>,
    min_pos: (f32, f32),
    max_pos: (f32, f32),
    water_level: f32,
    terrain_code: String,
    chest_troops: Vec<String>,
    outer_terrain: Option<String>,
}
