# API

**Note:** The API is still a work in progress. Feel free to make suggestions.

Replace `<domain>` with the address that the server listens on.

`<role>` can be one of the following values: 
```
client_individual
client_entity
contractor_individual
contractor_entity
eor_admin
```

For any error responses not listed here, the response body is not consistent.
Assume that the body is either `text/plain` or nonexistent.

[[_TOC_]]

## Getting refresh tokens

### Email sign up
**Endpoint**

```
<domain>/signup/<role>
```

**Request**

`POST` these fields as `application/x-www-form-urlencoded`:
```
email
password
confirm_password
```

**Response**

Success: `200 OK` - `text/plain`
```
<refresh token>
```

All fields present in request, but fields are invalid: `400 Bad Request` - `application/json`
```json
{
    "is_valid_email": <boolean>,
    "is_password_at_least_8_chars": <boolean>,
    "passwords_match": <boolean>
}
```

Email is unavailable: `422 Unprocessable Entity` - `text/plain`
```
Email is unavailable
```

### Email login
**Endpoint**

```
<domain>/login/<role>
```

**Request**

`POST` these fields as `application/x-www-form-urlencoded`:
```
email
password
```

**Response**

Success: `200 OK` - `text/plain`
```
<refresh token>
```

### Google

#### Getting the ID token
See Google's [guide](https://developers.google.com/identity/gsi/web/guides/display-button)
for displaying the Google sign in button. Tell Google to send the ID token to your JavaScript
handler.

#### Sending the ID token
**Endpoint**

```
<domain>/google/login/<role>
```

**Request**

`POST` Google's ID token as `application/x-www-form-urlencoded`:
```
credentials
```

**Response**

Success: `200 OK` - `text/plain`
```
<refresh token>
```

## Getting access tokens
**Endpoint**

```
<domain>/auth/refresh
```

**Request**

`POST` the refresh token in the `Authorization` header using the Bearer authentication scheme.

**Response**

Success: `200 OK` - `text/plain`
```
<access token>
```

## Getting the public key for verifying tokens
**Endpoint**

```
<domain>/auth/keys
```

**Request**

`GET`

**Response**

Success: `200 OK` - `text/plain`
```
<public key>
```

## Onboarding
**Endpoints**

Prefix all endpoints with `<domain>/onboarding/`.

| Endpoints              | Content Type                         | Prototype contains errors? |
|------------------------|--------------------------------------|----------------------------|
| `individual_details`   | `multipart/form-data`                |  **Yes**[^1]               |
| `entity_details`       | `multipart/form-data`                |  No                        |
| `pic_details`          | `multipart/form-data`                |  No                        |
| `eor_details`          | `multipart/form-data`                |  No                        |
| `bank_details`         | `application/x-www-form-urlencoded`  |  **Yes**[^2]               |
| `eor_bank_details`     | `application/x-www-form-urlencoded`  |  **Yes**[^2]               |

[^1]: The account details for individual contractors are wrong. They should match the form
for individual clients (see Asana ticket).
[^2]: The EOR forms contain errors, but we need confirmation for what the correct fields
should be (there is no Asana ticket). For now, the API mirrors the prototype fields
(except for the profile picture, which is optional).

**Request**

`POST` the form data as the appropriate content type. Put the access token in the
`Authorization` header using the Bearer authentication scheme.

**Response**

Success: `200 OK`

## Password reset
Work in progress.

## Notes
