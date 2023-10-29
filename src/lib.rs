pub mod graph;

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Directed;
    #[test]
    fn it_works() {
        let mut graph = graph::Graph::<(), i32, Directed, i32>::new(2);
        let e1: (usize, usize) = (1, 2);
        let e2: (usize, usize) = (2, 3);

        graph.extend_with_edges(&[(1, 2), (2, 3)]);

        let e = graph.get_edge(graph::EdgeIndex::new(1));
        assert_eq!(3, e.unwrap().target().index());
    }
}
