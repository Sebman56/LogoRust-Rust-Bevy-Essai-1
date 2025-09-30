
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║                          FICHIER: src/main.rs                            ║
// ║  Point d'entrée de l'application                                         ║
// ║  Rôle : Lancer l'application Bevy configurée dans lib.rs                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// Importe la fonction run() depuis le module library
use LogoRust_Bevy_20250929::run;

/// Point d'entrée principal de l'application
/// 
/// Cette fonction est appelée automatiquement au lancement du programme.
/// Elle délègue toute la logique à la fonction run() définie dans lib.rs
/// pour maintenir une séparation claire entre le point d'entrée et la logique.
fn main() {
    // Lancement de l'application Bevy
    run();
}

