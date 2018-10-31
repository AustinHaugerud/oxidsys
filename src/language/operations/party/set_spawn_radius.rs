use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetSpawnRadiusOp;

const DOC: &str = "Sets radius for party spawning with subsequent <spawn_around_party> operations.";

pub const OP_CODE: u32 = 1103;

pub const IDENT: &str = "set_spawn_radius";

impl Operation for SetSpawnRadiusOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
