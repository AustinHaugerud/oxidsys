use language::operations::Operation;
pub mod add_point_light;
pub mod add_point_light_to_entity;
pub mod particle_system_add_new;
pub mod particle_system_burst;
pub mod particle_system_burst_no_sync;
pub mod particle_system_emit;
pub mod prop_instance_add_particle_system;
pub mod prop_instance_animate_to_position;
pub mod prop_instance_clear_attached_missiles;
pub mod prop_instance_deform_in_cycle_loop;
pub mod prop_instance_deform_in_range;
pub mod prop_instance_deform_to_time;
pub mod prop_instance_dynamics_apply_impulse;
pub mod prop_instance_dynamics_set_omega;
pub mod prop_instance_dynamics_set_properties;
pub mod prop_instance_dynamics_set_velocity;
pub mod prop_instance_enable_physics;
pub mod prop_instance_get_animation_target_position;
pub mod prop_instance_get_current_deform_frame;
pub mod prop_instance_get_current_deform_progress;
pub mod prop_instance_get_position;
pub mod prop_instance_get_scale;
pub mod prop_instance_get_scene_prop_kind;
pub mod prop_instance_get_starting_position;
pub mod prop_instance_get_variation_id;
pub mod prop_instance_get_variation_id_2;
pub mod prop_instance_initialize_rotation_angles;
pub mod prop_instance_intersects_with_prop_instance;
pub mod prop_instance_is_animating;
pub mod prop_instance_is_valid;
pub mod prop_instance_play_sound;
pub mod prop_instance_receive_damage;
pub mod prop_instance_refill_hit_points;
pub mod prop_instance_rotate_to_position;
pub mod prop_instance_set_material;
pub mod prop_instance_set_position;
pub mod prop_instance_stop_all_particle_systems;
pub mod prop_instance_stop_animating;
pub mod prop_instance_stop_sound;
pub mod replace_prop_instance;
pub mod replace_scene_items_with_scene_props;
pub mod replace_scene_props;
pub mod scene_item_get_instance;
pub mod scene_item_get_num_instances;
pub mod scene_prop_enable_after_time;
pub mod scene_prop_fade_in;
pub mod scene_prop_fade_out;
pub mod scene_prop_get_hit_points;
pub mod scene_prop_get_instance;
pub mod scene_prop_get_max_hit_points;
pub mod scene_prop_get_num_instances;
pub mod scene_prop_get_team;
pub mod scene_prop_get_visibility;
pub mod scene_prop_has_agent_on_it;
pub mod scene_prop_set_cur_hit_points;
pub mod scene_prop_set_hit_points;
pub mod scene_prop_set_prune_time;
pub mod scene_prop_set_team;
pub mod scene_prop_set_visibility;
pub mod scene_spawned_item_get_instance;
pub mod set_current_color;
pub mod set_position_delta;
pub mod set_spawn_position;
pub mod spawn_item;
pub mod spawn_item_without_refill;
pub mod spawn_scene_prop;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(add_point_light::AddPointLightOp {}));
    result.push(Box::new(
        add_point_light_to_entity::AddPointLightToEntityOp {},
    ));
    result.push(Box::new(particle_system_add_new::ParticleSystemAddNewOp {}));
    result.push(Box::new(particle_system_burst::ParticleSystemBurstOp {}));
    result.push(Box::new(
        particle_system_burst_no_sync::ParticleSystemBurstNoSyncOp {},
    ));
    result.push(Box::new(particle_system_emit::ParticleSystemEmitOp {}));
    result.push(Box::new(
        prop_instance_add_particle_system::PropInstanceAddParticleSystemOp {},
    ));
    result.push(Box::new(
        prop_instance_animate_to_position::PropInstanceAnimateToPositionOp {},
    ));
    result.push(Box::new(
        prop_instance_clear_attached_missiles::PropInstanceClearAttachedMissilesOp {},
    ));
    result.push(Box::new(
        prop_instance_deform_in_cycle_loop::PropInstanceDeformInCycleLoopOp {},
    ));
    result.push(Box::new(
        prop_instance_deform_in_range::PropInstanceDeformInRangeOp {},
    ));
    result.push(Box::new(
        prop_instance_deform_to_time::PropInstanceDeformToTimeOp {},
    ));
    result.push(Box::new(
        prop_instance_dynamics_apply_impulse::PropInstanceDynamicsApplyImpulseOp {},
    ));
    result.push(Box::new(
        prop_instance_dynamics_set_omega::PropInstanceDynamicsSetOmegaOp {},
    ));
    result.push(Box::new(
        prop_instance_dynamics_set_properties::PropInstanceDynamicsSetPropertiesOp {},
    ));
    result.push(Box::new(
        prop_instance_dynamics_set_velocity::PropInstanceDynamicsSetVelocityOp {},
    ));
    result.push(Box::new(
        prop_instance_enable_physics::PropInstanceEnablePhysicsOp {},
    ));
    result.push(Box::new(
        prop_instance_get_animation_target_position::PropInstanceGetAnimationTargetPositionOp {},
    ));
    result.push(Box::new(
        prop_instance_get_current_deform_frame::PropInstanceGetCurrentDeformFrameOp {},
    ));
    result.push(Box::new(
        prop_instance_get_current_deform_progress::PropInstanceGetCurrentDeformProgressOp {},
    ));
    result.push(Box::new(
        prop_instance_get_position::PropInstanceGetPositionOp {},
    ));
    result.push(Box::new(prop_instance_get_scale::PropInstanceGetScaleOp {}));
    result.push(Box::new(
        prop_instance_get_scene_prop_kind::PropInstanceGetScenePropKindOp {},
    ));
    result.push(Box::new(
        prop_instance_get_starting_position::PropInstanceGetStartingPositionOp {},
    ));
    result.push(Box::new(
        prop_instance_get_variation_id::PropInstanceGetVariationIdOp {},
    ));
    result.push(Box::new(
        prop_instance_get_variation_id_2::PropInstanceGetVariationId2Op {},
    ));
    result.push(Box::new(
        prop_instance_initialize_rotation_angles::PropInstanceInitializeRotationAnglesOp {},
    ));
    result.push(Box::new(
        prop_instance_intersects_with_prop_instance::PropInstanceIntersectsWithPropInstanceOp {},
    ));
    result.push(Box::new(
        prop_instance_is_animating::PropInstanceIsAnimatingOp {},
    ));
    result.push(Box::new(prop_instance_is_valid::PropInstanceIsValidOp {}));
    result.push(Box::new(
        prop_instance_play_sound::PropInstancePlaySoundOp {},
    ));
    result.push(Box::new(
        prop_instance_receive_damage::PropInstanceReceiveDamageOp {},
    ));
    result.push(Box::new(
        prop_instance_refill_hit_points::PropInstanceRefillHitPointsOp {},
    ));
    result.push(Box::new(
        prop_instance_rotate_to_position::PropInstanceRotateToPositionOp {},
    ));
    result.push(Box::new(
        prop_instance_set_material::PropInstanceSetMaterialOp {},
    ));
    result.push(Box::new(
        prop_instance_set_position::PropInstanceSetPositionOp {},
    ));
    result.push(Box::new(
        prop_instance_stop_all_particle_systems::PropInstanceStopAllParticleSystemsOp {},
    ));
    result.push(Box::new(
        prop_instance_stop_animating::PropInstanceStopAnimatingOp {},
    ));
    result.push(Box::new(
        prop_instance_stop_sound::PropInstanceStopSoundOp {},
    ));
    result.push(Box::new(replace_prop_instance::ReplacePropInstanceOp {}));
    result.push(Box::new(
        replace_scene_items_with_scene_props::ReplaceSceneItemsWithScenePropsOp {},
    ));
    result.push(Box::new(replace_scene_props::ReplaceScenePropsOp {}));
    result.push(Box::new(scene_item_get_instance::SceneItemGetInstanceOp {}));
    result.push(Box::new(
        scene_item_get_num_instances::SceneItemGetNumInstancesOp {},
    ));
    result.push(Box::new(
        scene_prop_enable_after_time::ScenePropEnableAfterTimeOp {},
    ));
    result.push(Box::new(scene_prop_fade_in::ScenePropFadeInOp {}));
    result.push(Box::new(scene_prop_fade_out::ScenePropFadeOutOp {}));
    result.push(Box::new(
        scene_prop_get_hit_points::ScenePropGetHitPointsOp {},
    ));
    result.push(Box::new(scene_prop_get_instance::ScenePropGetInstanceOp {}));
    result.push(Box::new(
        scene_prop_get_max_hit_points::ScenePropGetMaxHitPointsOp {},
    ));
    result.push(Box::new(
        scene_prop_get_num_instances::ScenePropGetNumInstancesOp {},
    ));
    result.push(Box::new(scene_prop_get_team::ScenePropGetTeamOp {}));
    result.push(Box::new(
        scene_prop_get_visibility::ScenePropGetVisibilityOp {},
    ));
    result.push(Box::new(
        scene_prop_has_agent_on_it::ScenePropHasAgentOnItOp {},
    ));
    result.push(Box::new(
        scene_prop_set_cur_hit_points::ScenePropSetCurHitPointsOp {},
    ));
    result.push(Box::new(
        scene_prop_set_hit_points::ScenePropSetHitPointsOp {},
    ));
    result.push(Box::new(
        scene_prop_set_prune_time::ScenePropSetPruneTimeOp {},
    ));
    result.push(Box::new(scene_prop_set_team::ScenePropSetTeamOp {}));
    result.push(Box::new(
        scene_prop_set_visibility::ScenePropSetVisibilityOp {},
    ));
    result.push(Box::new(
        scene_spawned_item_get_instance::SceneSpawnedItemGetInstanceOp {},
    ));
    result.push(Box::new(set_current_color::SetCurrentColorOp {}));
    result.push(Box::new(set_position_delta::SetPositionDeltaOp {}));
    result.push(Box::new(set_spawn_position::SetSpawnPositionOp {}));
    result.push(Box::new(spawn_item::SpawnItemOp {}));
    result.push(Box::new(
        spawn_item_without_refill::SpawnItemWithoutRefillOp {},
    ));
    result.push(Box::new(spawn_scene_prop::SpawnScenePropOp {}));
    result
}
