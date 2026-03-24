/// Module de parsing des fichiers PCAP.
///
/// Gère l'ouverture et la lecture des fichiers PCAP/PCAPNG,
/// le décodage des en-têtes Radiotap et 802.11, l'extraction
/// des trames beacon et le parsing des éléments TLV.
use crate::drone_id::{self, DroneInfo};
use serde::Serialize;

/// Taille de l'en-tête 802.11 MAC pour les trames beacon.
const MAC_HEADER_SIZE: usize = 24;

/// Taille de l'en-tête fixe de gestion (Management Fixed Parameters).
const MGMT_FIXED_SIZE: usize = 12;

/// Informations extraites d'une trame beacon.
#[derive(Debug, Clone, Serialize)]
pub struct BeaconInfo {
    /// Adresse MAC source (transmitter)
    pub mac_address: String,
    /// Nom du réseau Wi-Fi (SSID)
    pub ssid: String,
    /// Informations DroneID si présentes
    pub drone_info: Option<DroneInfo>,
}

/// Formate une adresse MAC à partir de 6 octets.
fn format_mac(bytes: &[u8]) -> String {
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

/// Vérifie si le frame control correspond à une trame beacon.
///
/// Frame Control (2 octets, little-endian) :
/// - Bits 3-2 : type (0 = management)
/// - Bits 7-4 : subtype (8 = beacon)
fn is_beacon_frame(frame_control: u16) -> bool {
    let frame_type = (frame_control >> 2) & 0x03;
    let subtype = (frame_control >> 4) & 0x0F;
    frame_type == 0 && subtype == 8
}

/// Parse les éléments TLV d'un beacon frame.
///
/// Retourne le SSID et les éventuelles informations DroneID.
fn parse_tlv_elements(data: &[u8]) -> (String, Option<DroneInfo>) {
    let mut ssid = String::new();
    let mut drone_info = None;
    let mut offset = 0;

    while offset + 2 <= data.len() {
        let element_id = data[offset];
        let element_len = data[offset + 1] as usize;
        offset += 2;

        if offset + element_len > data.len() {
            break;
        }

        let element_data = &data[offset..offset + element_len];

        match element_id {
            0x00 => {
                // SSID
                ssid = String::from_utf8_lossy(element_data).to_string();
            }
            0xDD => {
                // Vendor Specific - vérifier si c'est du DroneID
                if drone_id::is_drone_id(element_data) {
                    drone_info = Some(drone_id::parse_drone_id(element_data));
                }
            }
            _ => {
                // Autre type TLV, on ignore
            }
        }

        offset += element_len;
    }

    (ssid, drone_info)
}

/// Analyse un fichier PCAP/PCAPNG et extrait les informations des trames beacon.
///
/// Retourne un vecteur de `BeaconInfo` pour chaque trame beacon trouvée.
pub fn parse_pcap_file(path: &str) -> Result<Vec<BeaconInfo>, String> {
    let mut cap =
        pcap::Capture::from_file(path).map_err(|e| format!("Erreur ouverture PCAP: {e}"))?;

    let mut beacons = Vec::new();

    while let Ok(packet) = cap.next_packet() {
        let data = packet.data;

        // L'en-tête Radiotap : la taille est dans les octets 2-3 (little-endian)
        if data.len() < 4 {
            continue;
        }
        let radiotap_len = u16::from_le_bytes([data[2], data[3]]) as usize;

        if data.len() < radiotap_len + 2 {
            continue;
        }

        // En-tête 802.11 MAC : Frame Control (2 octets, little-endian)
        let frame_control = u16::from_le_bytes([data[radiotap_len], data[radiotap_len + 1]]);

        if !is_beacon_frame(frame_control) {
            continue;
        }

        // Vérifier qu'on a assez de données pour le MAC header + management header
        let mac_start = radiotap_len;
        let mgmt_start = mac_start + MAC_HEADER_SIZE + MGMT_FIXED_SIZE;

        if data.len() < mgmt_start {
            continue;
        }

        // Adresse MAC source (transmitter) : octets 10-15 du MAC header
        let mac_addr = format_mac(&data[mac_start + 10..mac_start + 16]);

        // Les éléments TLV commencent après le fixed management header
        let (ssid, drone_info) = parse_tlv_elements(&data[mgmt_start..]);

        beacons.push(BeaconInfo {
            mac_address: mac_addr,
            ssid,
            drone_info,
        });
    }

    Ok(beacons)
}

/// Liste les interfaces réseau disponibles pour la capture.
pub fn list_interfaces() -> Result<Vec<String>, String> {
    let devices = pcap::Device::list().map_err(|e| format!("Erreur listage interfaces: {e}"))?;
    Ok(devices
        .iter()
        .map(|d| {
            let desc = d.desc.as_deref().unwrap_or("pas de description");
            format!("{} ({})", d.name, desc)
        })
        .collect())
}

/// Capture des trames en temps réel sur une interface réseau.
pub fn capture_live(
    interface: &str,
    filter: Option<&str>,
    packet_count: u32,
) -> Result<Vec<BeaconInfo>, String> {
    let mut cap = pcap::Capture::from_device(interface)
        .map_err(|e| format!("Erreur ouverture interface: {e}"))?
        .promisc(true)
        .timeout(5000)
        .open()
        .map_err(|e| format!("Erreur activation capture: {e}"))?;

    if let Some(f) = filter {
        cap.filter(f, true)
            .map_err(|e| format!("Erreur filtre: {e}"))?;
    }

    let mut beacons = Vec::new();
    let mut count = 0;

    while count < packet_count {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;

                if data.len() < 4 {
                    continue;
                }
                let radiotap_len = u16::from_le_bytes([data[2], data[3]]) as usize;

                if data.len() < radiotap_len + 2 {
                    continue;
                }

                let frame_control =
                    u16::from_le_bytes([data[radiotap_len], data[radiotap_len + 1]]);

                if !is_beacon_frame(frame_control) {
                    continue;
                }

                let mac_start = radiotap_len;
                let mgmt_start = mac_start + MAC_HEADER_SIZE + MGMT_FIXED_SIZE;

                if data.len() < mgmt_start {
                    continue;
                }

                let mac_addr = format_mac(&data[mac_start + 10..mac_start + 16]);
                let (ssid, drone_info) = parse_tlv_elements(&data[mgmt_start..]);

                beacons.push(BeaconInfo {
                    mac_address: mac_addr,
                    ssid,
                    drone_info,
                });

                count += 1;
            }
            Err(pcap::Error::TimeoutExpired) => {
                continue;
            }
            Err(e) => {
                return Err(format!("Erreur capture: {e}"));
            }
        }
    }

    Ok(beacons)
}
