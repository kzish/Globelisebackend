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
        .map_err(|_| Error::BadRequest)?
    {
        let name = field.name().ok_or(Error::BadRequest)?;
        let name: T = name.parse().map_err(|_| Error::BadRequest)?;

        if name.is_image() {
            let data = validate_image_field(field).await?;
            if text_fields.contains_key(&name) || byte_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest);
            }
        } else {
            let data = field.text().await.map_err(|_| Error::BadRequest)?;
            if byte_fields.contains_key(&name) || text_fields.insert(name, data).is_some() {
                return Err(Error::BadRequest);
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
            return Err(Error::BadRequest);
        }
    }

    Ok((text_fields, byte_fields))
}

pub async fn validate_image_field(field: Field<'_>) -> Result<Bytes, Error> {
    let filename = field.file_name().ok_or(Error::BadRequest)?;
    let mut file_extension = std::path::Path::new(filename)
        .extension()
        .ok_or(Error::UnsupportedMediaType)?
        .to_owned();
    file_extension.make_ascii_lowercase();
    let file_extension = file_extension;
    if !(file_extension == "png" || file_extension == "jpeg" || file_extension == "jpg") {
        return Err(Error::UnsupportedMediaType);
    }

    let content_type = field.content_type().ok_or(Error::BadRequest)?.to_owned();
    match (content_type.type_(), content_type.subtype()) {
        (mime::IMAGE, mime::PNG) => {
            if file_extension != "png" {
                return Err(Error::UnsupportedMediaType);
            }
        }
        (mime::IMAGE, mime::JPEG) => {
            if !(file_extension == "jpeg" || file_extension == "jpg") {
                return Err(Error::UnsupportedMediaType);
            }
        }
        _ => return Err(Error::UnsupportedMediaType),
    }

    let data = field.bytes().await.map_err(|_| Error::BadRequest)?;
    if data.len() > IMAGE_SIZE_LIMIT {
        return Err(Error::PayloadTooLarge);
    }

    match image::guess_format(data.as_ref()).map_err(|_| Error::UnsupportedMediaType)? {
        image::ImageFormat::Png => {
            if content_type != mime::IMAGE_PNG {
                return Err(Error::UnsupportedMediaType);
            }
        }
        image::ImageFormat::Jpeg => {
            if content_type != mime::IMAGE_JPEG {
                return Err(Error::UnsupportedMediaType);
            }
        }
        _ => return Err(Error::UnsupportedMediaType),
    }

    let image = image::load_from_memory(data.as_ref()).map_err(|_| Error::UnsupportedMediaType)?;
    let (width, height) = image::GenericImageView::dimensions(&image);
    if width > IMAGE_DIMENSION_LIMIT || height > IMAGE_DIMENSION_LIMIT {
        return Err(Error::PayloadTooLarge);
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
