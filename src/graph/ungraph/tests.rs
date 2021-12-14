use super::{Edge, UnGraph};
use std::io::{self, Write};
use std::rc::Weak;

fn sample_graph() -> UnGraph<()> {
    let mut graph: UnGraph<()> = UnGraph::new();
    graph.add_edge((4, 5, 35), (), ());
    graph.add_edge((4, 7, 37), (), ());
    graph.add_edge((5, 7, 28), (), ());
    graph.add_edge((0, 7, 16), (), ());
    graph.add_edge((1, 5, 32), (), ());
    graph.add_edge((0, 4, 38), (), ());
    graph.add_edge((2, 3, 17), (), ());
    graph.add_edge((1, 7, 19), (), ());
    graph.add_edge((0, 2, 26), (), ());
    graph.add_edge((1, 2, 36), (), ());
    graph.add_edge((1, 3, 29), (), ());
    graph.add_edge((2, 7, 34), (), ());
    graph.add_edge((6, 2, 40), (), ());
    graph.add_edge((3, 6, 52), (), ());
    graph.add_edge((6, 0, 58), (), ());
    graph.add_edge((6, 4, 93), (), ());
    graph
}

fn test_io(ms_tree: Vec<Weak<Edge>>, name: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(name.as_bytes())?;
    for edge in ms_tree {
        handle.write_fmt(format_args!("{:?}\n", Weak::upgrade(&edge).unwrap()))?;
    }
    handle.write(b"\n")?;

    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn test_lazy_prim_mst() {
    let graph = sample_graph();
    let ms_tree = graph.lazy_prim_mst(0).unwrap();
    test_io(ms_tree, "lazy_prim_mst\n");
}

#[test]
#[allow(unused_must_use)]
fn test_prim_mst() {
    let graph = sample_graph();
    let ms_tree = graph.prim_mst(0).unwrap();
    test_io(ms_tree, "prim_mst\n");
}

#[test]
#[allow(unused_must_use)]
fn test_kruskal_mst() {
    let graph = sample_graph();
    let ms_tree = graph.kruskal_mst();
    test_io(ms_tree, "kruskal_mst\n");
}
