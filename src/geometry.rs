
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                      FICHIER: src/geometry.rs                            ║
// ║  Fonctions de géométrie et création de mesh                              ║
// ║  Rôle : Générer les formes géométriques et calculer leurs positions     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

//! Module de géométrie
//! 
//! Ce module contient toutes les fonctions mathématiques et de génération
//! de formes géométriques. Il est divisé en plusieurs sections :
//! 
//! 1. Fonctions utilitaires (conversion d'angles)
//! 2. Création de mesh basiques (cercles, triangles)
//! 3. Calculs de positions pour les triangles extérieurs/intérieurs
//! 4. Définition du logo "R" personnalisé

use bevy::prelude::*;
use std::f32::consts::PI;

// ═══════════════════════════════════════════════════════════════════════════
//                        SECTION 1 : FONCTIONS UTILITAIRES
// ═══════════════════════════════════════════════════════════════════════════

/// Convertit un angle en degrés vers des radians
/// 
/// Les fonctions trigonométriques de Rust (sin, cos, tan) utilisent
/// les radians. Cette fonction permet d'utiliser des degrés (plus intuitifs).
/// 
/// # Formule
/// radians = degrés × π / 180
/// 
/// # Exemples
/// - 0° → 0 rad
/// - 90° → π/2 rad (≈1.57)
/// - 180° → π rad (≈3.14)
/// - 360° → 2π rad (≈6.28)
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

// ═══════════════════════════════════════════════════════════════════════════
//                      SECTION 2 : CRÉATION DE MESH BASIQUES
// ═══════════════════════════════════════════════════════════════════════════

/// Crée un mesh d'anneau (cercle avec un trou au centre)
/// 
/// Un anneau est créé en générant deux cercles concentriques
/// (extérieur et intérieur) puis en les reliant avec des triangles.
/// 
/// # Algorithme
/// 1. Générer les vertices du cercle extérieur
/// 2. Générer les vertices du cercle intérieur
/// 3. Créer des quadrilatères entre les deux cercles
/// 4. Diviser chaque quadrilatère en 2 triangles
/// 
/// # Arguments
/// * `outer_radius` - Rayon du cercle extérieur
/// * `inner_radius` - Rayon du cercle intérieur
/// * `segments` - Nombre de segments (qualité du cercle)
/// 
/// # Détails techniques
/// Pour N segments, on génère :
/// - 2N vertices (N extérieurs + N intérieurs)
/// - 2N triangles (2 triangles par segment)
/// - 6N indices (3 indices par triangle)
pub fn create_circle_mesh(outer_radius: f32, inner_radius: f32, segments: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // === GÉNÉRATION DES VERTICES DU CERCLE EXTÉRIEUR ===
    // On parcourt l'angle de 0 à 2π pour faire le tour complet
    for i in 0..segments {
        // Angle du segment actuel
        let angle = 2.0 * PI * i as f32 / segments as f32;
        
        // Position du vertex sur le cercle extérieur
        // x = rayon × cos(angle), y = rayon × sin(angle)
        positions.push([
            outer_radius * angle.cos(),
            outer_radius * angle.sin(),
            0.0,  // Z=0 car on travaille en 2D
        ]);
    }

    // === GÉNÉRATION DES VERTICES DU CERCLE INTÉRIEUR ===
    // Même principe mais avec le rayon intérieur
    for i in 0..segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        positions.push([
            inner_radius * angle.cos(),
            inner_radius * angle.sin(),
            0.0,
        ]);
    }

    // === CRÉATION DES TRIANGLES ===
    // Pour chaque segment, on crée un quadrilatère puis on le divise en 2 triangles
    for i in 0..segments {
        // Index du prochain segment (avec retour au début)
        let next = (i + 1) % segments;
        
        // Triangle 1 : coin inférieur gauche du quadrilatère
        // Vertices : extérieur_i, intérieur_i, extérieur_next
        indices.push(i as u32);
        indices.push((segments + i) as u32);
        indices.push(next as u32);
        
        // Triangle 2 : coin supérieur droit du quadrilatère
        // Vertices : extérieur_next, intérieur_i, intérieur_next
        indices.push(next as u32);
        indices.push((segments + i) as u32);
        indices.push((segments + next) as u32);
    }

    // === CRÉATION DU MESH BEVY ===
    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

/// Crée un mesh de cercle plein
/// 
/// Génère un cercle solide (disque) en créant des triangles
/// depuis le centre vers chaque point du contour.
/// 
/// # Algorithme
/// 1. Placer un vertex au centre (0, 0)
/// 2. Générer N vertices sur le contour
/// 3. Créer N triangles reliant le centre à chaque paire de vertices adjacents
/// 
/// # Arguments
/// * `radius` - Rayon du cercle
/// * `segments` - Nombre de segments du contour
pub fn create_filled_circle_mesh(radius: f32, segments: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // === VERTEX CENTRAL ===
    // Premier vertex au centre du cercle
    positions.push([0.0, 0.0, 0.0]);

    // === VERTICES DU CONTOUR ===
    // Génération des points sur le périmètre
    for i in 0..segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        positions.push([
            radius * angle.cos(),
            radius * angle.sin(),
            0.0,
        ]);
    }

    // === TRIANGLES EN ÉVENTAIL ===
    // Chaque triangle relie le centre à deux vertices adjacents du contour
    for i in 0..segments {
        // Calcul de l'index suivant (retour au début pour le dernier)
        let next = if i == segments - 1 { 1 } else { i + 2 };
        
        // Triangle : centre, vertex_i, vertex_suivant
        indices.push(0);              // Centre (index 0)
        indices.push((i + 1) as u32); // Vertex actuel
        indices.push(next as u32);    // Vertex suivant
    }

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

/// Crée un triangle à partir de trois points 2D
/// 
/// Fonction simple qui convertit 3 points Vec2 en un mesh triangulaire.
/// 
/// # Arguments
/// * `p1`, `p2`, `p3` - Les trois sommets du triangle
/// 
/// # Note
/// L'ordre des points définit l'orientation du triangle (sens horaire/antihoraire)
/// ce qui affecte la face visible (culling).
pub fn create_triangle_from_points(p1: Vec2, p2: Vec2, p3: Vec2) -> Mesh {
    // Conversion des points 2D en positions 3D (Z=0)
    let positions = vec![
        [p1.x, p1.y, 0.0],
        [p2.x, p2.y, 0.0],
        [p3.x, p3.y, 0.0],
    ];

    // Indices des 3 sommets dans l'ordre
    let indices = vec![0u32, 1, 2];

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

/// Crée un polygone complexe à partir d'une liste de points
/// 
/// Utilise une triangulation en éventail (fan triangulation) :
/// - Tous les triangles partagent le premier vertex
/// - Chaque triangle relie le premier vertex à deux vertices consécutifs
/// 
/// # Limitations
/// Cette méthode fonctionne bien pour les polygones convexes.
/// Pour les polygones concaves complexes, une triangulation plus
/// sophistiquée (comme l'algorithme ear-clipping) serait nécessaire.
/// 
/// # Arguments
/// * `points` - Slice de Vec2 représentant les sommets du polygone
/// 
/// # Panic
/// Panic si moins de 3 points sont fournis
pub fn create_polygon_from_points(points: &[Vec2]) -> Mesh {
    if points.len() < 3 {
        panic!("Un polygone doit avoir au moins 3 points");
    }
    
    // === CONVERSION DES POINTS EN POSITIONS 3D ===
    let positions: Vec<[f32; 3]> = points
        .iter()
        .map(|p| [p.x, p.y, 0.0])
        .collect();
    
    // === TRIANGULATION EN ÉVENTAIL ===
    // Pour N points : créer N-2 triangles
    // Triangle i relie les vertices 0, i, i+1
    let mut indices = Vec::new();
    for i in 1..(points.len() - 1) {
        indices.push(0u32);           // Premier vertex (pivot)
        indices.push(i as u32);       // Vertex actuel
        indices.push((i + 1) as u32); // Vertex suivant
    }
    
    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

// ═══════════════════════════════════════════════════════════════════════════
//            SECTION 3 : CALCULS DE POSITIONS DES TRIANGLES
// ═══════════════════════════════════════════════════════════════════════════

/// Calcule les coordonnées d'un triangle extérieur
/// 
/// Les triangles extérieurs sont positionnés de manière à ce qu'un
/// de leurs côtés (la base) touche le cercle principal.
/// 
/// # Stratégie de positionnement
/// 1. Calculer deux points sur le cercle (extrémités de la base)
/// 2. Calculer le troisième point à l'extérieur du cercle
/// 
/// # Arguments
/// * `base_angle` - Angle central du triangle en radians
/// * `circle_radius` - Rayon du cercle de référence
/// * `triangle_side` - Longueur d'un côté du triangle équilatéral
/// 
/// # Géométrie
/// Pour un triangle équilatéral de côté L :
/// - Hauteur : h = L × √3 / 2
/// - Les deux points de base sont espacés angulairement de L/R radians
///   où R est le rayon du cercle
pub fn calculate_exterior_triangle_points(
    base_angle: f32,
    circle_radius: f32,
    triangle_side: f32,
) -> (Vec2, Vec2, Vec2) {
    let half_side = triangle_side / 2.0;
    let height = triangle_side * (3.0_f32.sqrt() / 2.0);
    
    // === CALCUL DES DEUX POINTS DE BASE SUR LE CERCLE ===
    // L'écart angulaire est approximé par half_side / circle_radius
    let angle1 = base_angle - (half_side / circle_radius);
    let angle2 = base_angle + (half_side / circle_radius);
    
    // Point 1 : sur le cercle, à gauche du centre
    let p1 = Vec2::new(
        circle_radius * angle1.cos(),
        circle_radius * angle1.sin(),
    );
    
    // Point 2 : sur le cercle, à droite du centre
    let p2 = Vec2::new(
        circle_radius * angle2.cos(),
        circle_radius * angle2.sin(),
    );
    
    // === CALCUL DU SOMMET EXTÉRIEUR ===
    // Positionné à une distance de (rayon + hauteur) du centre
    let p3 = Vec2::new(
        (circle_radius + height) * base_angle.cos(),
        (circle_radius + height) * base_angle.sin(),
    );
    
    (p1, p2, p3)
}

/// Calcule les coordonnées d'un triangle intérieur
/// 
/// Similaire à calculate_exterior_triangle_points mais :
/// - La base touche le cercle intérieur
/// - Le sommet pointe vers le centre (intérieur)
/// 
/// # Arguments
/// * `base_angle` - Angle central du triangle en radians
/// * `inner_radius` - Rayon du cercle intérieur
/// * `triangle_side` - Longueur d'un côté du triangle
pub fn calculate_interior_triangle_points(
    base_angle: f32,
    inner_radius: f32,
    triangle_side: f32,
) -> (Vec2, Vec2, Vec2) {
    let half_side = triangle_side / 2.0;
    let height = triangle_side * (3.0_f32.sqrt() / 2.0);
    
    // === POINTS DE BASE SUR LE CERCLE INTÉRIEUR ===
    let angle1 = base_angle - (half_side / inner_radius);
    let angle2 = base_angle + (half_side / inner_radius);
    
    let p1 = Vec2::new(
        inner_radius * angle1.cos(),
        inner_radius * angle1.sin(),
    );
    let p2 = Vec2::new(
        inner_radius * angle2.cos(),
        inner_radius * angle2.sin(),
    );
    
    // === SOMMET VERS L'INTÉRIEUR ===
    // Distance : rayon - hauteur (pour pointer vers le centre)
    let p3 = Vec2::new(
        (inner_radius - height) * base_angle.cos(),
        (inner_radius - height) * base_angle.sin(),
    );
    
    (p1, p2, p3)
}

/// Calcule le centroïde (centre de gravité) d'un triangle
/// 
/// Le centroïde est le point d'intersection des médianes du triangle.
/// Pour un triangle, c'est simplement la moyenne des coordonnées
/// de ses trois sommets.
/// 
/// # Formule
/// Centroïde = ((x1+x2+x3)/3, (y1+y2+y3)/3)
/// 
/// # Arguments
/// * `p1`, `p2`, `p3` - Les trois sommets du triangle
/// 
/// # Propriété
/// Le centroïde est équidistant des trois côtés du triangle
pub fn calculate_triangle_centroid(p1: Vec2, p2: Vec2, p3: Vec2) -> Vec2 {
    Vec2::new(
        (p1.x + p2.x + p3.x) / 3.0,
        (p1.y + p2.y + p3.y) / 3.0,
    )
}

// ═══════════════════════════════════════════════════════════════════════════
//                    SECTION 4 : DÉFINITION DU LOGO "R"
// ═══════════════════════════════════════════════════════════════════════════

/// Structure représentant une partie du logo "R"
/// 
/// Chaque partie du logo est définie par :
/// - Un nom descriptif pour l'identification
/// - Une liste de points formant le polygone
/// - Un ordre de profondeur Z pour le rendu en couches
#[derive(Clone)]
pub struct RPartDefinition {
    /// Nom descriptif de la partie (pour debug et logs)
    pub name: &'static str,
    
    /// Liste ordonnée des sommets du polygone
    /// Les points doivent former un contour dans l'ordre
    pub points: Vec<Vec2>,
    
    /// Ordre de rendu sur l'axe Z (plus élevé = devant)
    /// Recommandation : utiliser des valeurs entre 0.40 et 0.50
    /// pour être devant les triangles intérieurs (Z=0.3)
    pub z_order: f32,
}

/// Retourne toutes les parties composant le logo "R"
/// 
/// Le logo "R" est décomposé en 8 parties géométriques distinctes :
/// 1. Haut : barre horizontale supérieure
/// 2. Gauche : barre verticale principale
/// 3. Arrondi : courbe de la partie droite supérieure
/// 4. Centre : petite barre de séparation
/// 5. Pied gauche : base élargie à gauche
/// 6. Milieu : zone de connexion centrale
/// 7. Jambe droite : diagonale partant vers la droite
/// 8. Pied droit : base élargie à droite
/// 
/// # Organisation
/// Chaque partie a un z_order incrémental (0.40 à 0.47) pour
/// s'assurer qu'elles se superposent correctement sans conflit visuel.
/// 
/// # Coordonnées
/// Les coordonnées sont définies dans un système où :
/// - (0, 0) est au centre de l'écran
/// - X positif va vers la droite
/// - Y positif va vers le haut
/// 
/// # Modification
/// Pour ajuster le logo :
/// 1. Modifier les Vec2::new(x, y) de chaque partie
/// 2. Pour ajouter une partie : ajouter un nouveau RPartDefinition
/// 3. Pour supprimer une partie : commenter ou retirer le bloc
pub fn get_all_r_parts() -> Vec<RPartDefinition> {
    vec![
        // === PARTIE 1 : BARRE HORIZONTALE SUPÉRIEURE ===
        RPartDefinition {
            name: "Haut du R",
            z_order: 0.40,
            points: vec![
                Vec2::new(-140.0, 90.0),  // Coin supérieur gauche
                Vec2::new(60.0, 90.0),    // Coin supérieur droit
                Vec2::new(60.0, 50.0),    // Coin inférieur droit
                Vec2::new(-100.0, 50.0),  // Coin inférieur gauche
            ],
        },
        
        // === PARTIE 2 : BARRE VERTICALE PRINCIPALE GAUCHE ===
        RPartDefinition {
            name: "Gauche du R",
            z_order: 0.41,
            points: vec![
                Vec2::new(-80.0, 50.0),   // Haut de la barre
                Vec2::new(-30.0, 50.0),   // Haut droit
                Vec2::new(-30.0, -50.0),  // Bas droit
                Vec2::new(-80.0, -50.0),  // Bas gauche
            ],
        },
        
        // === PARTIE 3 : COURBE ARRONDIE DROITE ===
        // Points formant l'arrondi caractéristique du "R"
        RPartDefinition {
            name: "Arrondi du R",
            z_order: 0.42,
            points: vec![
                Vec2::new(60.0, 90.0),    // Départ en haut
                Vec2::new(85.0, 60.0),    // Premier point de courbe
                Vec2::new(100.0, 30.0),   // Point le plus à droite
                Vec2::new(85.0, 0.0),     // Descente de la courbe
                Vec2::new(60.0, -30.0),   // Fin de l'arrondi
            ],
        },
        
        // === PARTIE 4 : SÉPARATEUR CENTRAL ===
        // Petite barre horizontale au milieu
        RPartDefinition {
            name: "Centre du R",
            z_order: 0.43,
            points: vec![
                Vec2::new(60.0, 50.0),    // Haut gauche
                Vec2::new(40.0, 50.0),    // Haut droit
                Vec2::new(60.0, 10.0),    // Bas gauche
                Vec2::new(40.0, 10.0),    // Bas droit
            ],
        },
        
        // === PARTIE 5 : PIED GAUCHE ÉLARGI ===
        RPartDefinition {
            name: "Pied gauche du R",
            z_order: 0.44,
            points: vec![
                Vec2::new(-80.0, -50.0),   // Connexion avec barre verticale
                Vec2::new(-10.0, -50.0),   // Vers le centre
                Vec2::new(-10.0, -80.0),   // Descente
                Vec2::new(-140.0, -80.0),  // Largeur maximale du pied
                Vec2::new(-160.0, -50.0),  // Retour vers le haut
                Vec2::new(-80.0, -50.0),   // Fermeture
            ],
        },
        
        // === PARTIE 6 : ZONE DE CONNEXION CENTRALE ===
        RPartDefinition {
            name: "Milieu du R",
            z_order: 0.45,
            points: vec![
                Vec2::new(60.0, -30.0),   // Connexion avec arrondi
                Vec2::new(60.0, 10.0),    // Montée
                Vec2::new(-30.0, 10.0),   // Vers la gauche
                Vec2::new(-30.0, -30.0),  // Redescente
            ],
        },
        
        // === PARTIE 7 : JAMBE DIAGONALE DROITE ===
        RPartDefinition {
            name: "Jambe droite du R",
            z_order: 0.46,
            points: vec![
                Vec2::new(60.0, -30.0),   // Départ sous l'arrondi
                Vec2::new(20.0, -30.0),   // Vers l'intérieur
                Vec2::new(60.0, -50.0),   // Diagonale vers le bas-droit
                Vec2::new(100.0, -50.0),  // Extension droite
            ],
        },
        
        // === PARTIE 8 : PIED DROIT ÉLARGI ===
        RPartDefinition {
            name: "Pied droit du R",
            z_order: 0.47,
            points: vec![
                Vec2::new(160.0, -50.0),  // Extrémité droite
                Vec2::new(30.0, -50.0),   // Vers le centre
                Vec2::new(30.0, -80.0),   // Descente
                Vec2::new(120.0, -80.0),  // Largeur du pied
            ],
        },
    ]
}
