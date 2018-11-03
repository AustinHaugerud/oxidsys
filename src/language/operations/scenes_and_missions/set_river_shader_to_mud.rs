use language::operations::{Operation, ParamInfo};

pub struct SetRiverShaderToMudOp;

const DOC : &str = "Changes river material for muddy env. This operation is not documented nor any examples of it's use could be found. Parameters are unknown.";

pub const OP_CODE: u32 = 2387;

pub const IDENT: &str = "set_river_shader_to_mud";

impl Operation for SetRiverShaderToMudOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}
