
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                        FICHIER: src/config.rs                            ║
// ║  Configuration globale de l'application                                  ║
// ║  Rôle : Centraliser toutes les constantes pour faciliter les ajustements║
// ╚══════════════════════════════════════════════════════════════════════════╝

//! Module de configuration
//! 
//! Ce module centralise toutes les valeurs configurables du projet.
//! Avantages :
//! - Modification facile des paramètres sans toucher au code métier
//! - Vue d'ensemble des dimensions et quantités
//! - Évite les "magic numbers" dispersés dans le code

// === CONFIGURATION DU CERCLE PRINCIPAL ===

/// Rayon du cercle principal en pixels
/// 
/// Détermine la taille globale du logo. Toutes les autres
/// dimensions sont calculées relativement à ce rayon.
/// Valeur recommandée : 150-300 pour un affichage confortable
pub const CIRCLE_RADIUS: f32 = 200.0;

/// Épaisseur de l'anneau du cercle principal en pixels
/// 
/// Définit l'épaisseur de la bande colorée du cercle.
/// Plus la valeur est élevée, plus l'anneau est épais.
/// Recommandation : 10-15% du rayon (20-40 px pour rayon 200)
pub const CIRCLE_THICKNESS: f32 = 30.0;

/// Nombre de segments pour le rendu des cercles
/// 
/// Plus le nombre est élevé, plus le cercle est lisse mais
/// plus le calcul est coûteux. Valeurs recommandées :
/// - 32 : performance (cercle légèrement anguleux)
/// - 64 : équilibré (recommandé)
/// - 128 : qualité maximale (pour zoom ou export)
pub const CIRCLE_SEGMENTS: usize = 64;

// === CONFIGURATION DES TRIANGLES EXTÉRIEURS ===

/// Nombre de triangles extérieurs disposés autour du cercle
/// 
/// Ces triangles forment un anneau arc-en-ciel autour du cercle.
/// Valeur actuelle : 36 triangles espacés de 10° (360°/36)
/// Modification : doit être un diviseur de 360 pour un espacement régulier
/// Exemples : 12 (30°), 24 (15°), 36 (10°), 72 (5°)
pub const EXTERIOR_TRIANGLES_COUNT: usize = 36;

/// Taille d'un côté des triangles extérieurs en pixels
/// 
/// Définit la dimension des petits triangles arc-en-ciel.
/// Recommandation : 10-15% du rayon pour un rendu harmonieux
pub const SMALL_TRIANGLE_SIDE: f32 = 25.0;

// === CONFIGURATION DES TRIANGLES INTÉRIEURS ===

/// Nombre de triangles intérieurs formant un pentagone
/// 
/// Valeur fixe : 5 triangles pour créer une étoile à 5 branches
/// Modification déconseillée sauf pour créer d'autres formes :
/// - 3 : triangle
/// - 4 : carré
/// - 6 : hexagone
pub const INTERIOR_TRIANGLES_COUNT: usize = 5;

/// Taille d'un côté des triangles intérieurs en pixels
/// 
/// Ces triangles sont plus grands que les extérieurs.
/// Recommandation : 35-45% du rayon pour remplir l'espace intérieur
pub const LARGE_TRIANGLE_SIDE: f32 = 80.0;

// === CONFIGURATION DES PETITS CERCLES ===

/// Rayon des petits cercles au centre de chaque triangle intérieur
/// 
/// Ces cercles marquent le centroïde de chaque grand triangle.
/// Recommandation : 5-10% du rayon principal
pub const SMALL_CIRCLE_RADIUS: f32 = 15.0;

/// Qualité de rendu des petits cercles
/// 
/// Peut être inférieur à CIRCLE_SEGMENTS car ces cercles sont plus petits.
/// Valeurs recommandées : 24-48
pub const SMALL_CIRCLE_SEGMENTS: usize = 32;


