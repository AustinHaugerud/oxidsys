use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetFogDistanceOp;

const DOC: &str = "Sets the density (and optionally color) of the fog for the mission.";

pub const OP_CODE: u32 = 1798;

pub const IDENT: &str = "set_fog_distance";

impl Operation for SetFogDistanceOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<distance_in_meters>", ""),
                make_param_doc("[fog_color]", ""),
            ],
        }
    }
}
