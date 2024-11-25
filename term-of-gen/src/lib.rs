use langspec::langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec as _, ToLiteral as _};
use langspec_gen_util::{byline, LangSpecGen, ProdGenData, SumGenData};

pub struct BasePaths {
    extension_of: syn::Path,
    reference: syn::Path,
    generate_to: syn::Path,
}

pub fn gen<L: LangSpec>(bp: &BasePaths, l: &L, ls_ty: &syn::Path) -> syn::ItemMod {
    let lg = LangSpecGen {
        bak: l,
        sort2rs_type: |_, _| panic!("should be independent of concrete types"),
        type_base_path: syn::parse_quote!(do_::not::use_::this::path),
    };
    let prods_and_sums = gen_products(&lg, bp, ls_ty).chain(gen_sums(&lg, bp, ls_ty));
    let termref = gen_termref();
    let markers = gen_markers(&lg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term {
            #termref
            #(#prods_and_sums)*
            #markers
        }
    }
}
pub(crate) fn gen_termref() -> syn::ItemStruct {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub struct TermRef<'a, LImpl, T, Marker> {
            ls: &'a LImpl,
            term: T,
            trait_: std::marker::PhantomData<Marker>,
        }
    }
}
pub(crate) fn gen_markers<L: LangSpec>(lg: &LangSpecGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let prods = lg.prod_gen_datas().map(|it| it.camel_ident);
    let sums = lg.sum_gen_datas().map(|it| it.camel_ident);
    syn::parse_quote! {
        #byline
        pub mod marker {
            #(
                pub struct #prods;
            )*
            #(
                pub struct #sums;
            )*
        }
    }
}
pub(crate) fn gen_products<L: LangSpec>(
    lg: &LangSpecGen<L>,
    BasePaths {
        extension_of,
        reference,
        generate_to,
        ..
    }: &BasePaths,
    ls_ty: &syn::Path,
) -> impl Iterator<Item = syn::ItemImpl> {
    lg.prod_gen_datas().zip(lg.bak.products()).map(
        move |(ProdGenData {
                  camel_ident,
                  idx,
                  sort_shapes,
                  ..
              }, pid)| {
            let pidlit = pid.to_literal();
            let ctor = sort_shapes.map(sort_ctor);
            let byline = byline!();
            syn::parse_quote! {
                #byline
                impl<'a, RefTy: #reference::#camel_ident<'a>> term_of::Product<'a, #ls_ty> for #generate_to::term::TermRef<'a, RefTy::LImpl, RefTy, #generate_to::term::marker::#camel_ident> {
                    fn ty_id(&self) -> <#ls_ty as langspec::langspec::LangSpec>::ProductId {
                        #pidlit
                    }
                    fn fields(&self) -> Box<dyn Iterator<Item = term_of::Term<'a, #ls_ty>> + 'a> {
                        Box::new(vec![
                            #({
                                let it = <RefTy as #extension_of::Projection<RefTy::LImpl, #idx>>::project(self.term, self.ls);
                                #ctor
                            },)*
                        ].into_iter())
                    }
                }
            }
        },
    ).collect::<Vec<_>>().into_iter()
}
pub(crate) fn gen_sums<L: LangSpec>(
    lg: &LangSpecGen<L>,
    BasePaths {
        reference,
        generate_to,
        ..
    }: &BasePaths,
    ls_ty: &syn::Path,
) -> impl Iterator<Item = syn::ItemImpl> {
    lg.sum_gen_datas().zip(lg.bak.sums()).map(
        move |(
            SumGenData {
                camel_ident,
                sort_rs_snake_idents,
                sort_shapes,
                ..
            },
            sid,
        )| {
            let sidlit = sid.to_literal();
            let byline = byline!();
            let ctor = sort_shapes.map(sort_ctor);
            syn::parse_quote! {
                #byline
                impl<'a, RefTy: #reference::#camel_ident<'a>> term_of::Sum<'a, #ls_ty> for #generate_to::term::TermRef<'a, RefTy::LImpl, RefTy, #generate_to::term::marker::#camel_ident> {
                    fn ty_id(&self) -> <#ls_ty as langspec::langspec::LangSpec>::SumId {
                        #sidlit
                    }
                    fn get(&self) -> Option<term_of::Term<'a, #ls_ty>> {
                        [
                            #(
                                self.term.#sort_rs_snake_idents(self.ls)
                                    .map(|it| #ctor)
                            ),*
                        ].into_iter()
                        .find_map(|it| it)
                    }
                }
            }
        },
    ).collect::<Vec<_>>().into_iter()
}

pub fn sort_ctor(shape: langspec::langspec::SortId<AlgebraicSortId<(), ()>>) -> syn::Expr {
    let inner: syn::Expr = syn::parse_quote! {
        crate::term::TermRef {
            ls: self.ls,
            term: it,
            trait_: std::marker::PhantomData,
        }
    };
    match shape {
        langspec::langspec::SortId::Algebraic(AlgebraicSortId::Product(_)) => {
            syn::parse_quote! {
                term_of::Term::Algebraic(term_of::AlgebraicTerm::Product(Box::new(
                    #inner
                )))
            }
        }
        langspec::langspec::SortId::Algebraic(AlgebraicSortId::Sum(_)) => {
            syn::parse_quote! {
                term_of::Term::Algebraic(term_of::AlgebraicTerm::Sum(Box::new(
                    #inner
                )))
            }
        }
        langspec::langspec::SortId::NatLiteral => {
            syn::parse_quote!(term_of::Term::NatLiteral(it.into()))
        }
        langspec::langspec::SortId::Set(_) => {
            syn::parse_quote!(term_of::Term::Set(Box::new(#inner)))
        }
        langspec::langspec::SortId::Sequence(_) => {
            syn::parse_quote!(term_of::Term::Sequence(Box::new(#inner)))
        }
    }
}

pub fn formatted(lsh: &langspec::humanreadable::LangSpecHuman) -> String {
    let lsf = langspec::flat::LangSpecFlat::canonical_from(lsh);
    let bps = BasePaths {
        extension_of: syn::parse_quote!(crate::extension_of),
        reference: syn::parse_quote!(crate::extension_of::reference),
        generate_to: syn::parse_quote!(crate),
    };
    let m = gen(
        &bps,
        &lsf,
        &syn::parse_quote! { langspec::flat::LangSpecFlat },
    );
    let extension_of = extensionof_gen::gen(&bps.extension_of, &lsf);
    prettyplease::unparse(&syn::parse_quote!(
        #m
        #extension_of
    ))
}
