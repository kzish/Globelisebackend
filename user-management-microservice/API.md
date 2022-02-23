# API

**Note:** The API is still a work in progress. Feel free to make suggestions.

Replace `<domain>` with the address that the server listens on.

`<role>` can be one of the following values:

```
client-individual
client-entity
contractor-individual
contractor-entity
eor-admin
```

Optional form fields will be marked `(optional)`.

For any error responses not listed here, assume that the body is either `text/plain` or nonexistent.

[[_TOC_]]

# Authentication

## Getting refresh tokens

### Email sign up

**Endpoint**

```
<domain>/auth/signup/<role>
```

**Request**

`POST` these fields as `application/x-www-form-urlencoded`:

```
email
password
confirm-password
```

**Response**

Success: `200 OK` - `text/plain`

```
<refresh token>
```

Email is unavailable: `422 Unprocessable Entity` - `text/plain`

```
Email is unavailable
```

### Email login

**Endpoint**

```
<domain>/auth/login/<role>
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
<domain>/auth/google/login/<role>
```

**Request**

`POST` Google's ID token as `application/x-www-form-urlencoded`.

**Response**

Success: `200 OK` - `text/plain`

```
<refresh token>
```

## Getting access tokens

**Endpoint**

```
<domain>/auth/access-token
```

**Request**

`POST` a refresh token via the bearer authentication scheme.

**Response**

Success: `200 OK` - `text/plain`

```
<access token>
```

## Getting the public key for verifying tokens

This endpoint is intended for backend use.

**Endpoint**

```
<domain>/auth/public-key
```

**Request**

`GET`

**Response**

Success: `200 OK` - `text/plain`

```
<public key>
```

# Onboarding

## Individual account details

**Endpoint**

```
<domain>/onboard/individual-details
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:

```
first-name
last-name
dob
dial-code
phone-number
country
city
address
postal-code
tax-id (optional)
time-zone
profile-picture (optional)
```

**Response**

Success: `200 OK`

## Entity account details

**Endpoint**

```
<domain>/onboard/entity-details
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:

```
company-name
country
entity-type
registration-number (optional)
tax-id (optional)
company-address
city
postal-code
time-zone
logo (optional)
```

**Response**

Success: `200 OK`

## PIC details

**Endpoint**

```
<domain>/onboard/pic-details
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:

```
first-name
last-name
dob
dial-code
phone-number
profile-picture (optional)
```

**Response**

Success: `200 OK`

## Bank details

**Endpoint**

```
<domain>/onboard/bank-details
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:

```
bank-name
account-name
account-number
```

**Response**

Success: `200 OK`

# Password reset

## Emailing the password reset link

**Endpoint**

```
<domain>/password/reset/email/<role>
```

**Request**

`POST` these fields as `application/x-www-form-urlencoded`:

```
user-email
```

**Response**

Success: `200 OK`

The submitted email address should receive an email with a link to reset their password.

## Accessing the password reset page

This endpoint should only be accessed via the link in password reset emails.

**Endpoint**

```
<domain>/password/reset/initiate
```

**Request**

`GET` with these query params:

```
token
```

**Response**

Success: `303 See Other`

Redirects user to the frontend password reset page with a new one-time token in the query params. 

```
<password reset page>?token=<new token>
```

## Executing the password reset

**Endpoint**

```
<domain>/password/reset/execute
```

**Request**

`POST`

- the provided one-time token via the bearer authentication scheme
- these fields as `application/x-www-form-urlencoded`:

```
new-password
confirm-new-password
```

**Response**

Success: `200 OK`
