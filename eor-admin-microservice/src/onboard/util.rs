use common_utils::error::GlobeliseError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DateWrapper(String);

impl TryFrom<DateWrapper> for sqlx::types::time::Date {
    type Error = GlobeliseError;

    fn try_from(date: DateWrapper) -> Result<Self, Self::Error> {
        sqlx::types::time::Date::parse(date.0, "%F")
            .map_err(|_| GlobeliseError::BadRequest("Date must use YYYY-MM-DD format"))
    }
}

#[derive(Debug)]
pub struct ImageData(Vec<u8>);

impl AsRef<[u8]> for ImageData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for ImageData {
    type Error = GlobeliseError;

    fn try_from(image_data: Vec<u8>) -> Result<Self, Self::Error> {
        match image::guess_format(&image_data)
            .map_err(|_| GlobeliseError::UnsupportedImageFormat)?
        {
            image::ImageFormat::Png | image::ImageFormat::Jpeg => (),
            _ => return Err(GlobeliseError::UnsupportedImageFormat),
        }

        let image = image::load_from_memory(&image_data)
            .map_err(|_| GlobeliseError::UnsupportedImageFormat)?;
        let (width, height) = image::GenericImageView::dimensions(&image);
        if width > IMAGE_DIMENSION_LIMIT || height > IMAGE_DIMENSION_LIMIT {
            return Err(GlobeliseError::PayloadTooLarge(
                "Image dimensions cannot exceed 400px x 400px",
            ));
        }

        Ok(Self(image_data))
    }
}

/// Maximum content length of an onboarding request.
pub const FORM_DATA_LENGTH_LIMIT: u64 = 1024 * 1024 + BASE64_ENCODED_IMAGE_SIZE_LIMIT;

/// Maximum size of an uploaded image when encoded in base64.
const BASE64_ENCODED_IMAGE_SIZE_LIMIT: u64 = IMAGE_SIZE_LIMIT * 4 / 3 + 1;

/// Maximum size of an uploaded image.
const IMAGE_SIZE_LIMIT: u64 = 8 * 1024 * 1024;

/// Maximum dimensions of an uploaded image.
const IMAGE_DIMENSION_LIMIT: u32 = 400;
