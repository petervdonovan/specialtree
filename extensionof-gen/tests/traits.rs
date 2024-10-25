pub mod extension_of {
    pub trait Projection<LImpl, const N: u8> {
        type To;
        fn project(self, l: &LImpl) -> Self::To;
    }
    pub trait LImpl {
        type NatLit: crate::extension_of::owned::NatLit;
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
        pub trait F: Eq {
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
            fn get_ref<'a>(self, l: &'a Self::LImpl) -> Self::Ref<'a>;
            fn get_mut<'a>(self, l: &'a mut Self::LImpl) -> Self::RefMut<'a>;
        }
        pub trait Nat: Eq {
            type LImpl: crate::extension_of::LImpl;
            type Ref<'a>;
            type RefMut<'a>;
            fn NatLit(
                l: &mut Self::LImpl,
                from: <Self::LImpl as crate::extension_of::LImpl>::NatLit,
            ) -> Self;
            fn F(l: &mut Self::LImpl, from: <Self::LImpl as crate::extension_of::LImpl>::F)
                -> Self;
        }
    }
    pub mod reference {
        pub trait Projection<LImpl, const N: u8> {
            type To;
            fn project(self, l: &LImpl) -> Self::To;
        }
        pub trait F<
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
        pub trait Nat<'a>: Copy {
            type LImpl: crate::extension_of::LImpl;
            fn NatLit<'b: 'a>(
                self,
                l: &'b Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::NatLit as crate::extension_of::owned::NatLit>::Ref<
                    'a,
                >,
            >;
            fn F<'b: 'a>(
                self,
                l: &'b Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::F as crate::extension_of::owned::F>::Ref<
                    'a,
                >,
            >;
        }
    }
    pub mod mut_reference {
        pub trait F<
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
        pub trait Nat<'a>: Copy {
            type LImpl: crate::extension_of::LImpl;
            fn NatLit<'b: 'a>(
                self,
                l: &'b mut Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::NatLit as crate::extension_of::owned::NatLit>::RefMut<
                    'b,
                >,
            >;
            fn F<'b: 'a>(
                self,
                l: &'b mut Self::LImpl,
            ) -> Option<
                <<Self::LImpl as crate::extension_of::LImpl>::F as crate::extension_of::owned::F>::RefMut<
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
