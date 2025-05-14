pub trait ToLiteral {
    fn to_literal(&self) -> syn::Expr;
}
impl ToLiteral for usize {
    fn to_literal(&self) -> syn::Expr {
        syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Int(syn::LitInt::new(
                &self.to_string(),
                proc_macro2::Span::call_site(),
            )),
        })
    }
}

impl ToLiteral for String {
    fn to_literal(&self) -> syn::Expr {
        syn::parse_quote! { #self }
    }
}
