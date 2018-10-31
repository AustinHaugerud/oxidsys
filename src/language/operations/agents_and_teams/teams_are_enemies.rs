use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TeamsAreEnemiesOp;

const DOC: &str = "Checks that the two teams are hostile to each other.";

pub const OP_CODE: u32 = 1788;

pub const IDENT: &str = "teams_are_enemies";

impl Operation for TeamsAreEnemiesOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<team_no>", ""),
                make_param_doc("<team_no_2>", ""),
            ],
        }
    }
}
