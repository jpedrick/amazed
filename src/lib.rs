pub mod graph;
pub mod geometry;

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    #[test]
    fn test_insert() {
        let mut g = Graph::<usize,6>::new();
        g.insert(1, &[2, 3, 4, 5]);
        g.insert(2, &[1, 3, 4]);
        g.insert(3, &[2, 3, 4]);
        g.insert(4, &[1, 2, 3]);
        g.insert(5, &[1]);

        let s = graph::GraphNodes::<usize,6>::from([
            (1, (&[Some(2), Some(3), Some(4), Some(5), None, None]).into()),
            (2, (&[Some(1), Some(3), Some(4), None, None, None]).into()),
            (3, (&[Some(2), Some(3), Some(4), None, None, None]).into()),
            (4, (&[Some(1), Some(2), Some(3), None, None, None]).into()),
            (5, (&[Some(1), None, None, None, None, None]).into()),
        ]);
        assert_eq!(*g, s);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::<usize,6>::new();
        g.insert(1, &[2, 3, 4, 5]);
        g.insert(2, &[1, 3, 4]);
        g.insert(3, &[2, 3, 4]);
        g.insert(4, &[1, 2, 3]);
        g.insert(5, &[1]);
        g.insert_edge(5, 3).expect("This should be ok");
        g.insert_edge(5, 2).expect("This should be ok");
        assert_eq!(g.insert_edge(5, 2).err(), Some(crate::graph::InsertEdgeError::AppendEdgeError(crate::graph::AddEdgeError::Exists)));

        let s = graph::GraphNodes::<usize,6>::from([
            (1, (&[Some(2), Some(3), Some(4), Some(5), None, None]).into()),
            (2, (&[Some(1), Some(3), Some(4), None, None, None]).into()),
            (3, (&[Some(2), Some(3), Some(4), None, None, None]).into()),
            (4, (&[Some(1), Some(2), Some(3), None, None, None]).into()),
            (5, (&[Some(1), Some(2), Some(3), None, None, None]).into()),
        ]);
        assert_eq!(*g, s);
    }

    #[test]
    fn test_overflow_node() {
        let mut g = Graph::<usize,6>::new();
        g.insert(1, &[2, 3, 4, 5]);
        g.insert(2, &[1, 3, 4]);
        g.insert(3, &[2, 3, 4]);
        g.insert(4, &[1, 2, 3]);
        g.insert(5, &[1]);
        g.insert(6, &[1]);
        g.insert(7, &[1]);
        g.insert(8, &[1]);
        g.insert_edge(1, 6).expect("Ok");
        g.insert_edge(1, 7).expect("Ok");
        assert_eq!(g.insert_edge(1, 8).err(), Some(crate::graph::InsertEdgeError::AppendEdgeError(crate::graph::AddEdgeError::Overflow)));
        assert_eq!(g.insert_edge(9, 1).err(), Some(crate::graph::InsertEdgeError::FromMissing));
    }
}
