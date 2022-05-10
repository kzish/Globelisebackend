use crate::error::GlobeliseError;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DateWrapper(String);

impl TryFrom<DateWrapper> for sqlx::types::time::Date {
    type Error = GlobeliseError;

    fn try_from(date: DateWrapper) -> Result<Self, Self::Error> {
        sqlx::types::time::Date::parse(date.0, "%F").map_err(GlobeliseError::bad_request)
    }
}

impl From<sqlx::types::time::Date> for DateWrapper {
    fn from(date: sqlx::types::time::Date) -> Self {
        Self(date.format("%F"))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionDateWrapper(Option<String>);

impl TryFrom<OptionDateWrapper> for Option<sqlx::types::time::Date> {
    type Error = GlobeliseError;

    fn try_from(date: OptionDateWrapper) -> Result<Self, Self::Error> {
        date.0
            .map(|v| sqlx::types::time::Date::parse(v, "%F").map_err(GlobeliseError::bad_request))
            .transpose()
    }
}

impl From<Option<sqlx::types::time::Date>> for OptionDateWrapper {
    fn from(date: Option<sqlx::types::time::Date>) -> Self {
        Self(date.map(|d| d.format("%F")))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailWrapper(String);

impl TryFrom<EmailWrapper> for EmailAddress {
    type Error = GlobeliseError;

    fn try_from(email: EmailWrapper) -> Result<Self, Self::Error> {
        email
            .0
            .parse::<EmailAddress>()
            .map_err(GlobeliseError::bad_request)
    }
}

impl From<EmailAddress> for EmailWrapper {
    fn from(email: EmailAddress) -> Self {
        EmailWrapper(email.to_string())
    }
}

#[derive(Debug)]
pub struct ImageData(pub Vec<u8>);

impl AsRef<[u8]> for ImageData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for ImageData {
    type Error = GlobeliseError;

    fn try_from(image_data: Vec<u8>) -> Result<Self, Self::Error> {
        match image::guess_format(&image_data).map_err(GlobeliseError::internal)? {
            image::ImageFormat::Png | image::ImageFormat::Jpeg => (),
            _ => return Err(GlobeliseError::UnsupportedImageFormat),
        }

        let image = image::load_from_memory(&image_data).map_err(GlobeliseError::internal)?;
        let (width, height) = image::GenericImageView::dimensions(&image);
        if width > IMAGE_DIMENSION_LIMIT || height > IMAGE_DIMENSION_LIMIT {
            return Err(GlobeliseError::payload_too_large(format!(
                "Image dimensions cannot exceed {IMAGE_DIMENSION_LIMIT} x {IMAGE_DIMENSION_LIMIT}",
            )));
        }

        Ok(Self(image_data))
    }
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "currency")]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYN,
    BZD,
    CAD,
    CDF,
    CHE,
    CHF,
    CHW,
    CLF,
    CLP,
    CNY,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STN,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    USN,
    UYI,
    UYU,
    UYW,
    UZS,
    VED,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XBA,
    XBB,
    XBC,
    XBD,
    XCD,
    XDR,
    XOF,
    XPD,
    XPF,
    XPT,
    XSU,
    XTS,
    XUA,
    XXX,
    YER,
    ZAR,
    ZMW,
    ZWL,
}

/// Maximum content length of an onboarding request.
pub const FORM_DATA_LENGTH_LIMIT: u64 = 1024 * 1024 + BASE64_ENCODED_IMAGE_SIZE_LIMIT;

/// Maximum size of an uploaded image when encoded in base64.
const BASE64_ENCODED_IMAGE_SIZE_LIMIT: u64 = IMAGE_SIZE_LIMIT * 4 / 3 + 1;

/// Maximum size of an uploaded image.
/// 8MB
const IMAGE_SIZE_LIMIT: u64 = 8 * 1024 * 1024;

/// Maximum dimensions of an uploaded image.
const IMAGE_DIMENSION_LIMIT: u32 = 10000;
