use either_id::Either;
use langspec::tymetafunc::TyMetaFuncSpec;

pub struct TmfsJoin<Tmfs0, Tmfs1> {
    phantom: std::marker::PhantomData<(Tmfs0, Tmfs1)>,
}
impl<Tmfs0: TyMetaFuncSpec, Tmfs1: TyMetaFuncSpec> TyMetaFuncSpec for TmfsJoin<Tmfs0, Tmfs1> {
    type TyMetaFuncId = Either<Tmfs0::TyMetaFuncId, Tmfs1::TyMetaFuncId>;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> langspec::tymetafunc::TyMetaFuncData {
        match id {
            Either::Left(id) => Tmfs0::ty_meta_func_data(id),
            Either::Right(id) => Tmfs1::ty_meta_func_data(id),
        }
    }

    fn my_type() -> syn::Type {
        let tmfs0 = Tmfs0::my_type();
        let tmfs1 = Tmfs1::my_type();
        syn::parse_quote! {
            extension_everywhere_alternative::TmfsJoin<#tmfs0, #tmfs1>
        }
    }
}
