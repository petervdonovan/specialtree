pub mod co_visit;
pub mod distribution_provider;
pub mod select;
pub mod tmfscore;

use distribution_provider::{DefaultDistributionProvider, DistributionProvider};
use rand::{SeedableRng, rngs::SmallRng};

pub struct ChoosingEnumerator<AllCurrentCases, DistributionProvider> {
    chosen_case: u32,
    bak: Enumerator<DistributionProvider>,
    phantom: std::marker::PhantomData<(AllCurrentCases, DistributionProvider)>,
}

pub struct Enumerator<DistributionProvider = DefaultDistributionProvider> {
    random_state: SmallRng,
    current_depth: TreeDepth,
    dp: DistributionProvider,
}

// impl Default for Enumerator {
//     fn default() -> Self {
//         Self {
//             random_state: SmallRng::seed_from_u64(0),
//             dp: DefaultDistributionProvider::new(0),
//         }
//     }
// }

// impl<AllCurrentCases, DistributionProvider> Default
//     for ChoosingEnumerator<AllCurrentCases, DistributionProvider>
// {
//     fn default() -> Self {
//         // todo: eliminate the need for this
//         Self {
//             chosen_case: 0,
//             random_state: SmallRng::seed_from_u64(0),
//             phantom: std::marker::PhantomData,
//         }
//     }
// }
#[derive(Clone, Copy)]
pub struct TreeDepth(pub u32);

impl<Dp: DistributionProvider> Enumerator<Dp> {
    pub fn new(seed: u64, depth_threshold: TreeDepth) -> Self {
        Self {
            random_state: SmallRng::seed_from_u64(seed),
            current_depth: TreeDepth(0),
            dp: Dp::from_depth_threshold(depth_threshold),
        }
    }
}
