// requis pour la fonction "derive"
use clap::Parser;

/// Simulateur de jeu d'instructions en Rust
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Chemin vers le fichier assembleur à exécuter
    #[clap()]
    pub path: String,

    /// Lance le programme en mode debug
    #[clap(short, long)]
    pub debug: bool,
}