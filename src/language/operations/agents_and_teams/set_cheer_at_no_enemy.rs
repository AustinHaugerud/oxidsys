use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetCheerAtNoEnemyOp;

const DOC : &str = "Version 1.153+. Determines whether the agents will cheer when no enemy remain on the map. 0 = do not cheer, 1 = cheer.";

pub const OP_CODE: u32 = 2379;

pub const IDENT: &str = "set_cheer_at_no_enemy";

impl Operation for SetCheerAtNoEnemyOp {
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
