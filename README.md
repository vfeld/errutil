# What is this

A small crate used to collect the source code location at which an error is generated.
In the following example an axum handler returns an error.

```rust
#[derive(thiserror::Error, Debug)]
pub enum SomeError {
    #[error("this error happend here")]
    InvalidId(errutil::ErrorInfo)
}

fn handler(id: u32) -> Result<Response, SomeError> {
    if id == 0 {
        Err(SomeError::InvalidId(errutil::info!())).into_response();
    } else {
        Ok(StatusCode::OK.into_response())
    }
}
```

For this to work the error type needs to implement the axum IntoResponse trait:

```rust
impl IntoResponse for SomeError {
    #[tracing::instrument(skip_all)]
    fn into_response(self) -> Response {
        match &self {
            SomeError::InvalidId(error_info) => errutil::err_resp(StatusCode::BAD_REQUEST, &self, error_info),
        }
    }
}
```
The errutil::err_resp function helps to generate the JSON body and logs an error using an unique error_id and the source code location of where the error is raised.

The http response json body will look as follows: 

```json
{
        "status_code": 400,
        "status_description": "Bad Request",
        "error_id": "9c5b94b1-35ad-49bb-b118-8e8fc24abf8",
}
```

# License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.