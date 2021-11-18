use super::UnGraph;
use std::io::{self, Write};
use std::rc::Weak;

fn sample_graph<'a>() -> UnGraph<'a, ()> {
    let mut graph: UnGraph<'a, ()> = UnGraph::new();
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

#[test]
fn test_lazy_prim_mst() -> io::Result<()> {
    let graph = sample_graph();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let ms_tree = graph.lazy_prim_mst(0);

    handle.write(b"lazy_prim_mst\n")?;
    for edge in ms_tree {
        handle.write_fmt(format_args!("{:?}\n", Weak::upgrade(&edge).unwrap()))?;
    }
    handle.write(b"\n")?;
    Ok(())
}

#[test]
fn test_prim_mst() -> io::Result<()> {
    let graph = sample_graph();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let ms_tree = graph.prim_mst(0);

    handle.write(b"prim_mst\n")?;
    for edge in ms_tree {
        handle.write_fmt(format_args!("{:?}\n", Weak::upgrade(&edge).unwrap()))?;
    }
    handle.write(b"\n")?;
    Ok(())
}

#[test]
fn test_kruskal_mst() -> io::Result<()> {
    let graph = sample_graph();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let ms_tree = graph.kruskal_mst();

    handle.write(b"kruskal_mst\n")?;
    for edge in ms_tree {
        handle.write_fmt(format_args!("{:?}\n", Weak::upgrade(&edge).unwrap()))?;
    }
    handle.write(b"\n")?;
    Ok(())
}
