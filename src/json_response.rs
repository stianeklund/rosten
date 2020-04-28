#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorConsignmentSet {
    pub error: Error,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "consignmentSet")]
pub struct Json {
    pub consignment_set: Vec<ErrorConsignmentSet>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct PackagesetSenderaddress {
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Recipientsignature {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct EventsetDefinition {
    pub term: Option<String>,
    pub explanation: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Eventset {
    pub description: Option<String>,
    pub status: Option<String>,
    pub recipient_signature: Option<Recipientsignature>,
    pub unit_id: Option<String>,
    pub unit_type: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub date_iso: Option<String>,
    pub display_date: Option<String>,
    pub display_time: Option<String>,
    pub consignment_event: Option<bool>,
    pub definitions: Option<Vec<EventsetDefinition>>,
    pub unit_information_url: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Packageset {
    pub status_description: Option<String>,
    pub descriptions: Option<Vec<String>>,
    pub package_number: Option<String>,
    pub previous_package_number: Option<String>,
    pub product_name: Option<String>,
    pub product_code: Option<String>,
    pub brand: Option<String>,
    pub length_in_cm: Option<i64>,
    pub width_in_cm: Option<i64>,
    pub height_in_cm: Option<i64>,
    pub volume_in_dm3: Option<f64>,
    pub weight_in_kgs: Option<f64>,
    pub pickup_code: Option<String>,
    pub date_of_return: Option<String>,
    pub sender_name: Option<String>,
    pub sender_address: Option<PackagesetSenderaddress>,
    pub recipient_handling_address: Option<PackagesetSenderaddress>,
    pub event_set: Vec<Eventset>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Consignmentset {
    pub consignment_id: Option<String>,
    pub previous_consignment_id: Option<String>,
    pub package_set: Vec<Packageset>,
    pub sender_name: Option<String>,
    pub sender_address: Option<PackagesetSenderaddress>,
    pub recipient_handling_address: Option<PackagesetSenderaddress>,
    pub sender_reference: Option<String>,
    pub total_weight_in_kgs: Option<f64>,
    pub total_volume_in_dm3: Option<f64>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct BringResponse {
    pub consignment_set: Vec<Consignmentset>,
}
