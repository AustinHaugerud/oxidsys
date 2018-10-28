use language::operations::Operation;
pub mod ai_mesh_face_group_show_hide;
pub mod auto_set_meta_mission_at_end_commited;
pub mod set_tooltip_text;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        ai_mesh_face_group_show_hide::AiMeshFaceGroupShowHideOp {},
    ));
    result.push(Box::new(
        auto_set_meta_mission_at_end_commited::AutoSetMetaMissionAtEndCommitedOp {},
    ));
    result.push(Box::new(set_tooltip_text::SetTooltipTextOp {}));
    result
}
