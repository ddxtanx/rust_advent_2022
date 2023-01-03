pub struct Node<T> {
    pub val: T,
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>
}

impl<T> Node<T> {
    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

pub struct ArenaTree<T> {
    nodes: Vec<Node<T>>
}

#[derive(Debug)]
pub enum GraphError {
    ParentIdxOutOfBounds,
    ChildIdxOutOfBounds,
    ChildAlreadyHasParent
}

impl<T: PartialEq> ArenaTree<T> {
    pub fn new_node(&mut self, val: T) -> usize {
        let idx = self.nodes.len();
        let node = Node {
            val,
            idx,
            parent: None,
            children: vec![]
        };

        self.nodes.push(node);

        idx
    }

    pub fn add_child(&mut self, par_idx: usize, child_idx: usize) -> Result<(), GraphError> {
        if par_idx >= self.nodes.len() {
            return Err(GraphError::ParentIdxOutOfBounds);
        }

        if par_idx >= self.nodes.len() {
            return Err(GraphError::ChildIdxOutOfBounds);
        }

        let child_node = self.nodes.get_mut(child_idx).unwrap();

        if child_node.parent.is_some() {
            return Err(GraphError::ChildAlreadyHasParent);
        }
        
        child_node.parent = Some(par_idx);

        let par_node = self.nodes.get_mut(par_idx).unwrap();
        par_node.children.push(child_idx);
        Ok(())
    }

    pub fn new_node_as_child(&mut self, par_node_idx: usize, child_val: T) -> Result<(), GraphError> {
        let child_idx = self.new_node(child_val);
        self.add_child(par_node_idx, child_idx)
    }

    pub fn get_root_node(&self) -> Option<&Node<T>> {
        self.nodes.get(0)
    }

    pub fn get_children(&self, par_idx: usize) -> Result<Vec<&Node<T>>, GraphError> {
        if par_idx >= self.nodes.len() {
            return Err(GraphError::ParentIdxOutOfBounds);
        }

        Ok(self.nodes.get(par_idx).unwrap().
           children.iter().
           map(|idx| self.nodes.get(*idx).unwrap())
           .collect())
    }

    pub fn get_node_children(&self, par_node: &Node<T>) -> Result<Vec<&Node<T>>, GraphError> {
        self.get_children(par_node.idx)
    }

    pub fn get_child_by_value(&self, par_node_idx: usize, val: T) -> Option<&Node<T>> {
        let idx = self.get_children(par_node_idx).unwrap().
            iter().find(|node| node.val == val).unwrap().
            idx;
        self.nodes.get(idx)

    }

    pub fn get_parent_node(&self, node_idx: usize) -> Option<&Node<T>> {
        let node_opt = self.nodes.get(node_idx);
        match node_opt {
            Some(node) => match node.parent {
                Some(idx) => self.nodes.get(idx),
                None => None
            },
            None => None
        }
    }

    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node_by_idx(&self, idx: usize) -> Option<&Node<T>> {
        self.nodes.get(idx)
    }

    pub fn new() -> Self {
        ArenaTree {
            nodes: vec![]
        }
    }
}
