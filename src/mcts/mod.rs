// Monte Carlo Tree Search engine
// Core AI for the game

pub mod actions;
pub mod simulation;
pub mod tree;

use crate::domain::aggregates::Port;
use crate::utils::random;

pub use actions::MCTSAction;
pub use tree::{MCTSNode, MCTSTree};

/// MCTS engine configuration
#[derive(Debug, Clone)]
pub struct MCTSConfig {
    pub num_simulations: usize,
    pub exploration_constant: f64, // UCB1 constant (√2 is standard)
    pub max_depth: usize,
    pub max_actions_per_turn: usize,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            num_simulations: 1000,
            exploration_constant: 1.41, // √2
            max_depth: 50,
            max_actions_per_turn: 1,
        }
    }
}

/// Main MCTS engine
pub struct MCTSEngine {
    config: MCTSConfig,
    tree: MCTSTree,
}

impl MCTSEngine {
    pub fn new(config: MCTSConfig) -> Self {
        Self {
            config,
            tree: MCTSTree::new(),
        }
    }

    /// Run MCTS search and return best action
    pub fn search(&mut self, port: &Port) -> Option<MCTSAction> {
        // Initialize root node with current state
        self.tree.init_root(port.clone());

        for _ in 0..self.config.num_simulations {
            // 1. Selection: traverse tree using UCB1
            let node_id = self.select();

            // 2. Expansion: add child nodes for unexplored actions
            let expand_id = self.expand(node_id, port);

            // 3. Simulation: play out randomly to get a score
            let score = self.simulate(expand_id);

            // 4. Backpropagation: update node statistics
            self.backpropagate(expand_id, score);
        }

        // Return best action from root
        self.tree.best_action()
    }

    fn select(&self) -> usize {
        self.tree.select_ucb1(self.config.exploration_constant)
    }

    fn expand(&mut self, node_id: usize, _port: &Port) -> usize {
        self.tree.expand(node_id, self.config.max_depth)
    }

    fn simulate(&self, node_id: usize) -> f64 {
        // Random playout simulation with depth limit
        let mut simulated_state = self.tree.get_state(node_id).clone();
        let mut depth = self.tree.node_depth(node_id);

        while depth < self.config.max_depth {
            let actions = self.tree.generate_actions(&simulated_state);
            if actions.is_empty() {
                break;
            }

            let action_index = random::range_usize(0, actions.len());
            if let Some(action) = actions.get(action_index).cloned() {
                MCTSTree::apply_action_to_state(&mut simulated_state, &action);
            } else {
                break;
            }

            depth += 1;
        }

        let mut score = simulated_state.calculate_score() as f64;
        score += random::range_f64(-5.0, 5.0);
        score
    }

    fn backpropagate(&mut self, node_id: usize, score: f64) {
        self.tree.backpropagate(node_id, score);
    }

    pub fn get_tree(&self) -> &MCTSTree {
        &self.tree
    }

    pub fn config(&self) -> &MCTSConfig {
        &self.config
    }

    pub fn get_statistics(&self) -> MCTSStatistics {
        MCTSStatistics {
            simulations_performed: self.config.num_simulations,
            total_nodes: self.tree.node_count(),
            max_depth_reached: self.tree.max_depth(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MCTSStatistics {
    pub simulations_performed: usize,
    pub total_nodes: usize,
    pub max_depth_reached: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::PlayerId;

    #[test]
    fn test_mcts_engine_creation() {
        let config = MCTSConfig::default();
        let engine = MCTSEngine::new(config);

        assert_eq!(engine.config.num_simulations, 1000);
    }

    #[test]
    fn test_mcts_search_simple() {
        let config = MCTSConfig {
            num_simulations: 10, // Small for testing
            exploration_constant: 1.41,
            max_depth: 10,
            max_actions_per_turn: 2,
        };

        let mut engine = MCTSEngine::new(config);
        let port = Port::new(PlayerId::new(), 2, 2);

        // Should not crash even with empty port
        let _action = engine.search(&port);
    }
}
