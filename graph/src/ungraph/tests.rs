use crate::{UnGraph, WeiEdge, Weight};
use std::io::{self, Write};

fn sample() -> UnGraph {
    let mut graph = UnGraph::new();

    graph.add_edge(35, (4, 5), (), ());
    graph.add_edge(37, (4, 7), (), ());
    graph.add_edge(28, (5, 7), (), ());
    graph.add_edge(16, (0, 7), (), ());
    graph.add_edge(32, (1, 5), (), ());
    graph.add_edge(38, (0, 4), (), ());
    graph.add_edge(17, (2, 3), (), ());
    graph.add_edge(19, (1, 7), (), ());
    graph.add_edge(26, (0, 2), (), ());
    graph.add_edge(36, (1, 2), (), ());
    graph.add_edge(29, (1, 3), (), ());
    graph.add_edge(34, (2, 7), (), ());
    graph.add_edge(40, (6, 2), (), ());
    graph.add_edge(52, (3, 6), (), ());
    graph.add_edge(58, (6, 0), (), ());
    graph.add_edge(93, (6, 4), (), ());

    graph
}

fn print_mst(name: &'static str, mst: Vec<&WeiEdge>) -> io::Result<()> {
    let weight_sum = weight_sum(&mst);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(name.as_bytes())?;
    for edge in mst {
        handle.write_fmt(format_args!("{:?}\n", edge))?;
    }
    handle.write_fmt(format_args!("{:?}\n", weight_sum))?;

    Ok(())
}

fn weight_sum(mst: &Vec<&WeiEdge>) -> Weight {
    mst.into_iter().map(|&edge| edge.weight).sum()
}

#[test]
#[allow(unused_must_use)]
fn lazy_prim_mst() {
    let graph = sample();
    let mst = graph.lazy_prim_mst(0).unwrap();
    print_mst("lazy_prim_mst\n", mst);
}

#[test]
#[allow(unused_must_use)]
fn prim_mst() {
    let graph = sample();
    let mst = graph.prim_mst(0).unwrap();
    print_mst("prim_mst\n", mst);
}

#[test]
#[allow(unused_must_use)]
fn kruskal_mst() {
    let graph = sample();
    let mst = graph.kruskal_mst().unwrap();
    print_mst("kruskal_mst\n", mst);
}
