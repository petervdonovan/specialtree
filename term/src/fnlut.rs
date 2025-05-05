// use type_equals::TypeEquals;

// pub trait HasFn<DispatchOn> {
//     type FnType;
//     fn get<T>(&self) -> Self::FnType
//     where
//         T: TypeEquals<Other = DispatchOn>;
// }
// // #[derive(Clone, Copy)]
// // pub struct PanickingFnlut;
// // impl<DispatchOn, FnType> HasFn<DispatchOn, FnType> for PanickingFnlut {
// //     fn get<T>(&self) -> FnType {
// //         panic!("no implementation provided");
// //     }
// // }

// #[macro_export]
// macro_rules! impl_fn_lut {
//     (witness_name $name:ident <$($lifetimes:lifetime),*> ; trait $trai:path ; fn_name $fn_name:ident ; get $get:ty ; types $($typ_snake_ident:ident = $typ:ty),*) => {
//         macro_rules! ty_type {
//             ($typ_snake_ident2:ident) => {
//                 fn_types::$typ_snake_ident2::Ty<$($lifetimes),*>
//             }
//         }
//         #[derive(Clone, Copy)]
//         pub struct $name<$($lifetimes),*> {
//             $(
//                 pub $typ_snake_ident: ty_type!($typ_snake_ident),
//             )*
//         }
//         // macro_rules! ctx_dependent_fn_type {
//         //     () => {
//         //         fn($($fn_args),*) -> $fn_ret_type
//         //     };
//         // }
//         macro_rules! ty_typedef {
//             () => {
//                 pub type Ty<$($lifetimes),*> = $get;
//             }
//         }
//         pub mod fn_types {
//             $(
//                 pub mod $typ_snake_ident {
//                     use super::super::*;
//                     type This = $typ;
//                     ty_typedef!();
//                 }
//             )*
//         }
//         macro_rules! impl_for_one {
//             ($typ2:ty, $typ_snake_ident2:ident) => {
//                 impl<$($lifetimes),*> $crate::fnlut::HasFn<$typ2> for $name<$($lifetimes),*>
//                 {
//                     type FnType = super::fn_types::$typ_snake_ident2::Ty<$($lifetimes),*>;
//                     #[allow(refining_impl_trait_reachable)]
//                     fn get<T>(&self) -> $get {
//                         self.$typ_snake_ident2
//                     }
//                 }
//             };
//         }
//         impl<$($lifetimes),*> $name<$($lifetimes),*>
//         // where
//         //     $(
//         //         $typ: $trai,
//         //     )*
//         {
//             pub fn new() -> Self {
//                 Self {
//                     $(
//                         $typ_snake_ident: <$typ as $trai>::$fn_name,
//                     )*
//                 }
//             }
//         }
//         $(
//             mod $typ_snake_ident {
//                 use super::*;
//                 type This = $typ;
//                 impl_for_one!($typ, $typ_snake_ident);
//             }
//         )*
//     };
// }

// #[cfg(test)]
// mod test {
//     #[derive(Clone)]
//     pub struct A;
//     #[derive(Clone)]
//     pub struct B<T>(T);

//     impl_fn_lut!(
//         witness_name CloneWitness <> ;
//         trait Clone ;
//         fn_name clone ;
//         get for<'a> fn (&'a This) -> This ;
//         types
//         a = A,
//         b = B<A>
//     );

//     #[test]
//     fn test() {
//         use super::*;
//         let a = A;
//         let b = B(a.clone());
//         let clone_lut = CloneWitness::new();
//         let _a_clone = (clone_lut.get::<A>())(&a);
//         let _b_clone = (clone_lut.get::<B<A>>())(&b);
//         // let clone_a_fn = <CloneWitness<A, B<A>> as HasFn<(&A,), A>>::get(&clone_lut);
//         // let clone_b_fn = <CloneWitness<A, B<A>> as HasFn<(&B<A>,), B<A>>>::get(&clone_lut);
//         // let cloned_a = clone_a_fn(&a);
//         // let cloned_b = clone_b_fn(&b);
//     }
// }
