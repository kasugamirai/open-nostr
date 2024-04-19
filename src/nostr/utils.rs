use indextree::{Arena, NodeId};

/// Utility function to get all children of a specified node in an Arena.
/// 
/// # Arguments
/// * `arena` - A reference to an Arena where the nodes are stored.
/// * `node_id` - The NodeId of the parent node whose children are to be collected.
/// 
/// # Returns
/// A vector of references to the child node data.
pub fn get_children<T>(arena: &Arena<T>, node_id: NodeId) -> Vec<&T> {
    let mut children = Vec::new();
    if let Some(first_child_id) = arena[node_id].first_child() {
        let mut current_id = Some(first_child_id);
        while let Some(node_id) = current_id {
            if let Some(node) = arena.get(node_id) {
                children.push(node.get());
            }
            current_id = arena[node_id].next_sibling();
        }
    }
    children
}

/// Collects all ancestor data of the specified node into a vector.
pub fn get_ancestors<T>(arena: &Arena<T>, node_id: NodeId) -> Vec<&T> {
    let mut ancestors = Vec::new();
    let mut current_id = Some(node_id);

    // Traverse up the tree by continuously going to the parent node
    while let Some(id) = current_id {
        if let Some(parent_id) = arena[id].parent() {
            if let Some(parent) = arena.get(parent_id) {
                ancestors.push(parent.get());
            }
            current_id = Some(parent_id);
        } else {
            break;
        }
    }

    ancestors
}