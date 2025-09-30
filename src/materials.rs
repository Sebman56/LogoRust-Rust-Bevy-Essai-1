
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                      FICHIER: src/materials.rs                           ║
// ║  Gestion des couleurs et matériaux                                       ║
// ║  Rôle : Définir l'apparence visuelle de tous les éléments               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

//! Module de gestion des matériaux et couleurs
//! 
//! Ce module centralise toutes les définitions de couleurs pour :
//! - Cohérence visuelle du projet
//! - Modification facile des palettes de couleurs
//! - Réutilisation des couleurs

use bevy::prelude::*;

/// Retourne le matériau pour le cercle principal (anneau épais)
/// 
/// Couleur actuelle : Rouge-orangé (#CC3319 approximatif)
/// Format SRGB : (Rouge: 0.8, Vert: 0.2, Bleu: 0.1)
/// 
/// Pour changer la couleur :
/// - Rouge vif : (1.0, 0.0, 0.0)
/// - Bleu : (0.2, 0.4, 0.8)
/// - Vert : (0.2, 0.8, 0.3)
pub fn get_main_circle_color() -> ColorMaterial {
    ColorMaterial::from(Color::srgb(0.8, 0.2, 0.1))
}

/// Génère une couleur arc-en-ciel basée sur l'index du triangle
/// 
/// Principe : utilise le système HSL (Teinte, Saturation, Luminosité)
/// - Teinte (H) : varie de 0° à 360° pour parcourir toutes les couleurs
/// - Saturation (S) : 0.8 (80%) pour des couleurs vives mais pas criardes
/// - Luminosité (L) : 0.6 (60%) pour éviter les couleurs trop claires ou trop sombres
/// 
/// # Arguments
/// * `index` - Position du triangle (0 à EXTERIOR_TRIANGLES_COUNT-1)
/// 
/// # Exemples de couleurs générées
/// - Index 0 (0°) : Rouge
/// - Index 9 (90°) : Jaune-vert
/// - Index 18 (180°) : Cyan
/// - Index 27 (270°) : Violet
pub fn get_rainbow_color(index: usize) -> ColorMaterial {
    // Calcul de l'angle de teinte : chaque triangle décale de 10°
    let hue_fraction = (index as f32 * 10.0) / 360.0;
    
    // Conversion en angle complet (0-360°)
    let hue_degrees = hue_fraction * 360.0;
    
    // Création de la couleur HSL
    let color = Color::hsl(
        hue_degrees,  // Teinte : 0-360°
        0.8,          // Saturation : 80% (couleurs vives)
        0.6           // Luminosité : 60% (ni trop clair ni trop foncé)
    );
    
    ColorMaterial::from(color)
}

/// Retourne la couleur d'un triangle intérieur selon son index
/// 
/// Palette de 5 couleurs distinctes qui se répètent si nécessaire.
/// Chaque couleur est choisie pour être bien distincte des autres.
/// 
/// # Palette actuelle
/// 0. Bleu azur : pour le calme et la confiance
/// 1. Orange : pour l'énergie et la créativité
/// 2. Vert émeraude : pour la nature et l'harmonie
/// 3. Rose fuchsia : pour l'originalité
/// 4. Violet : pour la sophistication
/// 
/// # Arguments
/// * `index` - Position du triangle (0 à INTERIOR_TRIANGLES_COUNT-1)
pub fn get_interior_triangle_color(index: usize) -> ColorMaterial {
    // Définition de la palette de couleurs
    let colors = [
        Color::srgb(0.2, 0.6, 0.9), // Bleu azur
        Color::srgb(0.9, 0.6, 0.2), // Orange
        Color::srgb(0.2, 0.9, 0.6), // Vert émeraude
        Color::srgb(0.9, 0.2, 0.6), // Rose fuchsia
        Color::srgb(0.6, 0.2, 0.9), // Violet
    ];
    
    // Utilisation du modulo pour gérer les index supérieurs à 5
    // Exemple : index 7 → 7 % 5 = 2 → couleur verte
    ColorMaterial::from(colors[index % colors.len()])
}

/// Retourne le matériau pour les petits cercles centraux
/// 
/// Couleur : Blanc semi-transparent (opacité 70%)
/// L'alpha (transparence) permet de voir les triangles en dessous
/// tout en marquant visuellement le centre.
/// 
/// Format SRGBA : (Rouge: 1.0, Vert: 1.0, Bleu: 1.0, Alpha: 0.7)
/// Pour ajuster la transparence, modifier la dernière valeur :
/// - 0.5 : plus transparent
/// - 0.9 : presque opaque
pub fn get_small_circle_color() -> ColorMaterial {
    ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.7))
}

