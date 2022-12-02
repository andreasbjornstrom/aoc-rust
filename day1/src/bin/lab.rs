fn lab() {
    struct Pryl<'a> {
        pryl: Elf,
        sub_pryl: SubPryl<'a>,
    }

    struct SubPryl<'a> {
        ref_pryl: &'a Elf,
    }
    let pryl_varde = Elf { index: 1, sum: 0 };

    let mut pryl = Pryl {
        pryl: pryl_varde,
        sub_pryl: SubPryl {
            ref_pryl: &Elf { index: 0, sum: 0 },
        },
    };

    pryl.sub_pryl.ref_pryl = &pryl.pryl;
    let array = vec![vec![1, 2, 3], vec![1, 2, 3]];
    let stor_vec = vec![0; 64];
    dbg!(&array);
}
