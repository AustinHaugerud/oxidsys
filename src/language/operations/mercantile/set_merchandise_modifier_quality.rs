use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetMerchandiseModifierQualityOp;

const DOC : &str = "Affects the probability of items with quality modifiers appearing in merchandise. Value is percentage, standard value is 100.";

pub const OP_CODE: u32 = 1490;

pub const IDENT: &str = "set_merchandise_modifier_quality";

impl Operation for SetMerchandiseModifierQualityOp {
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
