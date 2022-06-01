use std::str::FromStr;

use crate::error::GlobeliseError;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OffsetDateWrapper(pub String);

impl TryFrom<OffsetDateWrapper> for sqlx::types::time::OffsetDateTime {
    type Error = GlobeliseError;

    fn try_from(date: OffsetDateWrapper) -> Result<Self, Self::Error> {
        sqlx::types::time::OffsetDateTime::parse(date.0, time::Format::Rfc3339)
            .map_err(GlobeliseError::bad_request)
    }
}

impl From<sqlx::types::time::OffsetDateTime> for OffsetDateWrapper {
    fn from(date: sqlx::types::time::OffsetDateTime) -> Self {
        Self(date.format(time::Format::Rfc3339))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionOffsetDateWrapper(pub Option<String>);

impl TryFrom<OptionOffsetDateWrapper> for Option<sqlx::types::time::OffsetDateTime> {
    type Error = GlobeliseError;

    fn try_from(date: OptionOffsetDateWrapper) -> Result<Self, Self::Error> {
        date.0
            .map(|v| {
                sqlx::types::time::OffsetDateTime::parse(v, time::Format::Rfc3339)
                    .map_err(GlobeliseError::bad_request)
            })
            .transpose()
    }
}

impl From<Option<sqlx::types::time::OffsetDateTime>> for OptionOffsetDateWrapper {
    fn from(date: Option<sqlx::types::time::OffsetDateTime>) -> Self {
        Self(date.map(|d| d.format(time::Format::Rfc3339)))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailWrapper(pub EmailAddress);

impl AsRef<EmailWrapper> for EmailWrapper {
    fn as_ref(&self) -> &EmailWrapper {
        self
    }
}

impl sqlx::Type<sqlx::Postgres> for EmailWrapper {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("text")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for EmailWrapper {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let value: &'_ str = sqlx::decode::Decode::decode(value)?;
        let email = EmailAddress::from_str(value)?;
        Ok(EmailWrapper(email))
    }
}

impl sqlx::encode::Encode<'_, sqlx::Postgres> for EmailWrapper {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val = self.0.as_ref();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val = self.0.as_ref();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
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

impl sqlx::Type<sqlx::Postgres> for ImageData {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("bytea")
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for ImageData {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let value: &[u8] = sqlx::decode::Decode::decode(value)?;
        Ok(ImageData(value.to_owned()))
    }
}

impl sqlx::encode::Encode<'_, sqlx::Postgres> for ImageData {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let val: &[u8] = self.0.as_ref();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::encode(val, buf)
    }
    fn size_hint(&self) -> std::primitive::usize {
        let val: &[u8] = self.0.as_ref();
        sqlx::encode::Encode::<'_, sqlx::Postgres>::size_hint(&val)
    }
}

#[macro_export]
macro_rules! impl_enum_asfrom_str {
    ($name:ident, $($enum_variant:ident),+) => {
        #[derive(::std::clone::Clone, Copy, Debug, Deserialize, Serialize)]
        #[allow(clippy::upper_case_acronyms)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($enum_variant),+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $($name::$enum_variant => stringify!($enum_variant)),+
                }
            }

            pub fn as_array(&self) -> &'static [$name] {
                &[$($name::$enum_variant),+]
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $(stringify!($enum_variant) => Ok($name::$enum_variant)),+,
                    _ => Err(format!("Cannot convert '{}' into an enum variant of '{}'", s, stringify!($name)))
                }
            }
        }

        impl ::sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                ::sqlx::postgres::PgTypeInfo::with_name("text")
            }
        }

        impl<'r> ::sqlx::Decode<'r, ::sqlx::Postgres> for $name {
            fn decode(value: ::sqlx::postgres::PgValueRef<'r>) -> ::std::result::Result<Self, sqlx::error::BoxDynError> {
                let value: &'r str = ::sqlx::decode::Decode::decode(value)?;
                Ok(<$name as ::std::str::FromStr>::from_str(value)?)
            }
        }

        impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut ::sqlx::postgres::PgArgumentBuffer,
            ) -> ::sqlx::encode::IsNull {
                let val = self.as_str();
                ::sqlx::encode::Encode::<'_, ::sqlx::Postgres>::encode(val, buf)
            }
            fn size_hint(&self) -> ::std::primitive::usize {
                let val = self.as_str();
                ::sqlx::encode::Encode::<'_, ::sqlx::Postgres>::size_hint(&val)
            }
        }
    };
}

impl_enum_asfrom_str!(
    Currency, AED, AFN, ALL, AMD, ANG, AOA, ARS, AUD, AWG, AZN, BAM, BBD, BDT, BGN, BHD, BIF, BMD,
    BND, BOB, BOV, BRL, BSD, BTN, BWP, BYN, BZD, CAD, CDF, CHE, CHF, CHW, CLF, CLP, CNY, COP, COU,
    CRC, CUC, CUP, CVE, CZK, DJF, DKK, DOP, DZD, EGP, ERN, ETB, EUR, FJD, FKP, GBP, GEL, GHS, GIP,
    GMD, GNF, GTQ, GYD, HKD, HNL, HRK, HTG, HUF, IDR, ILS, INR, IQD, IRR, ISK, JMD, JOD, JPY, KES,
    KGS, KHR, KMF, KPW, KRW, KWD, KYD, KZT, LAK, LBP, LKR, LRD, LSL, LYD, MAD, MDL, MGA, MKD, MMK,
    MNT, MOP, MRU, MUR, MVR, MWK, MXN, MXV, MYR, MZN, NAD, NGN, NIO, NOK, NPR, NZD, OMR, PAB, PEN,
    PGK, PHP, PKR, PLN, PYG, QAR, RON, RSD, RUB, RWF, SAR, SBD, SCR, SDG, SEK, SGD, SHP, SLL, SOS,
    SRD, SSP, STN, SVC, SYP, SZL, THB, TJS, TMT, TND, TOP, TRY, TTD, TWD, TZS, UAH, UGX, USD, USN,
    UYI, UYU, UYW, UZS, VED, VES, VND, VUV, WST, XAF, XAG, XAU, XBA, XBB, XBC, XBD, XCD, XDR, XOF,
    XPD, XPF, XPT, XSU, XTS, XUA, XXX, YER, ZAR, ZMW, ZWL
);

impl_enum_asfrom_str!(
    Country, AF, AL, DZ, AS, AD, AO, AI, AQ, AG, AR, AM, AW, AU, AT, AZ, BS, BH, BD, BB, BY, BE,
    BZ, BJ, BM, BT, BO, BA, BW, BV, BR, IO, BN, BG, BF, BI, KH, CM, CA, CV, KY, CF, TD, CL, CN, CX,
    CC, CO, KM, CG, CD, CK, CR, CI, HR, CU, CY, CZ, DK, DJ, DM, DO, EC, EG, SV, GQ, ER, EE, ET, FK,
    FO, FJ, FI, FR, GF, PF, TF, GA, GM, GE, DE, GH, GI, GR, GL, GD, GP, GU, GT, GN, GW, GY, HT, HM,
    VA, HN, HK, HU, IS, IN, ID, IR, IQ, IE, IL, IT, JM, JP, JO, KZ, KE, KI, KP, KR, KW, KG, LA, LV,
    LB, LS, LR, LY, LI, LT, LU, MO, MK, MG, MW, MY, MV, ML, MT, MH, MQ, MR, MU, YT, MX, FM, MD, MC,
    MN, MS, MA, MZ, MM, NA, NR, NP, NL, AN, NC, NZ, NI, NE, NG, NU, NF, MP, NO, OM, PK, PW, PS, PA,
    PG, PY, PE, PH, PN, PL, PT, PR, QA, RE, RO, RU, RW, SH, KN, LC, PM, VC, WS, SM, ST, SA, SN, CS,
    SC, SL, SG, SK, SI, SB, SO, ZA, GS, ES, LK, SD, SR, SJ, SZ, SE, CH, SY, TW, TJ, TZ, TH, TL, TG,
    TK, TO, TT, TN, TR, TM, TC, TV, UG, UA, AE, GB, US, UM, UY, UZ, VU, VE, VN, VG, VI, WF, EH, YE,
    ZM, ZW
);

/// Maximum content length of an onboarding request.
pub const FORM_DATA_LENGTH_LIMIT: u64 = 1024 * 1024 + BASE64_ENCODED_IMAGE_SIZE_LIMIT;

/// Maximum size of an uploaded image when encoded in base64.
const BASE64_ENCODED_IMAGE_SIZE_LIMIT: u64 = IMAGE_SIZE_LIMIT * 4 / 3 + 1;

/// Maximum size of an uploaded image.
/// 8MB
const IMAGE_SIZE_LIMIT: u64 = 8 * 1024 * 1024;

/// Maximum dimensions of an uploaded image.
const IMAGE_DIMENSION_LIMIT: u32 = 10000;
