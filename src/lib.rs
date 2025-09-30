
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                          FICHIER: src/lib.rs                             ║
// ║  Module racine de la bibliothèque                                        ║
// ║  Rôle : Déclarer tous les modules et configurer l'application Bevy      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use bevy::prelude::*;

// === DÉCLARATION DES MODULES ===
// Chaque module est défini dans un fichier séparé pour une meilleure organisation

/// Module de configuration - Contient toutes les constantes du projet
pub mod config;

/// Module des matériaux - Gère les couleurs et apparences visuelles
pub mod materials;

/// Module de géométrie - Fonctions de création de formes et calculs mathématiques
pub mod geometry;

/// Module des systèmes - Contient la logique de setup et autres systèmes Bevy
pub mod systems;

// Import du système de setup pour l'utiliser dans la configuration
use systems::setup::setup_system;

/// Fonction principale qui configure et lance l'application Bevy
/// 
/// Cette fonction crée une nouvelle application Bevy avec :
/// - Les plugins par défaut (fenêtre, rendu, input, etc.)
/// - Le système de setup qui s'exécute au démarrage
/// 
/// L'application tourne en boucle jusqu'à ce que l'utilisateur ferme la fenêtre.
pub fn run() {
    App::new()
        // === PLUGINS BEVY ===
        // DefaultPlugins inclut tous les systèmes essentiels :
        // - WindowPlugin : gestion de la fenêtre
        // - RenderPlugin : moteur de rendu
        // - InputPlugin : clavier, souris, gamepad
        // - AssetPlugin : chargement des assets
        // - et bien d'autres...
        .add_plugins(DefaultPlugins)
        
        // === SYSTÈMES DE DÉMARRAGE ===
        // Startup : systèmes exécutés une seule fois au lancement
        // Notre système setup_system crée tous les éléments visuels
        .add_systems(Startup, setup_system)
        
        // === LANCEMENT ===
        // Démarre la boucle de jeu (game loop)
        // Cette fonction bloque jusqu'à la fermeture de l'application
        .run();
}


