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

For any error responses not listed here, assume that the body is either `text/plain` or nonexistent.

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

`POST` a refresh token via the bearer authentication scheme.

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

### Individual account details
**Endpoint**

```
<domain>/onboarding/individual_details
```

**Request**

`POST`
- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:
```
first_name
last_name
dob
dial_code
phone_number
country
city
address
postal_code
tax_id
time_zone
profile_picture
```

**Response**

Success: `200 OK`

### Entity account details
**Endpoint**

```
<domain>/onboarding/entity_details
```

**Request**

`POST`
- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:
```
company_name
country
entity_type
registration_number
tax_id
company_address
city
postal_code
time_zone
logo
```

**Response**

Success: `200 OK`

### PIC details
**Endpoint**

```
<domain>/onboarding/pic_details
```

**Request**

`POST`
- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:
```
first_name
last_name
dob
dial_code
phone_number
profile_picture
```

**Response**

Success: `200 OK`

### Bank details
**Endpoint**

```
<domain>/onboarding/bank_details
```

**Request**

`POST`
- an access token via the bearer authentication scheme
- these fields as `multipart/form-data`:
```
bank_name
account_name
account_number
```

**Response**

Success: `200 OK`

## Password reset
Work in progress.

## Notes
