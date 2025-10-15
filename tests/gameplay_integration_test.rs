//! Tests d'intégration exhaustifs du gameplay du Port Game
use port_game::domain::value_objects::{BerthId, CraneId, PlayerId};
use port_game::game::{GameMode, GameSession};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_gameplay_scenario() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // Spawn ships and dock them
        session.spawn_ships(2);

        // Récupérer tous les IDs nécessaires d'abord
        let berth_id = *session.player_port.berths.iter().next().unwrap().0;
        let ship_ids: Vec<_> = session
            .player_port
            .ships
            .iter()
            .map(|(id, _)| *id)
            .collect();
        let crane_id = *session.player_port.cranes.iter().next().unwrap().0;

        // Tests d'amarrage
        assert!(
            session.player_dock_ship(ship_ids[0], berth_id).is_ok(),
            "Should dock ship to berth"
        );
        assert!(
            session.player_dock_ship(ship_ids[1], berth_id).is_err(),
            "Should not dock to occupied berth"
        );

        // Tests d'assignation de grue
        assert!(
            session.player_assign_crane(crane_id, ship_ids[0]).is_ok(),
            "Should assign free crane"
        );
        assert!(
            session.player_assign_crane(crane_id, ship_ids[1]).is_err(),
            "Should not assign busy crane"
        );

        // Process containers
        session.start_turn();
        session.process_containers();

        // Vérifier le traitement des containers
        let ship = session
            .player_port
            .ships
            .iter()
            .find(|(id, _)| **id == ship_ids[0])
            .map(|(_, ship)| ship)
            .unwrap();
        assert!(
            ship.containers_remaining < ship.containers,
            "Containers should be processed"
        );
    }

    #[test]
    fn test_error_cases() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        session.spawn_ships(1);

        let ship_id = *session.player_port.ships.iter().next().unwrap().0;

        // Test avec un berth inexistant
        let fake_berth = BerthId::new(9999);
        assert!(
            session.player_dock_ship(ship_id, fake_berth).is_err(),
            "Docking to non-existent berth should fail"
        );

        // Test avec une grue inexistante
        let fake_crane = CraneId::new(9999);
        assert!(
            session.player_assign_crane(fake_crane, ship_id).is_err(),
            "Assigning non-existent crane should fail"
        );
    }

    #[test]
    fn test_game_end_conditions() {
        // Configuration initiale
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        println!("Test démarré");

        // Collecter les IDs des installations portuaires
        let berth_ids: Vec<_> = session
            .player_port
            .berths
            .iter()
            .map(|(id, _)| *id)
            .collect();
        let crane_ids: Vec<_> = session
            .player_port
            .cranes
            .iter()
            .map(|(id, _)| *id)
            .collect();

        // Ajouter les premiers navires
        session.spawn_ships(5);
        println!(
            "Navires initiaux créés: {}",
            session.player_port.ships.len()
        );

        // Simulation du jeu
        for turn in 0..30 {
            println!("\n=== Tour {} ===", turn);
            session.start_turn();

            // 1. Amarrer les navires disponibles
            let available_ships: Vec<_> = session
                .player_port
                .ships
                .iter()
                .filter(|(_, ship)| !ship.is_docked())
                .map(|(id, _)| *id)
                .collect();

            println!("Navires en attente d'amarrage: {}", available_ships.len());

            for ship_id in available_ships {
                // Essayer d'amarrer à un quai libre
                for berth_id in &berth_ids {
                    if session.player_dock_ship(ship_id, *berth_id).is_ok() {
                        println!("Navire {} amarré au quai {}", ship_id.0, berth_id.0);

                        // Assigner une grue disponible
                        for crane_id in &crane_ids {
                            if session.player_assign_crane(*crane_id, ship_id).is_ok() {
                                println!("Grue {} assignée au navire {}", crane_id.0, ship_id.0);
                                break;
                            }
                        }
                        break;
                    }
                }
            }

            // 2. Traiter les conteneurs
            for _ in 0..3 {
                // Traiter plusieurs fois par tour
                session.process_containers();
            }

            // 3. Libérer les navires terminés et leurs grues
            let completed_ships: Vec<_> = session
                .player_port
                .ships
                .iter()
                .filter(|(_, ship)| ship.is_docked() && ship.containers_remaining == 0)
                .map(|(id, ship)| (*id, ship.docked_at.unwrap(), ship.assigned_cranes.clone()))
                .collect();

            for (ship_id, berth_id, crane_ids) in completed_ships {
                // D'abord libérer les grues
                for crane_id in crane_ids {
                    println!(
                        "Désassignation de la grue {} du navire {}",
                        crane_id.0, ship_id.0
                    );
                    session.player_port.free_crane(crane_id);
                }
                // Ensuite libérer le navire
                println!("Libération du navire {} terminé", ship_id.0);
                session.player_port.undock_ship(ship_id, berth_id);
            }

            // 4. Réassigner les grues libérées aux navires en attente de traitement
            let docked_ships: Vec<_> = session
                .player_port
                .ships
                .iter()
                .filter(|(_, ship)| {
                    ship.is_docked()
                        && ship.containers_remaining > 0
                        && ship.assigned_cranes.is_empty()
                })
                .map(|(id, _)| *id)
                .collect();

            for ship_id in docked_ships {
                for crane_id in &crane_ids {
                    if session.player_assign_crane(*crane_id, ship_id).is_ok() {
                        println!(
                            "Réassignation de la grue {} au navire {}",
                            crane_id.0, ship_id.0
                        );
                        break;
                    }
                }
            }

            // Afficher l'état
            println!("Score: {}", session.player_port.score);
            println!(
                "Navires en attente: {}",
                session.player_port.waiting_ships().len()
            );
            println!(
                "Navires amarrés: {}",
                session.player_port.docked_ships().len()
            );

            // Vérifier les conditions de fin
            if session.is_game_over() {
                println!("\nPartie terminée au tour {} !", turn);
                println!("Score final: {}", session.player_port.score);
                assert!(turn < 30, "La partie doit se terminer avant le tour 30");
                return;
            }

            // Ajouter de nouveaux navires périodiquement
            if turn % 2 == 0 {
                session.spawn_ships(2);
                println!("+ 2 nouveaux navires ajoutés");
            }
        }

        panic!("La partie n'a pas terminé dans les 30 tours");
    }

    #[test]
    fn test_end_turn_sequence() {
        // Configuration initiale
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // Préparation du scénario
        session.spawn_ships(2);
        let berth_id = *session.player_port.berths.iter().next().unwrap().0;
        let ship_ids: Vec<_> = session
            .player_port
            .ships
            .iter()
            .map(|(id, _)| *id)
            .collect();
        let crane_id = *session.player_port.cranes.iter().next().unwrap().0;

        // Amarrer un navire et assigner une grue
        session
            .player_dock_ship(ship_ids[0], berth_id)
            .expect("Le navire devrait s'amarrer");
        session
            .player_assign_crane(crane_id, ship_ids[0])
            .expect("La grue devrait être assignée");

        // Simulation du traitement des conteneurs jusqu'à ce que le navire soit vide
        while session
            .player_port
            .ships
            .get(&ship_ids[0])
            .unwrap()
            .containers_remaining
            > 0
        {
            session.process_containers();
        }

        // Vérification avant la fin du tour
        let ship_before = session.player_port.ships.get(&ship_ids[0]).unwrap().clone();
        let crane_before = session.player_port.cranes.get(&crane_id).unwrap().clone();
        assert_eq!(
            ship_before.containers_remaining, 0,
            "Le navire devrait être vide"
        );
        assert!(
            crane_before.assigned_to.is_some(),
            "La grue devrait être assignée"
        );

        let turn_before = session.current_turn;

        // Exécution de la fin de tour
        session.end_turn();

        // Vérifications après la fin du tour
        assert!(
            !session.player_port.ships.contains_key(&ship_ids[0]),
            "Le navire terminé devrait être libéré"
        );

        let crane_after = session.player_port.cranes.get(&crane_id).unwrap();
        assert!(
            crane_after.assigned_to.is_none(),
            "La grue devrait être libérée"
        );

        // Vérification que le nouveau tour a commencé
        assert!(
            session.current_turn > turn_before,
            "Le compteur de tour devrait être incrémenté"
        );
    }

    #[test]
    fn test_crane_freed_and_reassignable_after_end_turn() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // Ajouter deux navires
        session.spawn_ships(2);
        let berth_id = *session.player_port.berths.iter().next().unwrap().0;
        let ship_ids: Vec<_> = session
            .player_port
            .ships
            .iter()
            .map(|(id, _)| *id)
            .collect();
        let crane_id = *session.player_port.cranes.iter().next().unwrap().0;

        // Premier tour : dock et assigne la grue au premier navire
        assert!(session.player_dock_ship(ship_ids[0], berth_id).is_ok());
        assert!(session.player_assign_crane(crane_id, ship_ids[0]).is_ok());

        // Vider complètement le navire
        {
            let ship = session.player_port.ships.get_mut(&ship_ids[0]).unwrap();
            ship.containers_remaining = 0;
        }

        // Terminer le tour devrait libérer la grue car le navire est vide
        session.end_turn();

        // Vérifie que la grue est libre après que le navire soit complété
        let crane = session.player_port.cranes.get(&crane_id).unwrap();
        assert!(
            crane.is_free(),
            "La grue doit être libre après avoir complété un navire"
        );

        // Deuxième tour : dock et assigne la même grue au second navire
        assert!(session.player_dock_ship(ship_ids[1], berth_id).is_ok());
        assert!(session.player_assign_crane(crane_id, ship_ids[1]).is_ok());

        // La grue devrait rester assignée car le navire n'est pas vide
        session.end_turn();

        // Vérifie que la grue reste assignée
        let crane = session.player_port.cranes.get(&crane_id).unwrap();
        assert!(
            !crane.is_free(),
            "La grue doit rester assignée au navire non complété"
        );
        assert_eq!(
            crane.assigned_to,
            Some(ship_ids[1]),
            "La grue doit être assignée au second navire"
        );
    }

    #[test]
    fn test_berth_freed_after_ship_undock() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // Spawn and dock a ship
        session.spawn_ships(1);
        let berth_id = *session.player_port.berths.iter().next().unwrap().0;
        let ship_id = *session.player_port.ships.iter().next().unwrap().0;
        assert!(
            session.player_dock_ship(ship_id, berth_id).is_ok(),
            "Should dock ship"
        );

        // Vider tous les containers du navire
        {
            let ship = session.player_port.ships.get_mut(&ship_id).unwrap();
            ship.containers_remaining = 0;
        }

        // Undock le navire
        session.player_port.undock_ship(ship_id, berth_id);

        // Le berth doit être libre !
        let berth = session.player_port.berths.get(&berth_id).unwrap();
        assert!(
            berth.is_free(),
            "Berth should be free after undocking ship with 0 containers"
        );
    }

    #[test]
    fn test_berth_freed_after_auto_undock_on_empty_ship() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // Spawn and dock a ship
        session.spawn_ships(1);
        let berth_id = *session.player_port.berths.iter().next().unwrap().0;
        let ship_id = *session.player_port.ships.iter().next().unwrap().0;
        assert!(
            session.player_dock_ship(ship_id, berth_id).is_ok(),
            "Should dock ship"
        );

        // Assigner une grue pour traiter les containers
        let crane_id = *session.player_port.cranes.iter().next().unwrap().0;
        assert!(
            session.player_assign_crane(crane_id, ship_id).is_ok(),
            "Should assign crane"
        );

        // Boucle de traitement jusqu'à ce que le navire soit vide
        while session
            .player_port
            .ships
            .get(&ship_id)
            .unwrap()
            .containers_remaining
            > 0
        {
            session.process_containers();
        }

        // Simuler la logique d'auto-undock (comme dans la boucle de fin de tour)
        let ship = session.player_port.ships.get(&ship_id).unwrap();
        if ship.is_docked() && ship.containers_remaining == 0 {
            let berth_id = ship.docked_at.unwrap();
            session.player_port.undock_ship(ship_id, berth_id);
        }

        // Le berth doit être libre !
        let berth = session.player_port.berths.get(&berth_id).unwrap();
        assert!(
            berth.is_free(),
            "Berth should be free after auto undock of empty ship"
        );
    }
}
