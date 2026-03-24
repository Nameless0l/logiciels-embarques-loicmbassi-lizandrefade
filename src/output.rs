/// Module de sauvegarde des résultats d'analyse.
///
/// Supporte l'export en JSON et CSV.
use crate::parser::BeaconInfo;
use std::fs::File;
use std::io::Write;

/// Sauvegarde les résultats au format JSON.
pub fn save_json(beacons: &[BeaconInfo], path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(beacons)
        .map_err(|e| format!("Erreur sérialisation JSON: {e}"))?;
    let mut file = File::create(path).map_err(|e| format!("Erreur création fichier: {e}"))?;
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Erreur écriture fichier: {e}"))?;
    Ok(())
}

/// Sauvegarde les résultats au format CSV.
pub fn save_csv(beacons: &[BeaconInfo], path: &str) -> Result<(), String> {
    let mut wtr = csv::Writer::from_path(path).map_err(|e| format!("Erreur création CSV: {e}"))?;

    // En-tête
    wtr.write_record([
        "mac_address",
        "ssid",
        "is_drone",
        "id_fr",
        "latitude",
        "longitude",
        "altitude",
        "height",
        "home_latitude",
        "home_longitude",
        "ground_speed",
        "heading",
    ])
    .map_err(|e| format!("Erreur écriture CSV: {e}"))?;

    for beacon in beacons {
        let (is_drone, id_fr, lat, lon, alt, height, home_lat, home_lon, speed, heading) =
            if let Some(ref drone) = beacon.drone_info {
                (
                    "true".to_string(),
                    drone.id_fr.clone().unwrap_or_default(),
                    drone.latitude.map_or(String::new(), |v| v.to_string()),
                    drone.longitude.map_or(String::new(), |v| v.to_string()),
                    drone.altitude.map_or(String::new(), |v| v.to_string()),
                    drone.height.map_or(String::new(), |v| v.to_string()),
                    drone.home_latitude.map_or(String::new(), |v| v.to_string()),
                    drone
                        .home_longitude
                        .map_or(String::new(), |v| v.to_string()),
                    drone.ground_speed.map_or(String::new(), |v| v.to_string()),
                    drone.heading.map_or(String::new(), |v| v.to_string()),
                )
            } else {
                (
                    "false".to_string(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                )
            };

        wtr.write_record([
            &beacon.mac_address,
            &beacon.ssid,
            &is_drone,
            &id_fr,
            &lat,
            &lon,
            &alt,
            &height,
            &home_lat,
            &home_lon,
            &speed,
            &heading,
        ])
        .map_err(|e| format!("Erreur écriture CSV: {e}"))?;
    }

    wtr.flush().map_err(|e| format!("Erreur flush CSV: {e}"))?;
    Ok(())
}

/// Sauvegarde les résultats selon le format spécifié.
pub fn save_results(beacons: &[BeaconInfo], format: &str, path: &str) -> Result<(), String> {
    match format {
        "json" => save_json(beacons, path),
        "csv" => save_csv(beacons, path),
        _ => Err(format!(
            "Format de sortie inconnu: {format}. Utilisez 'json' ou 'csv'."
        )),
    }
}
