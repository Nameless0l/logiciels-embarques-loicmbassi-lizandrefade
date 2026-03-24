/// Module de parsing des trames DroneID (signalement électronique français).
///
/// Implémente le décodage des trames Vendor Specific contenant
/// les informations de signalement électronique des drones,
/// conformément à l'arrêté du 27 décembre 2019.
use serde::Serialize;

/// OUI identifiant les trames de signalement électronique DroneID.
pub const DRONE_ID_OUI: [u8; 3] = [0x6A, 0x5C, 0x35];

/// VS Type pour le signalement électronique.
pub const DRONE_ID_VS_TYPE: u8 = 0x01;

/// Informations extraites d'une trame DroneID.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DroneInfo {
    /// Version du protocole
    pub protocol_version: Option<u8>,
    /// Identifiant français du drone (30 caractères)
    pub id_fr: Option<String>,
    /// Latitude courante en degrés WGS84
    pub latitude: Option<f64>,
    /// Longitude courante en degrés WGS84
    pub longitude: Option<f64>,
    /// Altitude au-dessus du niveau de la mer (mètres)
    pub altitude: Option<i16>,
    /// Hauteur au-dessus du point de décollage (mètres)
    pub height: Option<i16>,
    /// Latitude du point de décollage en degrés WGS84
    pub home_latitude: Option<f64>,
    /// Longitude du point de décollage en degrés WGS84
    pub home_longitude: Option<f64>,
    /// Vitesse au sol (m/s)
    pub ground_speed: Option<u8>,
    /// Cap en degrés (0-359)
    pub heading: Option<u16>,
}

impl DroneInfo {
    /// Crée une nouvelle instance vide de `DroneInfo`.
    pub fn new() -> Self {
        Self {
            protocol_version: None,
            id_fr: None,
            latitude: None,
            longitude: None,
            altitude: None,
            height: None,
            home_latitude: None,
            home_longitude: None,
            ground_speed: None,
            heading: None,
        }
    }
}

/// Vérifie si un élément Vendor Specific est une trame DroneID.
///
/// Retourne `true` si les 3 premiers octets correspondent à l'OUI DroneID
/// et que le 4e octet est le VS Type attendu.
pub fn is_drone_id(vendor_data: &[u8]) -> bool {
    vendor_data.len() >= 4
        && vendor_data[0..3] == DRONE_ID_OUI
        && vendor_data[3] == DRONE_ID_VS_TYPE
}

/// Parse le payload TLV d'une trame DroneID.
///
/// Le payload commence après l'OUI (3 octets) et le VS Type (1 octet),
/// soit à l'offset 4 du Vendor Specific data.
pub fn parse_drone_id(vendor_data: &[u8]) -> DroneInfo {
    let mut info = DroneInfo::new();

    // Le payload TLV commence après OUI (3) + VS Type (1)
    let tlv_data = &vendor_data[4..];
    let mut offset = 0;

    while offset + 2 <= tlv_data.len() {
        let tlv_type = tlv_data[offset];
        let tlv_len = tlv_data[offset + 1] as usize;
        offset += 2;

        if offset + tlv_len > tlv_data.len() {
            break;
        }

        let value = &tlv_data[offset..offset + tlv_len];

        match tlv_type {
            1 => {
                // Protocol version
                if tlv_len >= 1 {
                    info.protocol_version = Some(value[0]);
                }
            }
            2 => {
                // ID_FR (30 bytes UTF-8)
                info.id_fr = Some(
                    String::from_utf8_lossy(value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            3 => {
                // ID_ANSI_CTA (optionnel, on l'ignore pour l'instant)
            }
            4 => {
                // Latitude (int32 big-endian, * 10^5)
                if tlv_len >= 4 {
                    let raw = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                    info.latitude = Some(raw as f64 / 100_000.0);
                }
            }
            5 => {
                // Longitude (int32 big-endian, * 10^5)
                if tlv_len >= 4 {
                    let raw = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                    info.longitude = Some(raw as f64 / 100_000.0);
                }
            }
            6 => {
                // Altitude (int16 big-endian)
                if tlv_len >= 2 {
                    info.altitude = Some(i16::from_be_bytes([value[0], value[1]]));
                }
            }
            7 => {
                // Height (int16 big-endian)
                if tlv_len >= 2 {
                    info.height = Some(i16::from_be_bytes([value[0], value[1]]));
                }
            }
            8 => {
                // Home latitude (int32 big-endian, * 10^5)
                if tlv_len >= 4 {
                    let raw = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                    info.home_latitude = Some(raw as f64 / 100_000.0);
                }
            }
            9 => {
                // Home longitude (int32 big-endian, * 10^5)
                if tlv_len >= 4 {
                    let raw = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
                    info.home_longitude = Some(raw as f64 / 100_000.0);
                }
            }
            10 => {
                // Ground speed (uint8, m/s)
                if tlv_len >= 1 {
                    info.ground_speed = Some(value[0]);
                }
            }
            11 => {
                // Heading (uint16 big-endian, degrés)
                if tlv_len >= 2 {
                    info.heading = Some(u16::from_be_bytes([value[0], value[1]]));
                }
            }
            _ => {
                // Type inconnu, on ignore
            }
        }

        offset += tlv_len;
    }

    info
}
