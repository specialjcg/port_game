// MCTS Tree structure

use super::actions::MCTSAction;
use crate::domain::aggregates::Port;

/// MCTS tree node
#[derive(Debug, Clone)]
pub struct MCTSNode {
    pub state: Port,
    pub action: Option<MCTSAction>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub visits: usize,
    pub total_score: f64,
    pub depth: usize,
}

impl MCTSNode {
    pub fn new(
        state: Port,
        action: Option<MCTSAction>,
        parent: Option<usize>,
        depth: usize,
    ) -> Self {
        Self {
            state,
            action,
            parent,
            children: Vec::new(),
            visits: 0,
            total_score: 0.0,
            depth,
        }
    }

    pub fn average_score(&self) -> f64 {
        if self.visits == 0 {
            0.0
        } else {
            self.total_score / self.visits as f64
        }
    }

    /// UCB1 formula for node selection
    pub fn ucb1(&self, parent_visits: usize, exploration_constant: f64) -> f64 {
        if self.visits == 0 {
            f64::INFINITY // Always explore unvisited nodes first
        } else {
            let exploitation = self.average_score();
            let exploration =
                exploration_constant * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
            exploitation + exploration
        }
    }
}

/// MCTS tree
#[derive(Debug, Clone)]
pub struct MCTSTree {
    nodes: Vec<MCTSNode>,
    root_id: Option<usize>,
}

impl MCTSTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_id: None,
        }
    }

    pub fn init_root(&mut self, state: Port) {
        let root = MCTSNode::new(state, None, None, 0);
        self.nodes.clear();
        self.nodes.push(root);
        self.root_id = Some(0);
    }

    pub fn select_ucb1(&self, exploration_constant: f64) -> usize {
        let mut current_id = self.root_id.expect("Tree not initialized");

        loop {
            let node = &self.nodes[current_id];

            if node.children.is_empty() {
                return current_id;
            }

            // Select child with highest UCB1
            let parent_visits = node.visits;
            current_id = *node
                .children
                .iter()
                .max_by(|&&a, &&b| {
                    let ucb_a = self.nodes[a].ucb1(parent_visits, exploration_constant);
                    let ucb_b = self.nodes[b].ucb1(parent_visits, exploration_constant);
                    ucb_a.partial_cmp(&ucb_b).unwrap()
                })
                .expect("Children exist but none selected");
        }
    }

    pub fn expand(&mut self, node_id: usize, max_depth: usize) -> usize {
        // Clone necessary data before modifying self.nodes
        let (state, depth) = {
            let node = &self.nodes[node_id];
            (node.state.clone(), node.depth)
        };

        if depth >= max_depth {
            return node_id;
        }

        // Generate possible actions (simplified for MVP)
        let actions = self.generate_actions(&state);

        if actions.is_empty() {
            return node_id; // No expansion possible
        }

        // Create child nodes for each action
        let mut child_ids = Vec::new();
        for action in actions {
            let mut new_state = state.clone();
            Self::apply_action_to_state(&mut new_state, &action);
            let child = MCTSNode::new(new_state, Some(action), Some(node_id), depth + 1);
            let child_id = self.nodes.len();
            self.nodes.push(child);
            child_ids.push(child_id);
        }

        self.nodes[node_id].children = child_ids.clone();

        // Return first child for simulation
        child_ids.first().copied().unwrap_or(node_id)
    }

    pub fn backpropagate(&mut self, mut node_id: usize, score: f64) {
        loop {
            let node = &mut self.nodes[node_id];
            node.visits += 1;
            node.total_score += score;

            match node.parent {
                Some(parent_id) => node_id = parent_id,
                None => break,
            }
        }
    }

    pub fn best_action(&self) -> Option<MCTSAction> {
        let root_id = self.root_id?;
        let root = &self.nodes[root_id];

        if root.children.is_empty() {
            return None;
        }

        // Select child with highest visit count (most explored)
        let best_child_id = root
            .children
            .iter()
            .max_by_key(|&&id| self.nodes[id].visits)?;

        self.nodes[*best_child_id].action.clone()
    }

    pub fn get_state(&self, node_id: usize) -> &Port {
        &self.nodes[node_id].state
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn max_depth(&self) -> usize {
        self.nodes.iter().map(|n| n.depth).max().unwrap_or(0)
    }

    pub(crate) fn node_depth(&self, node_id: usize) -> usize {
        self.nodes
            .get(node_id)
            .map(|node| node.depth)
            .unwrap_or_default()
    }

    pub(crate) fn generate_actions(&self, port: &Port) -> Vec<MCTSAction> {
        let mut actions = Vec::new();

        // Generate DockShip actions
        for ship in port.waiting_ships() {
            for berth in port.free_berths() {
                actions.push(MCTSAction::DockShip {
                    ship_id: ship.id,
                    berth_id: berth.id,
                });
            }
        }

        // Generate AssignCrane actions
        for crane in port.free_cranes() {
            for ship in port.docked_ships() {
                actions.push(MCTSAction::AssignCrane {
                    crane_id: crane.id,
                    ship_id: ship.id,
                });
            }
        }

        // Always have Pass as an option
        if actions.is_empty() {
            actions.push(MCTSAction::Pass);
        }

        actions
    }

    pub(crate) fn apply_action_to_state(state: &mut Port, action: &MCTSAction) {
        match action {
            MCTSAction::DockShip { ship_id, berth_id } => {
                if let Some(ship) = state.ships.get_mut(ship_id) {
                    ship.dock(*berth_id);
                }
                if let Some(berth) = state.berths.get_mut(berth_id) {
                    berth.occupy(*ship_id);
                }
                state.current_time += 1.0;
            }
            MCTSAction::AssignCrane { crane_id, ship_id } => {
                if let Some(crane) = state.cranes.get_mut(crane_id) {
                    crane.assign(*ship_id);
                }
                if let Some(ship) = state.ships.get_mut(ship_id) {
                    ship.assign_crane(*crane_id);
                }
                state.current_time += 1.0;
            }
            MCTSAction::UnassignCrane { crane_id } => {
                if let Some(crane) = state.cranes.get_mut(crane_id) {
                    if let Some(assigned_ship) = crane.assigned_to {
                        if let Some(ship) = state.ships.get_mut(&assigned_ship) {
                            ship.unassign_crane(*crane_id);
                        }
                    }
                    crane.unassign();
                }
                state.current_time += 0.5;
            }
            MCTSAction::Pass => {
                state.current_time += 0.5;
            }
        }

        // Simple heuristic: process containers for docked ships with assigned cranes
        let ship_ids: Vec<_> = state.ships.keys().copied().collect();
        let mut ships_to_remove = Vec::new();

        for ship_id in ship_ids {
            if let Some(ship) = state.ships.get_mut(&ship_id) {
                if ship.is_docked() && !ship.assigned_cranes.is_empty() {
                    let crane_count = ship.assigned_cranes.len() as u32;
                    let processed = 10 * crane_count;
                    ship.process_containers(processed);

                    if ship.is_completed() {
                        // Free cranes assigned to the ship
                        for crane_id in ship.assigned_cranes.clone() {
                            if let Some(crane) = state.cranes.get_mut(&crane_id) {
                                crane.unassign();
                            }
                        }

                        if let Some(berth_id) = ship.docked_at {
                            if let Some(berth) = state.berths.get_mut(&berth_id) {
                                berth.free();
                            }
                        }

                        ships_to_remove.push(ship_id);
                    }
                }
            }
        }

        if !ships_to_remove.is_empty() {
            for ship_id in ships_to_remove {
                state.ships.remove(&ship_id);
            }
        }
    }
}

impl Default for MCTSTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::PlayerId;

    #[test]
    fn test_tree_initialization() {
        let mut tree = MCTSTree::new();
        let port = Port::new(PlayerId::new(), 2, 2);

        tree.init_root(port);

        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.root_id, Some(0));
    }

    #[test]
    fn test_ucb1_unvisited_node() {
        let port = Port::new(PlayerId::new(), 2, 2);
        let node = MCTSNode::new(port, None, None, 0);

        let ucb = node.ucb1(10, 1.41);

        assert_eq!(ucb, f64::INFINITY);
    }

    #[test]
    fn test_backpropagation() {
        let mut tree = MCTSTree::new();
        let port = Port::new(PlayerId::new(), 2, 2);

        tree.init_root(port);
        tree.backpropagate(0, 100.0);

        let root = &tree.nodes[0];
        assert_eq!(root.visits, 1);
        assert_eq!(root.total_score, 100.0);
    }
}
