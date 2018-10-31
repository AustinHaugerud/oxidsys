use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauSetOverrideFlagsOp;

const DOC : &str = "When creating a troop image for current tableau, this operation allows to override troop's inventory partially or completely. See af_* flags in header_mission_templates.py for reference.";

pub const OP_CODE: u32 = 1997;

pub const IDENT: &str = "cur_tableau_set_override_flags";

impl Operation for CurTableauSetOverrideFlagsOp {
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
