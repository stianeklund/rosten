// TODO #[serde(rename(deserialize="name1"))]
#![allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Error {
    pub code: i64,
    pub message: String,
}
#[derive(Deserialize, Debug)]
pub struct ErrorConsignmentSet {
    pub error: Error
}
#[derive(Deserialize, Debug)]
pub struct Json {
    pub consignmentSet: Vec<ErrorConsignmentSet>,
}
#[derive(Deserialize, Debug)]
pub struct PackagesetSenderaddress {
    pub addressLine1: Option<String>,
    pub addressLine2: Option<String>,
    pub postalCode: Option<String>,
    pub city: Option<String>,
    pub countryCode: Option<String>,
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
#[derive(Deserialize, Debug)]
pub struct Eventset {
    pub description: Option<String>,
    pub status: Option<String>,
    pub recipientSignature: Option<Recipientsignature>,
    pub unitId: Option<String>,
    pub unitType: Option<String>,
    pub postalCode: Option<String>,
    pub city: Option<String>,
    pub countryCode: Option<String>,
    pub country: Option<String>,
    pub dateIso: Option<String>,
    pub displayDate: Option<String>,
    pub displayTime: Option<String>,
    pub consignmentEvent: Option<bool>,
    pub definitions: Option<Vec<EventsetDefinition>>,
    pub unitInformationUrl: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Packageset {
    pub statusDescription: Option<String>,
    pub descriptions: Option<Vec<String>>,
    pub packageNumber: Option<String>,
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
    pub senderName: Option<String>,
    pub senderAddress: Option<PackagesetSenderaddress>,
    pub recipientHandlingAddress: Option<PackagesetSenderaddress>,
    pub eventSet: Vec<Eventset>,
}
#[derive(Deserialize, Debug)]
pub struct Consignmentset {
    pub consignmentId: Option<String>,
    pub previousConsignmentId: Option<String>,
    pub packageSet: Vec<Packageset>,
    pub senderName: Option<String>,
    pub senderAddress: Option<PackagesetSenderaddress>,
    pub recipientHandlingAddress: Option<PackagesetSenderaddress>,
    pub senderReference: Option<String>,
    pub totalWeightInKgs: Option<f64>,
    pub totalVolumeInDm3: Option<f64>,
}
#[derive(Deserialize, Debug)]
pub struct BringResponse {
    pub consignmentSet: Vec<Consignmentset>,
}
