pub fn hopcroft_karp_asserts(
    matching_siz: usize,
    l_to_r: &[Option<usize>],
    r_to_l: &[Option<usize>],
    mvc_l: &[bool],
    mvc_r: &[bool],
    edge_list: &[(usize, usize)],
) {
    assert_eq!(
        matching_siz,
        l_to_r.iter().filter(|elem| elem.is_some()).count()
    );
    assert_eq!(
        matching_siz,
        r_to_l.iter().filter(|elem| elem.is_some()).count()
    );

    for (i, elem) in r_to_l.iter().enumerate().filter(|(_, elem)| elem.is_some()) {
        assert_eq!(Some(i), l_to_r[elem.unwrap()]);
    }

    assert_eq!(
        matching_siz,
        mvc_l.iter().filter(|&&elem| elem).count() + mvc_r.iter().filter(|&&elem| elem).count()
    );
    for &(u, v) in edge_list.iter() {
        let either = mvc_l[u] || mvc_r[v];
        assert!(either);
    }
}
