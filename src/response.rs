// TODO Better struct names
#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentsetPackagesetSenderaddress {
    pub addressLine1: String,
    pub addressLine2: String,
    pub postalCode: String,
    pub city: String,
    pub countryCode: String,
    pub country: String,
}
#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentsetPackagesetEventsetRecipientsignature {
    pub name: String,
}
#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentsetPackagesetEventsetDefinition {
    pub term: String,
    pub explanation: String,
}
#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentsetPackagesetEventset {
    pub description: String,
    pub status: String,
    pub recipientSignature: JsonConsignmentsetPackagesetEventsetRecipientsignature,
    pub unitId: String,
    // pub unitInformationUrl: String,
    pub unitType: String,
    pub postalCode: String,
    pub city: String,
    pub countryCode: String,
    pub country: String,
    pub dateIso: String,
    pub displayDate: String,
    pub displayTime: String,
    pub consignmentEvent: bool,
//    pub definitions: Vec<JsonConsignmentsetPackagesetEventsetDefinition>,
}
#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentsetPackageset {
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
    pub senderAddress: JsonConsignmentsetPackagesetSenderaddress,
    pub recipientHandlingAddress: JsonConsignmentsetPackagesetSenderaddress,
    pub eventSet: Vec<JsonConsignmentsetPackagesetEventset>,
}

#[derive(RustcDecodable, Debug)]
pub struct JsonConsignmentset {
    pub consignmentId: String,
    pub previousConsignmentId: String,
    pub packageSet: Vec<JsonConsignmentsetPackageset>,
    pub senderName: String,
    pub senderAddress: JsonConsignmentsetPackagesetSenderaddress,
    pub recipientHandlingAddress: JsonConsignmentsetPackagesetSenderaddress,
    pub senderReference: String,
    pub totalWeightInKgs: f64,
    pub totalVolumeInDm3: f64,
}
#[derive(RustcDecodable, Debug)]
pub struct Response {
    pub consignmentSet: Vec<JsonConsignmentset>,
}
