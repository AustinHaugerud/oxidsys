use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsGetBattleSizeOp;

const DOC : &str = "Version 1.161+. Retrieves current battle size slider value (in the range of 0..1000). Note that this is the slider value, not the battle size itself.";

pub const OP_CODE: u32 = 270;

pub const IDENT: &str = "options_get_battle_size";

impl Operation for OptionsGetBattleSizeOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
