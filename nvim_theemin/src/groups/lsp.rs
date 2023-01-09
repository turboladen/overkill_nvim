/// :h lsp-highlight
///
pub trait Highlighting {
    def_group_fn!(lsp_reference_text);
    def_group_fn!(lsp_reference_read);
    def_group_fn!(lsp_reference_write);

    def_group_fn!(lsp_code_lens);
    def_group_fn!(lsp_code_lens_separator);
    def_group_fn!(lsp_signature_active_parameter);
}
