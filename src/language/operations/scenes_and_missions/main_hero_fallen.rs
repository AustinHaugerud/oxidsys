use language::operations::{Operation, ParamInfo};

pub struct MainHeroFallenOp;

const DOC: &str = "Checks that the player has been knocked out.";

pub const OP_CODE: u32 = 1006;

pub const IDENT: &str = "main_hero_fallen";

impl Operation for MainHeroFallenOp {
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
