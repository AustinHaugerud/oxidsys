use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnAgentOp;

const DOC : &str = "Spawns a new troop in the specified position and saves the reference to the new agent in reg0.";

pub const OP_CODE: u32 = 1972;

pub const IDENT: &str = "spawn_agent";

impl Operation for SpawnAgentOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
