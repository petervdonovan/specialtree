use langspec::langspec::{AlgebraicSortId, LangSpec, Name, SortId, SortShape};
use syn::parse_quote;

pub fn name_as_snake_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.snake, proc_macro2::Span::call_site())
}
pub fn name_as_camel_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.camel, proc_macro2::Span::call_site())
}

pub struct ProdGenData<
    I0: Iterator<Item = syn::Ident>,
    I1: Iterator<Item = syn::Ident>,
    I2: Iterator<Item = syn::Type>,
    I3: Iterator<Item = SortId<AlgebraicSortId<(), ()>>>,
> {
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub rs_ty: syn::Type,
    pub n_sorts: usize,
    pub sort_rs_camel_idents: I0,
    pub sort_rs_snake_idents: I1,
    pub sort_rs_types: I2,
    pub sort_shapes: I3,
}

pub struct SumGenData<
    I0: Iterator<Item = syn::Ident>,
    I1: Iterator<Item = syn::Ident>,
    I2: Iterator<Item = syn::Type>,
    I3: Iterator<Item = SortId<AlgebraicSortId<(), ()>>>,
> {
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub rs_ty: syn::Type,
    pub n_sorts: usize,
    pub sort_rs_camel_idents: I0,
    pub sort_rs_snake_idents: I1,
    pub sort_rs_types: I2,
    pub sort_shapes: I3,
}

#[macro_export]
macro_rules! transpose {
    ($records:expr, $($field:ident),*) => {
        $(
            let mut $field = vec![];
        )*
        for record67142647 in $records {
            $(
                $field.push(record67142647.$field);
            )*
        }
        $(
            let $field = $field;
        )*
    };
}

pub struct LangSpecGen<'a, L: LangSpec> {
    pub bak: &'a L,
    pub sort2rs_type: fn(SortId<syn::Type>) -> syn::Type,
}

impl<'a, L: LangSpec> LangSpecGen<'a, L> {
    pub fn asi2rs_type(&self, asi: AlgebraicSortId<L::ProductId, L::SumId>) -> syn::Type {
        let name = self.bak.algebraic_sort_name(asi);
        let ty_name = name_as_camel_ident(name);
        parse_quote!(#ty_name)
    }
    pub fn sort2rs_type(&self, sort: SortId<L::AlgebraicSortId>) -> syn::Type {
        (self.sort2rs_type)(sort.fmap(|it| self.asi2rs_type(self.bak.asi_convert(it))))
    }
    pub fn asi2rs_ident(&self, asi: AlgebraicSortId<L::ProductId, L::SumId>) -> syn::Ident {
        let name = self.bak.algebraic_sort_name(asi);
        name_as_camel_ident(name)
    }
    pub fn sort2rs_ident(
        &self,
        sort: SortId<AlgebraicSortId<L::ProductId, L::SumId>>,
    ) -> syn::Ident {
        match sort {
            SortId::NatLiteral => syn::Ident::new("NatLit", proc_macro2::Span::call_site()),
            SortId::Algebraic(asi) => self.asi2rs_ident(asi),
            SortId::Set(asi) => syn::Ident::new(
                &format!("NatSetOf{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
            SortId::Sequence(asi) => syn::Ident::new(
                &format!("NatSeqOf{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
        }
    }
    pub fn sort2rs_snake_ident(
        &self,
        sort: SortId<AlgebraicSortId<L::ProductId, L::SumId>>,
    ) -> syn::Ident {
        match sort {
            SortId::NatLiteral => syn::Ident::new("nat_lit", proc_macro2::Span::call_site()),
            SortId::Algebraic(asi) => {
                let name = self.bak.algebraic_sort_name(asi);
                name_as_snake_ident(name)
            }
            SortId::Set(asi) => syn::Ident::new(
                &format!("nat_set_of_{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
            SortId::Sequence(asi) => syn::Ident::new(
                &format!("nat_seq_of_{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
        }
    }
    pub fn prod_gen_datas<'b>(
        &'b self,
    ) -> impl Iterator<
        Item = ProdGenData<
            impl Iterator<Item = syn::Ident> + 'b,
            impl Iterator<Item = syn::Ident> + 'b,
            impl Iterator<Item = syn::Type> + 'b,
            impl Iterator<Item = SortId<AlgebraicSortId<(), ()>>> + 'b,
        >,
    > + 'b
    where
        'a: 'b,
    {
        self.bak.products().map(move |id| {
            let snake_ident = syn::Ident::new(
                &self.bak.product_name(id.clone()).snake.clone(),
                proc_macro2::Span::call_site(),
            );
            let camel_ident = syn::Ident::new(
                &self.bak.product_name(id.clone()).camel.clone(),
                proc_macro2::Span::call_site(),
            );
            let rs_ty = parse_quote!(#camel_ident);
            let sort_rs_camel_idents = self
                .bak
                .product_sorts(id.clone())
                .map(|sort| self.sort2rs_ident(self.bak.sid_convert(sort)));
            let sort_rs_snake_idents = self
                .bak
                .product_sorts(id.clone())
                .map(|sort| self.sort2rs_snake_ident(self.bak.sid_convert(sort)));
            let sort_rs_types = self
                .bak
                .product_sorts(id.clone())
                .map(|sort| self.sort2rs_type(sort));
            let sort_shapes = self
                .bak
                .product_sorts(id.clone())
                .map(|it| SortShape::project(self.bak, it));
            let n_sorts = self.bak.product_sorts(id).count();
            ProdGenData {
                snake_ident,
                camel_ident,
                rs_ty,
                n_sorts,
                sort_rs_camel_idents,
                sort_rs_snake_idents,
                sort_rs_types,
                sort_shapes,
            }
        })
    }
    pub fn sum_gen_datas<'b>(
        &'b self,
    ) -> impl Iterator<
        Item = SumGenData<
            impl Iterator<Item = syn::Ident> + 'b,
            impl Iterator<Item = syn::Ident> + 'b,
            impl Iterator<Item = syn::Type> + 'b,
            impl Iterator<Item = SortShape> + 'b,
        >,
    > + 'b
    where
        'a: 'b,
    {
        self.bak.sums().map(move |id| {
            let snake_ident = syn::Ident::new(
                &self.bak.sum_name(id.clone()).snake.clone(),
                proc_macro2::Span::call_site(),
            );
            let camel_ident = syn::Ident::new(
                &self.bak.sum_name(id.clone()).camel.clone(),
                proc_macro2::Span::call_site(),
            );
            let rs_ty = parse_quote!(#camel_ident);
            let sort_rs_camel_idents = self
                .bak
                .sum_sorts(id.clone())
                .map(|sort| self.sort2rs_ident(self.bak.sid_convert(sort)));
            let sort_rs_snake_idents = self
                .bak
                .sum_sorts(id.clone())
                .map(|sort| self.sort2rs_snake_ident(self.bak.sid_convert(sort)));
            let sort_rs_types = self
                .bak
                .sum_sorts(id.clone())
                .map(|sort| self.sort2rs_type(sort));
            let sort_shapes = self
                .bak
                .sum_sorts(id.clone())
                .map(|it| it.fmap(|it| self.bak.asi_convert(it).fmap_p(|_| ()).fmap_s(|_| ())));
            let n_sorts = self.bak.sum_sorts(id).count();
            SumGenData {
                snake_ident,
                camel_ident,
                rs_ty,
                n_sorts,
                sort_rs_camel_idents,
                sort_rs_snake_idents,
                sort_rs_types,
                sort_shapes,
            }
        })
    }
}
