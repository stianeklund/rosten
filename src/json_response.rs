// TODO Better struct names
#![allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
pub struct Senderaddress {
    pub addressLine1: String,
    pub addressLine2: String,
    pub postalCode: String,
    pub city: String,
    pub countryCode: String,
    pub country: String,
}
#[derive(RustcDecodable, Debug)]
pub struct Recipientsignature {
    pub name: String,
}
#[derive(RustcDecodable, Debug)]
pub struct EventsetDefinition {
    pub term: String,
    pub explanation: String,
}
#[derive(RustcDecodable, Debug)]
pub struct Eventset {
    pub description: String,
    pub status: String,
    pub recipientSignature: Recipientsignature,
    pub unitId: String,
    pub unitInformationUrl: Option<String>,
    pub unitType: String,
    pub postalCode: String,
    pub city: String,
    pub countryCode: String,
    pub country: String,
    pub dateIso: String,
    pub displayDate: String,
    pub displayTime: String,
    pub consignmentEvent: bool,
    pub definitions: Option<Vec<EventsetDefinition>>
}
#[derive(RustcDecodable, Debug)]
pub struct Packageset {
    pub statusDescription: String,
    pub descriptions: Vec<String>,
    pub packageNumber: String,
    pub previousPackageNumber: String,
    pub productName: String,
    pub productCode: String,
    pub brand: String,
    pub lengthInCm: i64,
    pub widthInCm: i64,
    pub heightInCm: i64,
    pub volumeInDm3: f64,
    pub weightInKgs: f64,
    pub pickupCode: String,
    pub dateOfReturn: String,
    pub senderName: String,
    pub senderAddress: Senderaddress,
    pub recipientHandlingAddress: Senderaddress,
    pub eventSet: Vec<Eventset>,
}

#[derive(RustcDecodable, Debug)]
pub struct Consignmentset {
    pub consignmentId: String,
    pub previousConsignmentId: String,
    pub packageSet: Vec<Packageset>,
    pub senderName: String,
    pub senderAddress: Senderaddress,
    pub recipientHandlingAddress: Senderaddress,
    pub senderReference: String,
    pub totalWeightInKgs: f64,
    pub totalVolumeInDm3: f64,
}
#[derive(RustcDecodable, Debug)]
pub struct BringResponse {
    pub consignmentSet: Vec<Consignmentset>,
}
