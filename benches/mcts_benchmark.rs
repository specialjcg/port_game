// MCTS Performance Benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use port_game::domain::aggregates::Port;
use port_game::domain::value_objects::PlayerId;
use port_game::mcts::{MCTSConfig, MCTSEngine};

fn benchmark_mcts_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("mcts_search");

    // Test different simulation counts
    for num_sims in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_sims),
            num_sims,
            |b, &num_sims| {
                let config = MCTSConfig {
                    num_simulations: num_sims,
                    exploration_constant: 1.41,
                    max_depth: 20,
                    max_actions_per_turn: 3,
                };
                let mut engine = MCTSEngine::new(config);
                let port = create_test_port();

                b.iter(|| engine.search(black_box(&port)));
            },
        );
    }

    group.finish();
}

fn benchmark_mcts_with_ships(c: &mut Criterion) {
    let mut group = c.benchmark_group("mcts_with_ships");

    // Test with different numbers of ships
    for num_ships in [1, 3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_ships),
            num_ships,
            |b, &num_ships| {
                let config = MCTSConfig {
                    num_simulations: 100,
                    exploration_constant: 1.41,
                    max_depth: 20,
                    max_actions_per_turn: 3,
                };
                let mut engine = MCTSEngine::new(config);
                let port = create_port_with_ships(num_ships);

                b.iter(|| engine.search(black_box(&port)));
            },
        );
    }

    group.finish();
}

fn benchmark_tree_expansion(c: &mut Criterion) {
    c.bench_function("tree_expansion", |b| {
        let mut tree = port_game::mcts::MCTSTree::new();
        let port = create_test_port();
        tree.init_root(port);

        b.iter(|| tree.expand(black_box(0), 20));
    });
}

fn benchmark_ucb1_calculation(c: &mut Criterion) {
    c.bench_function("ucb1_calculation", |b| {
        let mut tree = port_game::mcts::MCTSTree::new();
        let port = create_test_port();
        tree.init_root(port);
        tree.expand(0, 20);

        b.iter(|| tree.select_ucb1(black_box(1.41)));
    });
}

// Helper functions
fn create_test_port() -> Port {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);

    // Add some ships for realistic testing
    use port_game::domain::entities::Ship;
    use port_game::domain::value_objects::ShipId;

    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships
        .insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 0.0));

    port
}

fn create_port_with_ships(num_ships: usize) -> Port {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 3, 3);

    use port_game::domain::entities::Ship;
    use port_game::domain::value_objects::ShipId;

    for i in 0..num_ships {
        port.ships.insert(
            ShipId::new(i),
            Ship::new(ShipId::new(i), 20 + (i as u32 * 10), i as f64),
        );
    }

    port
}

criterion_group!(
    benches,
    benchmark_mcts_search,
    benchmark_mcts_with_ships,
    benchmark_tree_expansion,
    benchmark_ucb1_calculation
);
criterion_main!(benches);
