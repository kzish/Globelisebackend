# API

**Warning:** The API is still unstable. Feel free to make suggestions.

[[_TOC_]]

# Notes

- Replace `<domain>` with the address that the server listens on
- `<user type>` can be one of the following values:
    - `individual`
    - `entity`
- `<role>` can be one of the following values:
    - `client`
    - `contractor`
- Optional fields will be marked `(optional)`
- All binary data sent via JSON **must** be base64 encoded

# Authentication

## Getting refresh tokens

### Email sign up

**Endpoint**

```
<domain>/auth/signup/<user type>
```

**Request**

`POST` these fields as `application/json`:

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
<domain>/auth/login/<user type>
```

**Request**

`POST` these fields as `application/json`:

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
<domain>/auth/google/login/<user type>
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
<domain>/onboard/individual-details/<role>
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `application/json`:

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
<domain>/onboard/entity-details/<role>
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `application/json`:

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
<domain>/onboard/pic-details/<role>
```

**Request**

`POST`

- an access token via the bearer authentication scheme
- these fields as `application/json`:

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
- these fields as `application/json`:

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
<domain>/auth/password/reset/email/<user type>
```

**Request**

`POST` these fields as `application/json`:

```
email
```

**Response**

Success: `200 OK`

The submitted email address should receive an email with a link to reset their password.

## Accessing the password reset page

This endpoint should only be accessed via the link in password reset emails.

**Endpoint**

```
<domain>/auth/password/reset/initiate
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
<domain>/auth/password/reset/execute
```

**Request**

`POST`

- the provided one-time token via the bearer authentication scheme
- these fields as `application/json`:

```
new-password
confirm-new-password
```

**Response**

Success: `200 OK`

# Index users

This endpoint is intended for backend use.

**Endpoint**

```
<domain>/eor-admin/users
```

**Request**

`GET` with queries:

| Name          | Description            |
| ------------- | ---------------------- |
| `page`        | Integer                |
| `per_page`    | Integer                |
| `search_text` | String                 |
| `user_type`   | `individual`, `entity` |
| `user_role`   | `contractor`, `client` |

**Response**

Success: `200 OK` - `application/json`

```
<TODO>
```
