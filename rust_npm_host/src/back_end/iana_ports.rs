use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;


#[derive(Debug, Deserialize)]
struct PortRecord {
    #[serde(rename = "Service Name")]
    service_name: String,
    #[serde(rename = "Port Number")]
    port_number: String,
    #[serde(rename = "Transport Protocol")]
    transport_protocol: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Assignee")]
    assignee: String,
    #[serde(rename = "Contact")]
    contact: String,
    #[serde(rename = "Registration Date")]
    registration_date: String,
    #[serde(rename = "Modification Date")]
    modification_date: String,
    #[serde(rename = "Reference")]
    reference: String,
    #[serde(rename = "Service Code")]
    service_code: String,
    #[serde(rename = "Unauthorized Use Reported")]
    unauthorized_use_reported: String,
    #[serde(rename = "Assignment Notes")]
    assignment_notes: String
}




fn load_port_services(file_path: &str) -> Result<(HashMap<String, PortRecord>, HashMap<String, PortRecord>), Box<dyn Error>> {
    let mut tcp_services: HashMap<String, PortRecord> = HashMap::new();
    let mut udp_services: HashMap<String, PortRecord> = HashMap::new();
    let mut range_services: HashMap<String, PortRecord> = HashMap::new();



    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: PortRecord = result?;


        if !record.port_number.is_empty() && !record.service_name.is_empty() {
            
            // I need to see if this conatins a - as a string
            if record.port_number.contains("-") {

                range_services.insert(record.port_number.clone(), record);
                continue;
            }

            if record.port_number.parse::<u16>().is_ok() {

                match record.transport_protocol.to_lowercase().as_str() {
                    "tcp" => {
                        tcp_services.insert(record.port_number.clone(), record);
                    }
                    "udp" => {
                        udp_services.insert(record.port_number.clone(), record);
                    }
                    _ => {} // This is to ignore the unparseble protocols, and all the other ones.
                }
            }
        }
    }
    Ok((tcp_services, udp_services))
}
