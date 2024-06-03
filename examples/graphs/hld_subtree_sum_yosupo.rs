// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut adj = vec![vec![]; n];
    for c in 1..n {
        input! {
            p: usize,
        }
        adj[c].push(p);
        adj[p].push(c);
    }

    let hld_nodes = HLD::new(&mut adj, false);
    let mut fenwick = Fenwick::<usize>::new(n);

    let hld_edges = HLD::new(&mut adj, true);

    for (i, &elem) in a.iter().enumerate() {
        fenwick.add(hld_nodes.tin[i], elem);
    }

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    u: usize,
                    delta: usize,
                }
                fenwick.add(hld_nodes.tin[u], delta);
            }
            _ => {
                input! {
                    u: usize,
                }
                let nodes_subtree = hld_nodes.sub_tree(u);
                let edges_subtree = hld_edges.sub_tree(u);
                assert_eq!(nodes_subtree.start + 1, edges_subtree.start);
                assert_eq!(nodes_subtree.end, edges_subtree.end);
                println!("{}", fenwick.sum(nodes_subtree));
            }
        }
    }
}
