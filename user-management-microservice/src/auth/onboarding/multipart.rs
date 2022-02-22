use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use axum::{
    body::Bytes,
    extract::{multipart::Field, Multipart},
};
use strum::IntoEnumIterator;

use crate::auth::error::Error;

pub async fn extract_multipart_form_data<T>(
    mut multipart: Multipart,
) -> Result<(HashMap<T, String>, HashMap<T, Bytes>), Error>
where
    T: Copy + Eq + Hash + MultipartFormFields + IntoEnumIterator + FromStr,
{
    let mut text_fields = HashMap::<T, String>::new();
    let mut byte_fields = HashMap::<T, Bytes>::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::BadRequest("Bad request"))?
    {
        let name = field.name().ok_or(Error::BadRequest("Bad request"))?;
        let name: T = name.parse().map_err(|_| Error::BadRequest("Bad request"))?;

        if name.is_image() {
            let data = validate_image_field(field).await?;
            if text_fields.contains_key(&name) || byte_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest("Bad request"));
            }
        } else {
            let data = field
                .text()
                .await
                .map_err(|_| Error::BadRequest("Bad request"))?;
            if byte_fields.contains_key(&name) || text_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest("Bad request"));
            }
        }
    }

    let fields_found: HashSet<T> = text_fields
        .keys()
        .chain(byte_fields.keys())
        .copied()
        .collect();
    for field in T::iter().filter(|f| f.is_required()) {
        if !fields_found.contains(&field) {
            return Err(Error::BadRequest("Bad request"));
        }
    }

    Ok((text_fields, byte_fields))
}

pub async fn validate_image_field(field: Field<'_>) -> Result<Bytes, Error> {
    let filename = field.file_name().ok_or(Error::BadRequest("Bad request"))?;
    let mut file_extension = std::path::Path::new(filename)
        .extension()
        .ok_or(Error::UnsupportedImageFormat)?
        .to_owned();
    file_extension.make_ascii_lowercase();
    let file_extension = file_extension;
    if !(file_extension == "png" || file_extension == "jpeg" || file_extension == "jpg") {
        return Err(Error::UnsupportedImageFormat);
    }

    let content_type = field
        .content_type()
        .ok_or(Error::BadRequest("Bad request"))?
        .to_owned();
    match (content_type.type_(), content_type.subtype()) {
        (mime::IMAGE, mime::PNG) => {
            if file_extension != "png" {
                return Err(Error::UnsupportedImageFormat);
            }
        }
        (mime::IMAGE, mime::JPEG) => {
            if !(file_extension == "jpeg" || file_extension == "jpg") {
                return Err(Error::UnsupportedImageFormat);
            }
        }
        _ => return Err(Error::UnsupportedImageFormat),
    }

    let data = field
        .bytes()
        .await
        .map_err(|_| Error::BadRequest("Bad request"))?;
    if data.len() > IMAGE_SIZE_LIMIT {
        return Err(Error::PayloadTooLarge("File size cannot exceed 8MiB"));
    }

    match image::guess_format(data.as_ref()).map_err(|_| Error::UnsupportedImageFormat)? {
        image::ImageFormat::Png => {
            if content_type != mime::IMAGE_PNG {
                return Err(Error::UnsupportedImageFormat);
            }
        }
        image::ImageFormat::Jpeg => {
            if content_type != mime::IMAGE_JPEG {
                return Err(Error::UnsupportedImageFormat);
            }
        }
        _ => return Err(Error::UnsupportedImageFormat),
    }

    let image =
        image::load_from_memory(data.as_ref()).map_err(|_| Error::UnsupportedImageFormat)?;
    let (width, height) = image::GenericImageView::dimensions(&image);
    if width > IMAGE_DIMENSION_LIMIT || height > IMAGE_DIMENSION_LIMIT {
        return Err(Error::PayloadTooLarge(
            "Image dimensions cannot exceed 400px x 400px",
        ));
    }

    Ok(data)
}

pub trait MultipartFormFields {
    fn is_required(&self) -> bool;
    fn is_image(&self) -> bool;
}

/// Maximum length of a `multipart/form-data` request.
pub const FORM_DATA_LENGTH_LIMIT: u64 = 9 * 1024 * 1024;
/// Maximum size of an uploaded image.
const IMAGE_SIZE_LIMIT: usize = 8 * 1024 * 1024;
/// Maximum dimensions of an uploaded image.
const IMAGE_DIMENSION_LIMIT: u32 = 400;
