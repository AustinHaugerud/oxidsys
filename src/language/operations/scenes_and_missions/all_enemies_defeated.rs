use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AllEnemiesDefeatedOp;

const DOC : &str = "Checks if all agents from the specified team are defeated. When team_id is omitted default enemy team is checked.";

pub const OP_CODE: u32 = 1003;

pub const IDENT: &str = "all_enemies_defeated";

impl Operation for AllEnemiesDefeatedOp {
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
            num_optional: 1,
            param_docs: vec![make_param_doc("[team_id]", "")],
        }
    }
}
