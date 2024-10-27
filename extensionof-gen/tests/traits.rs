pub mod extension_of {
    pub trait Projection<LImpl, const N: u8> {
        type To;
        fn project(self, l: &LImpl) -> Self::To;
    }
    pub trait LImpl {
        type NatLit: crate::extension_of::owned::NatLit;
        type Plus: crate::extension_of::owned::Plus;
        type F: crate::extension_of::owned::F;
        type Nat: crate::extension_of::owned::Nat;
    }
    pub trait ExtensionOf {
        type Any;
        fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
    }
    pub mod owned {
        pub trait NatLit: Into<usize> + From<usize> {
            type Ref<'a>;
            type RefMut<'a>;
        }
        pub trait Plus: Eq {
            type LImpl: crate::extension_of::LImpl;
            type Ref<'a>;
            type RefMut<'a>;
            fn new(
                l: &mut Self::LImpl,
                args: (
                    <Self::LImpl as crate::extension_of::LImpl>::Nat,
                    <Self::LImpl as crate::extension_of::LImpl>::Nat,
                ),
            ) -> Self;
            fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_>;
            fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_>;
        }
        pub trait F: Eq {
            type LImpl: crate::extension_of::LImpl;
            type Ref<'a>;
            type RefMut<'a>;
            fn new(
                l: &mut Self::LImpl,
                args: (<Self::LImpl as crate::extension_of::LImpl>::Nat),
            ) -> Self;
            fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_>;
            fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_>;
        }
        pub trait Nat: Eq {
            type LImpl: crate::extension_of::LImpl;
            type Ref<'a>;
            type RefMut<'a>;
            fn nat_lit(
                l: &mut Self::LImpl,
                from: <Self::LImpl as crate::extension_of::LImpl>::NatLit,
            ) -> Self;
            fn f(
                l: &mut Self::LImpl,
                from: <Self::LImpl as crate::extension_of::LImpl>::F,
            ) -> Self;
            fn plus(
                l: &mut Self::LImpl,
                from: <Self::LImpl as crate::extension_of::LImpl>::Plus,
            ) -> Self;
        }
    }
    pub mod reference {
        pub trait Plus<
            'a,
        >: Copy + crate::extension_of::Projection<
                Self::LImpl,
                0,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::Ref<
                    'a,
                >,
            > + crate::extension_of::Projection<
                Self::LImpl,
                1,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::Ref<
                    'a,
                >,
            > {
            type LImpl: crate::extension_of::LImpl;
        }
        pub trait F<
            'a,
        >: Copy + crate::extension_of::Projection<
                Self::LImpl,
                0,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::Ref<
                    'a,
                >,
            > {
            type LImpl: crate::extension_of::LImpl;
        }
        pub trait Nat<'a>: Copy {
            type LImpl: crate::extension_of::LImpl;
            fn nat_lit<'b: 'a>(
                self,
                l: &'b Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::NatLit as crate::extension_of::owned::NatLit>::Ref<
                    'a,
                >,
            >;
            fn f<'b: 'a>(
                self,
                l: &'b Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::F as crate::extension_of::owned::F>::Ref<
                    'a,
                >,
            >;
            fn plus<'b: 'a>(
                self,
                l: &'b Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::Plus as crate::extension_of::owned::Plus>::Ref<
                    'a,
                >,
            >;
        }
    }
    pub mod mut_reference {
        pub trait Plus<
            'a,
        >: Copy + crate::extension_of::Projection<
                Self::LImpl,
                0,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::RefMut<
                    'a,
                >,
            > + crate::extension_of::Projection<
                Self::LImpl,
                1,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::RefMut<
                    'a,
                >,
            > {
            type LImpl: crate::extension_of::LImpl;
        }
        pub trait F<
            'a,
        >: Copy + crate::extension_of::Projection<
                Self::LImpl,
                0,
                To = <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::RefMut<
                    'a,
                >,
            > {
            type LImpl: crate::extension_of::LImpl;
        }
        pub trait Nat<'a>: Copy {
            type LImpl: crate::extension_of::LImpl;
            fn nat_lit<'b: 'a>(
                self,
                l: &'b mut Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::NatLit as crate::extension_of::owned::NatLit>::RefMut<
                    'b,
                >,
            >;
            fn f<'b: 'a>(
                self,
                l: &'b mut Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::F as crate::extension_of::owned::F>::RefMut<
                    'b,
                >,
            >;
            fn plus<'b: 'a>(
                self,
                l: &'b mut Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::Plus as crate::extension_of::owned::Plus>::RefMut<
                    'b,
                >,
            >;
            fn set<'b: 'a, 'c>(
                self,
                l: &'b mut Self::LImpl,
                value: <<Self::LImpl as crate::extension_of::LImpl>::Nat as crate::extension_of::owned::Nat>::Ref<
                    'c,
                >,
            );
        }
    }
}
