use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerIsBusyWithMenusOp;

const DOC : &str = "Undocumented. Educated guess is it's true when player is running a presentation without prsntf_read_only flag.";

pub const OP_CODE: u32 = 438;

pub const IDENT: &str = "player_is_busy_with_menus";

impl Operation for PlayerIsBusyWithMenusOp {
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
            param_docs: vec![make_param_doc("<player_id>", "")],
        }
    }
}
