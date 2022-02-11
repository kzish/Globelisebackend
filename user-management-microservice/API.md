# API

**Note:** The API is still a work in progress. Feel free to make suggestions.

Replace `<domain>` with the address that the server listens on.
Currently, it listens on `localhost:3000`.

`<role>` can be one of the following values: 
```
client_individual
client_entity
contractor_individual
contractor_entity
```

There is also an `admin` value, but it has been disabled until there is a way to restrict access
to those pages. Only Globelise admins should be able to access the admin pages, and only after
they have been invited.

For any error responses not listed here, assume that the response body is either plaintext or empty.

[[_TOC_]]

## Getting refresh tokens

### Email sign up
**Endpoint**

```
<domain>/signup/<role>
```

**Request**

`POST` these fields as `x-www-form-urlencoded`:
```
email
password
confirm_password
```

**Response**

Success: `text/plain`
```
<refresh token>
```

Unsuccessful sign up, but all fields are present in request: `application/json`
```json
{
    "is_valid_email": <boolean>,
    "is_email_available": <boolean>,
    "is_password_at_least_8_chars": <boolean>,
    "passwords_match": <boolean>
}
```

### Email login
**Endpoint**

```
<domain>/login/<role>
```

**Request**

`POST` these fields as `x-www-form-urlencoded`:
```
email
password
```

**Response**

Success: `text/plain`
```
<refresh token>
```

### Google

#### Display the sign in with Google button
See Google's [guide](https://developers.google.com/identity/gsi/web/guides/display-button)
for displaying the Google sign in button.

For **sign up**, when rendering the button using:
- HTML - set `data-login_uri` to `<domain>/google/signup/<role>?=<callback uri>`
- JavaScript - set `login_uri` to `<domain>/google/signup/<role>?=<callback uri>`

For **login**, when rendering the button using:
- HTML - set `data-login_uri` to `<domain>/google/login/<role>?=<callback uri>`
- JavaScript - set `login_uri` to `<domain>/google/login/<role>?=<callback uri>`

#### Handling the response
After validating the ID token from Google, the server will redirect the user to:
```
<callback uri>?token=<one time token>
```
where `<callback uri>` is a URI of your choosing. The one-time token will be sent as a
query parameter using the key `token`. To get a refresh token:

**Endpoint**

```
<domain>/google/authorize
```

**Request**

`POST` the one-time token using the Bearer authentication scheme.

**Response**

Success: `text/plain`
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

Success: `text/plain`
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

Success: `text/plain`
```
<public key>
```
