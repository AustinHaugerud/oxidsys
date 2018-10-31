use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateComboLabelOverlayOp;

const DOC : &str = "Creates a combo label overlay. Looks like plain text label. Individual items can be added with (overlay_add_item) and currently selected item can be set with (overlay_set_val). Returns combo block's overlay_id.";

pub const OP_CODE: u32 = 948;

pub const IDENT: &str = "create_combo_label_overlay";

impl Operation for CreateComboLabelOverlayOp {
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
