use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionSetZToGroundLevelOp;

const DOC : &str = "This will bring the position Z coordinate so it rests on the ground level (i.e. an agent could stand on that position). This takes scene props with their collision meshes into account. Only works during a mission, so you can't measure global map height using this.";

pub const OP_CODE: u32 = 791;

pub const IDENT: &str = "position_set_z_to_ground_level";

impl Operation for PositionSetZToGroundLevelOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<position>", "")],
        }
    }
}
