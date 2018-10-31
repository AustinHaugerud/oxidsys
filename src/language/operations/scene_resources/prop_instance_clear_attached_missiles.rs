use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceClearAttachedMissilesOp;

const DOC : &str = "Version 1.153+. Removes all missiles currently attached to the scene prop. Only works with dynamic scene props.";

pub const OP_CODE: u32 = 1885;

pub const IDENT: &str = "prop_instance_clear_attached_missiles";

impl Operation for PropInstanceClearAttachedMissilesOp {
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
            param_docs: vec![make_param_doc("<scene_prop_id>", "")],
        }
    }
}
