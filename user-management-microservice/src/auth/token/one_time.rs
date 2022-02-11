use std::marker::PhantomData;

use super::*;

/// Creates a one-time token.
pub fn create_one_time_token<T>(ulid: Ulid, role: Role) -> Result<(String, i64), Error>
where
    T: OneTimeTokenAudience,
{
    let expiration = match OffsetDateTime::now_utc().checked_add(T::lifetime()) {
        Some(datetime) => datetime.unix_timestamp(),
        None => return Err(Error::Internal),
    };

    let claims = OneTimeToken::<T> {
        sub: ulid.to_string(),
        role: role.to_string(),
        aud: T::name().into(),
        iss: ISSSUER.into(),
        exp: expiration as usize,
        one_time_audience: PhantomData,
    };

    Ok((
        encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding)
            .map_err(|_| Error::Internal)?,
        expiration,
    ))
}

/// Claims for one-time token
#[derive(Deserialize, Serialize)]
pub struct OneTimeToken<T>
where
    T: OneTimeTokenAudience,
{
    pub sub: String,
    pub role: String,
    aud: String,
    iss: String,
    exp: usize,
    one_time_audience: PhantomData<T>,
}

#[async_trait]
impl<B, T> FromRequest<B> for OneTimeToken<T>
where
    B: Send,
    T: Send + OneTimeTokenAudience,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| Error::Unauthorized)?;
        let Extension(shared_state) = Extension::<SharedState>::from_request(req)
            .await
            .map_err(|_| Error::Internal)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[T::name()]);
        validation.set_issuer(&[ISSSUER]);
        validation.set_required_spec_claims(&["aud", "iss", "exp"]);
        let validation = validation;

        let TokenData { claims, .. } =
            decode::<OneTimeToken<T>>(bearer.token(), &KEYS.decoding, &validation)
                .map_err(|_| Error::Unauthorized)?;
        let ulid: Ulid = claims.sub.parse().map_err(|_| Error::Unauthorized)?;
        let role: Role = claims.role.parse().map_err(|_| Error::Unauthorized)?;

        // Make sure the user actually exists.
        let mut shared_state = shared_state.lock().await;
        if shared_state.user(ulid, role).await?.is_none() {
            return Err(Error::Unauthorized);
        }

        // Do not authorize if the token has already been used.
        if shared_state
            .is_one_time_token_valid::<T>(ulid, bearer.token().as_bytes())
            .await?
        {
            Ok(claims)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

pub trait OneTimeTokenAudience {
    fn name() -> &'static str;
    fn from_str(s: &str) -> Result<(), Error>;
    fn lifetime() -> Duration;
}
