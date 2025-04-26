use type_equals::TypeEquals;

pub trait HasFn<ArgsType, RetType> {
    fn fn_type<T>(&self) -> impl fn_traits::Fn<ArgsType, Output = RetType>
    where
        T: TypeEquals<Other = ArgsType>;
}
// pub struct PanickingFnLut;
// impl<ArgsType, RetType> HasFn<ArgsType, RetType> for PanickingFnLut {
//     fn fn_type(&self) -> impl fn_traits::Fn<ArgsType, Output = RetType> {
//         |_: ArgsType| panic!("PanickingFnLut")
//     }
// }

macro_rules! impl_fn_lut {
    (witness_name $name:ident ; trait $trai:tt ; fn_name $fn_name:ident ; fn_args $($fn_args:ty),* ; fn_ret_type $fn_ret_type:ty ; types $($typ_snake_ident:ident / $typ_camel_ident:ident = $typ:ty),*) => {
        pub struct $name<
            $(
                $typ_camel_ident,
            )*
        > {
            $(
                $typ_snake_ident: $typ_camel_ident,
            )*
        }
        macro_rules! impl_for_one {
            ($typ2:ty, $typ_camel_ident2:ident, $typ_snake_ident2:ident) => {
                impl<
                    $(
                        $typ_camel_ident,
                    )*
                > $crate::fnlut::HasFn<($($fn_args,)*), $fn_ret_type> for $name<$(
                    $typ_camel_ident,
                )*>
                    where $typ_camel_ident2: Fn($($fn_args,)*) -> $fn_ret_type + Copy,
                {
                    #[allow(refining_impl_trait_reachable)]
                    fn fn_type<T>(&self) -> impl Fn($($fn_args,)*) -> $fn_ret_type {
                        self.$typ_snake_ident2
                    }
                }
            };
        }
        // impl<
        //     $(
        //         $typ_camel_ident: $trai,
        //     )*
        // > $name<$(
        //     $typ_camel_ident,
        // )*>
        // where
        //     $(
        //         $typ_camel_ident: $trai,
        //     )*
        // {
        //     pub fn new() -> Self {
        //         Self {
        //             $(
        //                 $typ_snake_ident: <$typ_camel_ident as $trai>::$fn_name,
        //             )*
        //         }
        //     }
        // }
        $(
            mod $typ_snake_ident {
                use super::*;
                type This = $typ;
                impl_for_one!($typ, $typ_camel_ident, $typ_snake_ident);
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
        a / AArg = A,
        b / BArg = B<A>
    );

    #[test]
    fn test() {
        use super::*;
        let a = A;
        let b = B(a.clone());
        let clone_lut = CloneWitness {
            a: A::clone,
            b: <B<A> as Clone>::clone,
        };
        let a_clone = (clone_lut.fn_type::<(&A,)>())(&a);
        let b_clone = (clone_lut.fn_type::<(&B<A>,)>())(&b);
        // let clone_a_fn = <CloneWitness<A, B<A>> as HasFn<(&A,), A>>::fn_type(&clone_lut);
        // let clone_b_fn = <CloneWitness<A, B<A>> as HasFn<(&B<A>,), B<A>>>::fn_type(&clone_lut);
        // let cloned_a = clone_a_fn(&a);
        // let cloned_b = clone_b_fn(&b);
    }
}
