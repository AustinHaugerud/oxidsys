use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod copy_position;
pub mod get_angle_between_positions;
pub mod get_distance_between_positions;
pub mod get_distance_between_positions_in_meters;
pub mod get_sq_distance_between_position_heights;
pub mod init_position;
pub mod map_get_land_position_around_position;
pub mod map_get_random_position_around_position;
pub mod map_get_water_position_around_position;
pub mod position_copy_origin;
pub mod position_copy_rotation;
pub mod position_get_distance_to_ground_level;
pub mod position_get_distance_to_terrain;
pub mod position_get_rotation_around_x;
pub mod position_get_rotation_around_y;
pub mod position_get_rotation_around_z;
pub mod position_get_scale_x;
pub mod position_get_scale_y;
pub mod position_get_scale_z;
pub mod position_get_screen_projection;
pub mod position_get_x;
pub mod position_get_y;
pub mod position_get_z;
pub mod position_has_line_of_sight_to_position;
pub mod position_is_behind_position;
pub mod position_move_x;
pub mod position_move_y;
pub mod position_move_z;
pub mod position_normalize_origin;
pub mod position_rotate_x;
pub mod position_rotate_x_floating;
pub mod position_rotate_y;
pub mod position_rotate_y_floating;
pub mod position_rotate_z;
pub mod position_rotate_z_floating;
pub mod position_set_scale_x;
pub mod position_set_scale_y;
pub mod position_set_scale_z;
pub mod position_set_x;
pub mod position_set_y;
pub mod position_set_z;
pub mod position_set_z_to_ground_level;
pub mod position_transform_position_to_local;
pub mod position_transform_position_to_parent;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(copy_position::CopyPositionOp {}));
    result.push(Box::new(
        get_angle_between_positions::GetAngleBetweenPositionsOp {},
    ));
    result.push(Box::new(
        get_distance_between_positions::GetDistanceBetweenPositionsOp {},
    ));
    result.push(Box::new(
        get_distance_between_positions_in_meters::GetDistanceBetweenPositionsInMetersOp {},
    ));
    result.push(Box::new(
        get_sq_distance_between_position_heights::GetSqDistanceBetweenPositionHeightsOp {},
    ));
    result.push(Box::new(init_position::InitPositionOp {}));
    result.push(Box::new(
        map_get_land_position_around_position::MapGetLandPositionAroundPositionOp {},
    ));
    result.push(Box::new(
        map_get_random_position_around_position::MapGetRandomPositionAroundPositionOp {},
    ));
    result.push(Box::new(
        map_get_water_position_around_position::MapGetWaterPositionAroundPositionOp {},
    ));
    result.push(Box::new(position_copy_origin::PositionCopyOriginOp {}));
    result.push(Box::new(position_copy_rotation::PositionCopyRotationOp {}));
    result.push(Box::new(
        position_get_distance_to_ground_level::PositionGetDistanceToGroundLevelOp {},
    ));
    result.push(Box::new(
        position_get_distance_to_terrain::PositionGetDistanceToTerrainOp {},
    ));
    result.push(Box::new(
        position_get_rotation_around_x::PositionGetRotationAroundXOp {},
    ));
    result.push(Box::new(
        position_get_rotation_around_y::PositionGetRotationAroundYOp {},
    ));
    result.push(Box::new(
        position_get_rotation_around_z::PositionGetRotationAroundZOp {},
    ));
    result.push(Box::new(position_get_scale_x::PositionGetScaleXOp {}));
    result.push(Box::new(position_get_scale_y::PositionGetScaleYOp {}));
    result.push(Box::new(position_get_scale_z::PositionGetScaleZOp {}));
    result.push(Box::new(
        position_get_screen_projection::PositionGetScreenProjectionOp {},
    ));
    result.push(Box::new(position_get_x::PositionGetXOp {}));
    result.push(Box::new(position_get_y::PositionGetYOp {}));
    result.push(Box::new(position_get_z::PositionGetZOp {}));
    result.push(Box::new(
        position_has_line_of_sight_to_position::PositionHasLineOfSightToPositionOp {},
    ));
    result.push(Box::new(
        position_is_behind_position::PositionIsBehindPositionOp {},
    ));
    result.push(Box::new(position_move_x::PositionMoveXOp {}));
    result.push(Box::new(position_move_y::PositionMoveYOp {}));
    result.push(Box::new(position_move_z::PositionMoveZOp {}));
    result.push(Box::new(
        position_normalize_origin::PositionNormalizeOriginOp {},
    ));
    result.push(Box::new(position_rotate_x::PositionRotateXOp {}));
    result.push(Box::new(
        position_rotate_x_floating::PositionRotateXFloatingOp {},
    ));
    result.push(Box::new(position_rotate_y::PositionRotateYOp {}));
    result.push(Box::new(
        position_rotate_y_floating::PositionRotateYFloatingOp {},
    ));
    result.push(Box::new(position_rotate_z::PositionRotateZOp {}));
    result.push(Box::new(
        position_rotate_z_floating::PositionRotateZFloatingOp {},
    ));
    result.push(Box::new(position_set_scale_x::PositionSetScaleXOp {}));
    result.push(Box::new(position_set_scale_y::PositionSetScaleYOp {}));
    result.push(Box::new(position_set_scale_z::PositionSetScaleZOp {}));
    result.push(Box::new(position_set_x::PositionSetXOp {}));
    result.push(Box::new(position_set_y::PositionSetYOp {}));
    result.push(Box::new(position_set_z::PositionSetZOp {}));
    result.push(Box::new(
        position_set_z_to_ground_level::PositionSetZToGroundLevelOp {},
    ));
    result.push(Box::new(
        position_transform_position_to_local::PositionTransformPositionToLocalOp {},
    ));
    result.push(Box::new(
        position_transform_position_to_parent::PositionTransformPositionToParentOp {},
    ));
    result
}
