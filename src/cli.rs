/// Module de gestion des arguments de ligne de commande.
///
/// Utilise la bibliothèque `clap` en mode derive pour définir
/// les arguments de manière déclarative.
use clap::Parser;

/// Analyseur de trames Wi-Fi pour la détection de drones (DroneID).
///
/// Ce programme permet d'analyser des fichiers PCAP ou de capturer
/// des trames en temps réel pour identifier les trames beacon
/// contenant des informations de signalement électronique de drones.
#[derive(Parser, Debug)]
#[command(name = "pcap_analyzer", about = "Analyse de trames Wi-Fi DroneID")]
pub struct Cli {
    /// Fichier PCAP à analyser
    #[arg(long, conflicts_with = "interface")]
    pub pcap: Option<String>,

    /// Interface réseau pour la capture en temps réel
    #[arg(long, conflicts_with = "pcap")]
    pub interface: Option<String>,

    /// Afficher la liste des interfaces réseau disponibles
    #[arg(long)]
    pub cards: bool,

    /// Filtre de capture (ex: "wlan type mgt subtype beacon")
    #[arg(long)]
    pub filter: Option<String>,

    /// Nombre de paquets à capturer (par défaut 10)
    #[arg(long, default_value_t = 10)]
    pub packet_count: u32,

    /// Format de sortie: json ou csv
    #[arg(long, default_value = "json")]
    pub output_format: String,

    /// Fichier de sortie
    #[arg(long, default_value = "results.json")]
    pub output_file: String,
}
