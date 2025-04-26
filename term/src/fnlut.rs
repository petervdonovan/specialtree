use type_equals::TypeEquals;

pub trait HasFn<DispatchOn, ArgsType, RetType> {
    fn fn_type<T>(&self) -> impl fn_traits::Fn<ArgsType, Output = RetType>
    where
        T: TypeEquals<Other = DispatchOn>;
}
pub struct PanickingFnLut;
pub struct PanicFunction<RetType>(std::marker::PhantomData<RetType>);
impl<ArgsType, RetType> fn_traits::Fn<ArgsType> for PanicFunction<RetType> {
    type Output = RetType;

    fn call(&self, _: ArgsType) -> Self::Output {
        panic!("no implementation provided");
    }
}
impl<DispatchOn, ArgsType, RetType> HasFn<DispatchOn, ArgsType, RetType> for PanickingFnLut {
    fn fn_type<T>(&self) -> impl fn_traits::Fn<ArgsType, Output = RetType> {
        PanicFunction(std::marker::PhantomData)
    }
}

#[macro_export]
macro_rules! impl_fn_lut {
    (witness_name $name:ident ; trait $trai:tt ; fn_name $fn_name:ident ; fn_args $($fn_args:ty),* ; fn_ret_type $fn_ret_type:ty ; types $($typ_snake_ident:ident = $typ:ty),*) => {
        pub struct $name {
            $(
                $typ_snake_ident: fn_types::$typ_snake_ident::Ty,
            )*
        }
        macro_rules! ctx_dependent_fn_type {
            () => {
                fn($($fn_args),*) -> $fn_ret_type
            };
        }
        pub mod fn_types {
            $(
                pub mod $typ_snake_ident {
                    use super::super::*;
                    type This = $typ;
                    pub type Ty = ctx_dependent_fn_type!();
                }
            )*
        }
        macro_rules! impl_for_one {
            ($typ2:ty, $typ_snake_ident2:ident) => {
                impl $crate::fnlut::HasFn<$typ2, ($($fn_args,)*), $fn_ret_type> for $name
                {
                    #[allow(refining_impl_trait_reachable)]
                    fn fn_type<T>(&self) -> impl Fn($($fn_args,)*) -> $fn_ret_type {
                        self.$typ_snake_ident2
                    }
                }
            };
        }
        impl $name
        where
            $(
                $typ: $trai,
            )*
        {
            pub fn new() -> Self {
                Self {
                    $(
                        $typ_snake_ident: <$typ as $trai>::$fn_name,
                    )*
                }
            }
        }
        $(
            mod $typ_snake_ident {
                use super::*;
                type This = $typ;
                impl_for_one!($typ, $typ_snake_ident);
            }
        )*
    };
}

#[cfg(test)]
mod test {
    #[derive(Clone)]
    pub struct A;
    #[derive(Clone)]
    pub struct B<T>(T);

    impl_fn_lut!(
        witness_name CloneWitness ;
        trait Clone ;
        fn_name clone ;
        fn_args &This ;
        fn_ret_type This ;

        types
        a = A,
        b = B<A>
    );

    #[test]
    fn test() {
        use super::*;
        let a = A;
        let b = B(a.clone());
        let clone_lut = CloneWitness::new();
        let _a_clone = (clone_lut.fn_type::<A>())(&a);
        let _b_clone = (clone_lut.fn_type::<B<A>>())(&b);
        // let clone_a_fn = <CloneWitness<A, B<A>> as HasFn<(&A,), A>>::fn_type(&clone_lut);
        // let clone_b_fn = <CloneWitness<A, B<A>> as HasFn<(&B<A>,), B<A>>>::fn_type(&clone_lut);
        // let cloned_a = clone_a_fn(&a);
        // let cloned_b = clone_b_fn(&b);
    }
}
