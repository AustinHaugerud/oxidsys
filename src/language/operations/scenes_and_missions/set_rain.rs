use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetRainOp;

const DOC : &str = "Sets a new weather for the mission. Rain_type values: 0 = clear, 1 = rain, 2 = snow. Strength is in range 0..100.";

pub const OP_CODE: u32 = 1797;

pub const IDENT: &str = "set_rain";

impl Operation for SetRainOp {
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
            param_docs: vec![make_param_doc("<strength>", "")],
        }
    }
}
