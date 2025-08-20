#[cfg(test)]
mod tests {
    use crate::{
        ccfs_exploded_by_unit_paths, get_direct_ccf_rels,
        unit_ccf_paths_quadratically_large_closure,
    };
    use langspec::humanreadable::LangSpecHuman;
    use langspec::langspec::SortIdOf;
    use langspec_examples;

    #[test]
    fn test_gdcr() {
        let ls = langspec_examples::fib();
        type L = LangSpecHuman<tymetafuncspec_core::Core>;
        let dcr = get_direct_ccf_rels(&ls);
        for rel in &dcr {
            println!("{rel:?}\n");
        }
        let non_transparent_sorts = &[
            langspec::humanreadable::SortId::<tymetafuncspec_core::Core>::Algebraic(
                langspec::langspec::AlgebraicSortId::Sum("ℕ".into()),
            ),
        ];
        let ucr = unit_ccf_paths_quadratically_large_closure::<SortIdOf<L>>(
            dcr.as_slice(),
            non_transparent_sorts,
        );
        for rel in &ucr {
            println!("{rel:?}\n");
        }
        let cebup = ccfs_exploded_by_unit_paths(dcr.as_slice(), &ucr, non_transparent_sorts);
        for rel in &cebup {
            println!("{rel:?}\n");
        }
        println!("Direct CCF relations: {}", dcr.len());
        println!("Unit CCF relations: {}", ucr.len());
        println!("Exploded CCF relations: {}", cebup.len());
    }
}
