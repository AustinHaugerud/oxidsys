use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod cur_agent_set_banner_tableau_material;
pub mod cur_item_set_tableau_material;
pub mod cur_map_icon_set_tableau_material;
pub mod cur_scene_prop_set_tableau_material;
pub mod cur_tableau_add_horse;
pub mod cur_tableau_add_map_icon;
pub mod cur_tableau_add_mesh;
pub mod cur_tableau_add_mesh_with_scale_and_vertex_color;
pub mod cur_tableau_add_mesh_with_vertex_color;
pub mod cur_tableau_add_override_item;
pub mod cur_tableau_add_point_light;
pub mod cur_tableau_add_sun_light;
pub mod cur_tableau_add_tableau_mesh;
pub mod cur_tableau_add_troop;
pub mod cur_tableau_clear_override_items;
pub mod cur_tableau_render_as_alpha_mask;
pub mod cur_tableau_set_ambient_light;
pub mod cur_tableau_set_background_color;
pub mod cur_tableau_set_camera_parameters;
pub mod cur_tableau_set_camera_position;
pub mod cur_tableau_set_override_flags;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        cur_agent_set_banner_tableau_material::CurAgentSetBannerTableauMaterialOp {},
    ));
    result.push(Box::new(
        cur_item_set_tableau_material::CurItemSetTableauMaterialOp {},
    ));
    result.push(Box::new(
        cur_map_icon_set_tableau_material::CurMapIconSetTableauMaterialOp {},
    ));
    result.push(Box::new(
        cur_scene_prop_set_tableau_material::CurScenePropSetTableauMaterialOp {},
    ));
    result.push(Box::new(cur_tableau_add_horse::CurTableauAddHorseOp {}));
    result.push(Box::new(
        cur_tableau_add_map_icon::CurTableauAddMapIconOp {},
    ));
    result.push(Box::new(cur_tableau_add_mesh::CurTableauAddMeshOp {}));
    result.push(Box::new(cur_tableau_add_mesh_with_scale_and_vertex_color::CurTableauAddMeshWithScaleAndVertexColorOp {}));
    result.push(Box::new(
        cur_tableau_add_mesh_with_vertex_color::CurTableauAddMeshWithVertexColorOp {},
    ));
    result.push(Box::new(
        cur_tableau_add_override_item::CurTableauAddOverrideItemOp {},
    ));
    result.push(Box::new(
        cur_tableau_add_point_light::CurTableauAddPointLightOp {},
    ));
    result.push(Box::new(
        cur_tableau_add_sun_light::CurTableauAddSunLightOp {},
    ));
    result.push(Box::new(
        cur_tableau_add_tableau_mesh::CurTableauAddTableauMeshOp {},
    ));
    result.push(Box::new(cur_tableau_add_troop::CurTableauAddTroopOp {}));
    result.push(Box::new(
        cur_tableau_clear_override_items::CurTableauClearOverrideItemsOp {},
    ));
    result.push(Box::new(
        cur_tableau_render_as_alpha_mask::CurTableauRenderAsAlphaMaskOp {},
    ));
    result.push(Box::new(
        cur_tableau_set_ambient_light::CurTableauSetAmbientLightOp {},
    ));
    result.push(Box::new(
        cur_tableau_set_background_color::CurTableauSetBackgroundColorOp {},
    ));
    result.push(Box::new(
        cur_tableau_set_camera_parameters::CurTableauSetCameraParametersOp {},
    ));
    result.push(Box::new(
        cur_tableau_set_camera_position::CurTableauSetCameraPositionOp {},
    ));
    result.push(Box::new(
        cur_tableau_set_override_flags::CurTableauSetOverrideFlagsOp {},
    ));
    result
}
