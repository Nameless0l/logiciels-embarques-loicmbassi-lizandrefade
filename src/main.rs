use clap::Parser;
use pcap_analyzer::cli::Cli;
use pcap_analyzer::output;
use pcap_analyzer::parser;

fn main() {
    let cli = Cli::parse();

    // Mode --cards : lister les interfaces et quitter
    if cli.cards {
        match parser::list_interfaces() {
            Ok(interfaces) => {
                println!("Interfaces réseau disponibles :");
                for iface in &interfaces {
                    println!("  - {iface}");
                }
            }
            Err(e) => eprintln!("Erreur: {e}"),
        }
        return;
    }

    // Analyse depuis un fichier PCAP
    if let Some(ref pcap_path) = cli.pcap {
        println!("Analyse du fichier: {pcap_path}");

        match parser::parse_pcap_file(pcap_path) {
            Ok(beacons) => {
                // Compter les drones
                let drone_count = beacons.iter().filter(|b| b.drone_info.is_some()).count();

                println!(
                    "{} trames beacon détectées, dont {} DroneID",
                    beacons.len(),
                    drone_count
                );
                println!();

                // Afficher les beacons
                for beacon in &beacons {
                    print!("[{}] SSID: \"{}\"", beacon.mac_address, beacon.ssid);
                    if let Some(ref drone) = beacon.drone_info {
                        print!(" ** DRONE **");
                        if let Some(ref id) = drone.id_fr {
                            print!(" ID: {id}");
                        }
                        if let (Some(lat), Some(lon)) = (drone.latitude, drone.longitude) {
                            print!(" GPS: ({lat:.5}, {lon:.5})");
                        }
                        if let Some(alt) = drone.altitude {
                            print!(" Alt: {alt}m");
                        }
                        if let Some(speed) = drone.ground_speed {
                            print!(" Vitesse: {speed}m/s");
                        }
                    }
                    println!();
                }

                // Sauvegarder les résultats
                match output::save_results(&beacons, &cli.output_format, &cli.output_file) {
                    Ok(()) => println!(
                        "\nRésultats sauvegardés dans {} (format {})",
                        cli.output_file, cli.output_format
                    ),
                    Err(e) => eprintln!("Erreur sauvegarde: {e}"),
                }
            }
            Err(e) => eprintln!("Erreur analyse: {e}"),
        }
        return;
    }

    // Capture en temps réel
    if let Some(ref interface) = cli.interface {
        println!("Capture sur l'interface: {interface}");
        println!("Nombre de paquets: {}", cli.packet_count);

        match parser::capture_live(interface, cli.filter.as_deref(), cli.packet_count) {
            Ok(beacons) => {
                println!("{} trames beacon capturées", beacons.len());
                for beacon in &beacons {
                    print!("[{}] SSID: \"{}\"", beacon.mac_address, beacon.ssid);
                    if let Some(ref drone) = beacon.drone_info {
                        print!(" ** DRONE **");
                        if let Some(ref id) = drone.id_fr {
                            print!(" ID: {id}");
                        }
                        if let (Some(lat), Some(lon)) = (drone.latitude, drone.longitude) {
                            print!(" GPS: ({lat:.5}, {lon:.5})");
                        }
                    }
                    println!();
                }

                match output::save_results(&beacons, &cli.output_format, &cli.output_file) {
                    Ok(()) => println!(
                        "\nRésultats sauvegardés dans {} (format {})",
                        cli.output_file, cli.output_format
                    ),
                    Err(e) => eprintln!("Erreur sauvegarde: {e}"),
                }
            }
            Err(e) => eprintln!("Erreur capture: {e}"),
        }
        return;
    }

    // Aucune option spécifiée
    eprintln!("Erreur: spécifiez --pcap, --interface, ou --cards. Utilisez --help pour l'aide.");
}
