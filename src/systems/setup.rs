
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                   FICHIER: src/systems/setup.rs                          ║
// ║  Système d'initialisation de la scène                                    ║
// ║  Rôle : Créer tous les éléments visuels au démarrage                    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

//! Module setup
//! 
//! Ce module contient toute la logique d'initialisation de l'application.
//! Il est responsable de la création de :
//! - La caméra 2D
//! - Le cercle principal (anneau)
//! - Les triangles extérieurs (arc-en-ciel)
//! - Les triangles intérieurs (pentagone)
//! - Les petits cercles centraux
//! - Le logo "R" personnalisé
//! 
//! Organisation du rendu par profondeur Z :
//! - Z = 0.0  : Cercle principal (arrière-plan)
//! - Z = 0.1  : Triangles extérieurs
//! - Z = 0.2  : Triangles intérieurs
//! - Z = 0.3  : Petits cercles
//! - Z = 0.4+ : Logo "R" (8 parties de 0.40 à 0.47)

use bevy::prelude::*;
use crate::{config, materials, geometry};

/// Système principal d'initialisation
/// 
/// Ce système est exécuté une seule fois au démarrage (Startup schedule).
/// Il reçoit trois ressources mutables de Bevy :
/// 
/// # Arguments
/// * `commands` - File de commandes pour créer/supprimer des entités
/// * `meshes` - Collection Asset des meshes 3D/2D
/// * `materials` - Collection Asset des matériaux (couleurs, textures)
/// 
/// # Ordre d'exécution
/// 1. Création de la caméra (sans elle, rien n'est visible)
/// 2. Création des éléments de fond vers l'avant (ordre Z croissant)
/// 3. Affichage du résumé dans la console
/// 
/// # Note technique
/// Les systèmes Bevy sont des fonctions qui reçoivent des paramètres
/// spéciaux (Commands, Query, Res, ResMut, etc.) injectés automatiquement
/// par l'ECS (Entity Component System) de Bevy.
pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // === CAMÉRA 2D ===
    // Obligatoire : sans caméra, aucun élément n'est rendu
    // Camera2d::default() crée une caméra orthographique 2D centrée
    commands.spawn(Camera2d::default());
    
    // === CRÉATION DES ÉLÉMENTS VISUELS ===
    // Ordre logique : du fond vers l'avant (mais le Z détermine l'ordre réel)
    
    create_main_circle(&mut commands, &mut meshes, &mut materials);
    create_exterior_triangles(&mut commands, &mut meshes, &mut materials);
    create_interior_triangles(&mut commands, &mut meshes, &mut materials);
    create_r_logo(&mut commands, &mut meshes, &mut materials);
    
    // === RÉSUMÉ CONSOLE ===
    print_creation_summary();
}

// ═══════════════════════════════════════════════════════════════════════════
//                  FONCTIONS DE CRÉATION DES ÉLÉMENTS
// ═══════════════════════════════════════════════════════════════════════════

/// Crée le cercle principal (anneau épais)
/// 
/// Le cercle principal est l'élément de fond du logo.
/// Il est créé comme un anneau (donut) avec un trou au centre.
/// 
/// # Paramètres utilisés
/// - CIRCLE_RADIUS : rayon extérieur
/// - CIRCLE_THICKNESS : épaisseur de l'anneau
/// - CIRCLE_SEGMENTS : qualité du rendu
/// 
/// # Position
/// - X, Y : (0, 0) - centre de l'écran
/// - Z : 0.0 - complètement en arrière-plan
fn create_main_circle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // === CALCUL DES DIMENSIONS ===
    let outer_radius = config::CIRCLE_RADIUS;
    let inner_radius = config::CIRCLE_RADIUS - config::CIRCLE_THICKNESS;
    
    // === CRÉATION DU MESH ===
    let circle_mesh = geometry::create_circle_mesh(
        outer_radius,
        inner_radius,
        config::CIRCLE_SEGMENTS
    );
    
    // === AJOUT AUX ASSETS ===
    // add() retourne un Handle<Mesh> qui référence le mesh
    let circle_handle = meshes.add(circle_mesh);
    
    // === MATÉRIAU (COULEUR) ===
    let circle_material = materials.add(materials::get_main_circle_color());
    
    // === CRÉATION DE L'ENTITÉ ===
    // Une entité est créée avec 3 composants :
    // - Mesh2d : quel mesh afficher
    // - MeshMaterial2d : quelle apparence appliquer
    // - Transform : position, rotation, échelle
    commands.spawn((
        Mesh2d(circle_handle),
        MeshMaterial2d(circle_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// Crée les triangles extérieurs en arc-en-ciel
/// 
/// Génère 36 petits triangles disposés en cercle autour du cercle principal.
/// Chaque triangle a une couleur différente créant un gradient arc-en-ciel.
/// 
/// # Caractéristiques
/// - Nombre : EXTERIOR_TRIANGLES_COUNT (36)
/// - Espacement : 10° entre chaque (360° / 36)
/// - Couleur : progression HSL de 0° à 360°
/// - Position : base du triangle sur le cercle
/// - Z : 0.1 (devant le cercle principal)
/// 
/// # Algorithme
/// Pour chaque position angulaire :
/// 1. Calculer l'angle (i × 10°)
/// 2. Calculer les 3 points du triangle
/// 3. Créer le mesh triangulaire
/// 4. Attribuer la couleur arc-en-ciel
/// 5. Spawner l'entité
fn create_exterior_triangles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // === BOUCLE SUR TOUS LES TRIANGLES ===
    for i in 0..config::EXTERIOR_TRIANGLES_COUNT {
        // === CALCUL DE L'ANGLE ===
        // Espacement régulier : 360° / nombre de triangles
        let angle_degrees = (i as f32) * 10.0;
        let base_angle = geometry::degrees_to_radians(angle_degrees);
        
        // === CALCUL DES POINTS ===
        let (p1, p2, p3) = geometry::calculate_exterior_triangle_points(
            base_angle,
            config::CIRCLE_RADIUS,
            config::SMALL_TRIANGLE_SIDE
        );
        
        // === CRÉATION DU MESH ===
        let triangle_mesh = geometry::create_triangle_from_points(p1, p2, p3);
        let triangle_handle = meshes.add(triangle_mesh);
        
        // === COULEUR ARC-EN-CIEL ===
        // Chaque triangle a une teinte différente
        let color = materials::get_rainbow_color(i);
        let triangle_material = materials.add(color);
        
        // === SPAWN ===
        commands.spawn((
            Mesh2d(triangle_handle),
            MeshMaterial2d(triangle_material),
            Transform::from_xyz(0.0, 0.0, 0.1),
        ));
    }
}

/// Crée les triangles intérieurs avec petits cercles
/// 
/// Génère 5 grands triangles formant un pentagone régulier à l'intérieur
/// du cercle. Chaque triangle a un petit cercle blanc semi-transparent
/// positionné à son centre (centroïde).
/// 
/// # Caractéristiques
/// - Nombre : INTERIOR_TRIANGLES_COUNT (5)
/// - Disposition : pentagone régulier (72° entre triangles)
/// - Orientation : un triangle pointe vers le haut (+90° offset)
/// - Couleurs : palette de 5 couleurs distinctes
/// - Cercles centraux : blancs semi-transparents (alpha 0.7)
/// 
/// # Ordre de création par triangle
/// 1. Calculer l'angle de position
/// 2. Calculer les 3 sommets du triangle
/// 3. Créer et spawner le triangle (Z=0.2)
/// 4. Calculer le centroïde
/// 5. Créer et spawner le petit cercle (Z=0.3)
fn create_interior_triangles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // === BOUCLE SUR LES 5 TRIANGLES ===
    for i in 0..config::INTERIOR_TRIANGLES_COUNT {
        // === ANGLE DE POSITION ===
        // 72° d'espacement (360° / 5) + 90° pour orientation vers le haut
        let angle_degrees = (i as f32) * 72.0 + 90.0;
        let base_angle = geometry::degrees_to_radians(angle_degrees);
        
        // === RAYON INTÉRIEUR DISPONIBLE ===
        let inner_radius = config::CIRCLE_RADIUS - config::CIRCLE_THICKNESS;
        
        // === CALCUL DES POINTS DU TRIANGLE ===
        let (p1, p2, p3) = geometry::calculate_interior_triangle_points(
            base_angle,
            inner_radius,
            config::LARGE_TRIANGLE_SIDE
        );
        
        // === CALCUL DU CENTROÏDE ===
        // Le centroïde servira de position pour le petit cercle
        let triangle_center = geometry::calculate_triangle_centroid(p1, p2, p3);
        
        // === CRÉATION DU TRIANGLE ===
        let triangle_mesh = geometry::create_triangle_from_points(p1, p2, p3);
        let triangle_handle = meshes.add(triangle_mesh);
        
        // Couleur spécifique à ce triangle
        let triangle_color = materials::get_interior_triangle_color(i);
        let triangle_material = materials.add(triangle_color);
        
        // Spawn du triangle à Z=0.2
        commands.spawn((
            Mesh2d(triangle_handle),
            MeshMaterial2d(triangle_material),
            Transform::from_xyz(0.0, 0.0, 0.2),
        ));
        
        // === CRÉATION DU PETIT CERCLE CENTRAL ===
        let small_circle_mesh = geometry::create_filled_circle_mesh(
            config::SMALL_CIRCLE_RADIUS,
            config::SMALL_CIRCLE_SEGMENTS
        );
        let small_circle_handle = meshes.add(small_circle_mesh);
        
        // Matériau blanc semi-transparent
        let small_circle_material = materials.add(materials::get_small_circle_color());
        
        // Spawn du cercle au centroïde à Z=0.3
        commands.spawn((
            Mesh2d(small_circle_handle),
            MeshMaterial2d(small_circle_material),
            Transform::from_xyz(triangle_center.x, triangle_center.y, 0.3),
        ));
    }
}

/// Crée le logo "R" complet
/// 
/// Cette fonction génère toutes les parties du logo "R" de manière unifiée.
/// Le logo est composé de 8 polygones distincts qui forment ensemble
/// la lettre "R" stylisée.
/// 
/// # Avantages de cette approche
/// - Code unifié : une seule boucle pour toutes les parties
/// - Maintenance facile : ajout/suppression de parties simple
/// - Cohérence : même couleur et même logique pour toutes les parties
/// - Debug efficace : messages descriptifs pour chaque partie
/// 
/// # Configuration
/// - Couleur : orange vif (modifiable en une ligne)
/// - Z-order : 0.40 à 0.47 (défini dans chaque RPartDefinition)
/// - Géométrie : définie dans geometry::get_all_r_parts()
/// 
/// # Process par partie
/// 1. Validation (minimum 3 points)
/// 2. Création du mesh polygonal
/// 3. Application du matériau coloré
/// 4. Spawn à la position centrale avec Z-order approprié
/// 5. Log de confirmation
fn create_r_logo(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // === COULEUR UNIQUE POUR TOUT LE LOGO ===
    // Modifier cette ligne pour changer la couleur de tout le "R"
    // Exemples :
    // - Rouge : Color::srgba(1.0, 0.0, 0.0, 0.8)
    // - Bleu : Color::srgba(0.2, 0.6, 1.0, 0.8)
    // - Vert : Color::srgba(0.2, 0.8, 0.3, 0.8)
    let r_color = Color::srgba(1.0, 0.5, 0.0, 0.8); // Orange
    
    // === PARCOURS DE TOUTES LES PARTIES ===
    for part in geometry::get_all_r_parts() {
        // === VALIDATION ===
        // Un polygone nécessite au moins 3 points
        if part.points.len() < 3 {
            println!("⚠️ '{}' ignoré : moins de 3 points", part.name);
            continue;
        }
        
        // === CRÉATION DU MESH POLYGONAL ===
        let mesh = geometry::create_polygon_from_points(&part.points);
        let mesh_handle = meshes.add(mesh);
        
        // === MATÉRIAU ===
        let material = materials.add(ColorMaterial::from(r_color));
        
        // === SPAWN DE LA PARTIE ===
        // Position : centre (0, 0)
        // Z : défini dans part.z_order pour chaque partie
        commands.spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, part.z_order),
        ));
        
        // === LOG DE CONFIRMATION ===
        println!("   ✨ '{}' créé avec {} points (Z={})", 
                 part.name, part.points.len(), part.z_order);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//                        FONCTION DE RÉSUMÉ
// ═══════════════════════════════════════════════════════════════════════════

/// Affiche un résumé détaillé de la création
/// 
/// Cette fonction est appelée à la fin du setup pour confirmer
/// que tous les éléments ont été créés correctement.
/// Elle affiche dans la console :
/// - Le nombre d'éléments de chaque type
/// - Les paramètres de configuration utilisés
/// - L'organisation des couches Z
/// 
/// Utile pour :
/// - Vérifier que l'initialisation s'est bien passée
/// - Débugger en cas de problème visuel
/// - Documenter la structure du rendu
fn print_creation_summary() {
    // Comptage dynamique des parties du logo R
    let r_parts_count = geometry::get_all_r_parts().len();
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║           CRÉATION TERMINÉE - RÉSUMÉ                     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    
    println!("\n📊 ÉLÉMENTS CRÉÉS :");
    println!("   ✓ 1 cercle principal (anneau épais)");
    println!("   ✓ {} triangles extérieurs arc-en-ciel", config::EXTERIOR_TRIANGLES_COUNT);
    println!("   ✓ {} triangles intérieurs colorés", config::INTERIOR_TRIANGLES_COUNT);
    println!("   ✓ {} petits cercles centraux", config::INTERIOR_TRIANGLES_COUNT);
    println!("   ✓ {} parties du logo 'R'", r_parts_count);
    
    println!("\n🔧 PARAMÈTRES DE CONFIGURATION :");
    println!("   • Rayon principal : {} px", config::CIRCLE_RADIUS);
    println!("   • Épaisseur anneau : {} px", config::CIRCLE_THICKNESS);
    println!("   • Rayon intérieur : {} px", config::CIRCLE_RADIUS - config::CIRCLE_THICKNESS);
    println!("   • Qualité cercles : {} segments", config::CIRCLE_SEGMENTS);
    println!("   • Taille triangles extérieurs : {} px", config::SMALL_TRIANGLE_SIDE);
    println!("   • Taille triangles intérieurs : {} px", config::LARGE_TRIANGLE_SIDE);
    
    println!("\n🎬 ORGANISATION DES COUCHES (Z) :");
    println!("   • Z = 0.0  : Cercle principal (arrière-plan)");
    println!("   • Z = 0.1  : Triangles extérieurs arc-en-ciel");
    println!("   • Z = 0.2  : Triangles intérieurs colorés");
    println!("   • Z = 0.3  : Petits cercles blancs");
    println!("   • Z = 0.4+ : Logo 'R' ({} parties)", r_parts_count);
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  🚀 Application Bevy prête - Fenêtre ouverte             ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
}