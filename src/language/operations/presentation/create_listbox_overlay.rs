use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateListboxOverlayOp;

const DOC : &str = "Creates a listbox overlay. Individual items can be added with (overlay_add_item) and index of currently selected item can be set with (overlay_set_val). Returns listbox overlay_id. Importance of later two parameters unclear (default text&value?). 4research.";

pub const OP_CODE: u32 = 943;

pub const IDENT: &str = "create_listbox_overlay";

impl Operation for CreateListboxOverlayOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<string>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
