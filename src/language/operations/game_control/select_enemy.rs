use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SelectEnemyOp;

const DOC : &str = "When joining a battle, this determines what side player will be helping. Defending party is always 0, and attacking party is always 1. Player can support either attackers (value = 0, i.e. defenders are the enemy) or defenders (value = 1).";

pub const OP_CODE: u32 = 1303;

pub const IDENT: &str = "select_enemy";

impl Operation for SelectEnemyOp {
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
