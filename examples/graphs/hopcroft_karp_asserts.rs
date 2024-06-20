pub fn hopcroft_karp_asserts(
    matching_siz: usize,
    l_to_r: &[Option<usize>],
    r_to_l: &[Option<usize>],
    mvc_l: &[bool],
    mvc_r: &[bool],
    edge_list: &[(usize, usize)],
) {
    for (left, right) in [(l_to_r, r_to_l), (r_to_l, l_to_r)] {
        assert_eq!(
            matching_siz,
            left.iter().filter(|elem| elem.is_some()).count()
        );
        for (i, elem) in left.iter().enumerate().filter(|(_, elem)| elem.is_some()) {
            assert_eq!(Some(i), right[elem.unwrap()]);
        }
    }

    fn count_true(a: &[bool]) -> usize {
        a.iter().filter(|&&elem| elem).count()
    }
    assert_eq!(matching_siz, count_true(mvc_l) + count_true(mvc_r));

    for &(u, v) in edge_list.iter() {
        // this might look weird, but it's done so that code coverage passes:
        //
        // this:
        // assert!(mvc_l[u] || mvc_r[v]);
        // will fail code coverage, saying that the false branch is never run
        let either = mvc_l[u] || mvc_r[v];
        assert!(either);
    }
}
