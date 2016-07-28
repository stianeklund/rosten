#![allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]

pub struct ConsignmentsetError {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i64>,
    pub message: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Senderaddress {
    pub addressLine1: Option<String>,
    pub addressLine2: Option<String>,
    pub postalCode: Option<String>,
    pub city: Option<String>,
    pub countryCode: Option<String>,
    pub country: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipientsignature {
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EventsetDefinition {
    pub term: Option<String>,
    pub explanation: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Eventset {
    pub description: String,
    pub status: String,
    pub recipientSignature: Option<Recipientsignature>,
    pub unitId: Option<String>,
    pub unitInformationUrl: Option<String>,
    pub unitType: Option<String>,
    pub postalCode: Option<String>,
    pub city: Option<String>,
    pub countryCode: Option<String>,
    pub country: Option<String>,
    pub dateIso: Option<String>,
    pub displayDate: Option<String>,
    pub displayTime: Option<String>,
    pub consignmentEvent: bool,
    pub definitions: Option<Vec<EventsetDefinition>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Packageset {
    pub statusDescription: String,
    pub descriptions: Vec<String>,
    pub packageNumber: String,
    pub previousPackageNumber: Option<String>,
    pub productName: Option<String>,
    pub productCode: Option<String>,
    pub brand: Option<String>,
    pub lengthInCm: Option<i64>,
    pub widthInCm: Option<i64>,
    pub heightInCm: Option<i64>,
    pub volumeInDm3: Option<f64>,
    pub weightInKgs: Option<f64>,
    pub pickupCode: Option<String>,
    pub dateOfReturn: Option<String>,
    pub senderName: String,
    pub senderAddress: Option<Senderaddress>,
    pub recipientHandlingAddress: Option<Senderaddress>,
    pub eventSet: Vec<Eventset>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Consignmentset {
    pub consignmentId: Option<String>,
    pub previousConsignmentId: Option<String>,
    pub packageSet: Vec<Packageset>,
    pub senderName: Option<String>,
    pub senderAddress: Option<Senderaddress>,
    pub recipientHandlingAddress: Option<Senderaddress>,
    pub senderReference: Option<String>,
    pub totalWeightInKgs: Option<f64>,
    pub totalVolumeInDm3: Option<f64>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub error: Vec<ConsignmentsetError>, // This probably shouldn't be a Vec?
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BringResponse {
    pub consignmentSet: Vec<Consignmentset>,
}
