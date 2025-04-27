use rand::Rng;
use rand_distr::{Distribution as _, Geometric, Poisson};
use tymetafuncspec_core::{BoundedNat, Set};

use crate::TreeDepth;

pub trait DistributionProvider {
    fn from_depth_threshold(d: TreeDepth) -> Self;
}

pub trait HasDistributionFor<T>: DistributionProvider {
    type Output;
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, depth: TreeDepth) -> Self::Output;
}

pub struct CollectionSize(pub usize);

pub struct DefaultDistributionProvider {
    depth_threshold: TreeDepth,
    nat_dist: Geometric,
}

impl DistributionProvider for DefaultDistributionProvider {
    fn from_depth_threshold(d: TreeDepth) -> Self {
        Self {
            depth_threshold: d,
            nat_dist: Geometric::new(0.125).unwrap(),
        }
    }
}

impl<H> HasDistributionFor<BoundedNat<H>> for DefaultDistributionProvider {
    type Output = BoundedNat<H>;
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, _depth: TreeDepth) -> Self::Output {
        let sample = self.nat_dist.sample(rng);
        BoundedNat::new(sample as usize)
    }
}

impl<H, E> HasDistributionFor<Set<H, E>> for DefaultDistributionProvider {
    type Output = CollectionSize;
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, depth: TreeDepth) -> Self::Output {
        // limiting expected size must be < 1
        // current implementation of Poisson is cheap to construct for small depths
        let poisson =
            Poisson::new(((self.depth_threshold.0 as f64) / (depth.0 as f64)).sqrt()).unwrap();
        CollectionSize(poisson.sample(rng) as usize)
    }
}
