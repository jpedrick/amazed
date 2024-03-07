use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeLabel(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<const MAX_NEIGHBORS: usize>([Option<usize>; MAX_NEIGHBORS], usize);

#[derive(Debug, PartialEq)]
pub enum AddEdgeError {
    Overflow,
    Exists
}

pub fn option_less_than(p: &Option<usize>,q: &Option<usize>) -> std::cmp::Ordering{
    match (p,q) {
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
        (Some(x), Some(y)) => x.cmp(y)
    }
} 

impl<const MAX_NEIGHBORS: usize> Neighbors<MAX_NEIGHBORS> {
    pub fn append(&mut self, node_id: usize) -> Result<&mut Self, AddEdgeError> {
        if self.1 < MAX_NEIGHBORS {
            match self.0.binary_search_by(|x| option_less_than(x,&Some(node_id))) {
                Ok(_) => Err(AddEdgeError::Exists),
                Err(pos) => {
                    if pos != self.1 {
                        self.0.copy_within(pos..self.1, pos+1);
                    } 
                    self.0[pos] = Some(node_id);
                    self.1 += 1;

                    Ok(self)
                }
            }
        } else {
            Err(AddEdgeError::Overflow)
        }
    }
}

impl<const N: usize, const MAX_NEIGHBORS: usize> From<&[usize; N]> for Neighbors<MAX_NEIGHBORS> {
    fn from(neighbors: &[usize; N]) -> Neighbors<MAX_NEIGHBORS> {
        let mut ns = Neighbors([None; MAX_NEIGHBORS], N);

        ns.0[0..N].copy_from_slice( &neighbors.map(|x|Some(x)) );

        ns
    }
}

impl<const MAX_NEIGHBORS: usize> From<&[Option<usize>; MAX_NEIGHBORS]> for Neighbors<MAX_NEIGHBORS> {
    fn from(neighbors: &[Option<usize>; MAX_NEIGHBORS]) -> Neighbors<MAX_NEIGHBORS> {
        let first_null = neighbors.iter().position(|x| x.is_none()).unwrap_or(MAX_NEIGHBORS);

        Neighbors::<MAX_NEIGHBORS>(*neighbors, first_null)
    }
}

pub type GraphNodes<const MAX_NEIGHBORS: usize> = HashMap<usize, Neighbors<MAX_NEIGHBORS>>;

#[derive(Clone, Debug)]
pub struct Graph<const MAX_NEIGHBORS: usize>(GraphNodes<MAX_NEIGHBORS>);

impl<const MAX_NEIGHBORS: usize> std::ops::Deref for Graph<MAX_NEIGHBORS> {
    type Target = GraphNodes<MAX_NEIGHBORS>;

    fn deref(&self) -> &GraphNodes<MAX_NEIGHBORS> {
        &self.0
    }
}

impl<const MAX_NEIGHBORS: usize> std::ops::DerefMut for Graph<MAX_NEIGHBORS> {
    fn deref_mut(&mut self) -> &mut GraphNodes<MAX_NEIGHBORS> {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum InsertEdgeError {
    FromMissing,
    AppendEdgeError(AddEdgeError)
}

impl<const MAX_NEIGHBORS: usize> Graph<MAX_NEIGHBORS> {
    pub fn new() -> Self {
        Graph(GraphNodes::new())
    }

    pub fn insert<const N: usize>(&mut self, node_id: usize, neighbors: &[usize; N]) {
        self.0.insert(node_id, Neighbors::from(neighbors));
    }

    pub fn insert_edge(&mut self, from: usize, to: usize) -> Result<&mut Self, InsertEdgeError> {
        if let Some(from_node) = self.0.get_mut(&from) {
            match from_node.append(to) {
                Ok(_) => Ok(self),
                Err(e) => Err(InsertEdgeError::AppendEdgeError(e))
            }
        } else {
            Err(InsertEdgeError::FromMissing)
        }
    }
}
