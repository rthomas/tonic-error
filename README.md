[![Crates Badge](https://img.shields.io/crates/v/tonic-error)](https://crates.io/crates/tonic-error)
[![License: Apache 2.0](https://img.shields.io/crates/l/tonic-error)](LICENSE)

A helper trait to assist with passing error types through return `tonic::Status`
messages.

## Usage

This works with the `thiserror` crates, but using that is not required. If you
are not using `thiserror` then at the moment you will need to manually implement
`std::fmt::Display` for your type. Your error type will also need to derive
`serde::{Serialize, Deserialize}`.

In order to use this, you will need to `#[derive(TonicError)]` on your error type.

```rust
#[derive(Debug, Error, TonicError, Serialize, Deserialize)]
pub enum MathsError {
    #[error("division by zero for inputs: a={0} b={1}")]
    DivByZero(i32, i32),
}
```

The `TonicError` trait provides implementations of `std::convert::TryFrom` for
your type, and an implementation of `std::convert::From` for `tonic::Status`.

These examples are taken from the included examples.

### Server Side

```rust
async fn div(&self, req: Request<DivRequest>) -> Result<Response<DivResponse>, Status> {
    let req = req.into_inner();
    if req.b == 0 {
        return Err(MathsError::DivByZero(req.a, req.b).into());
    }
    let result = req.a as f64 / req.b as f64;
    Ok(Response::new(DivResponse { result }))
}
```

### Client Side

```rust
pub async fn div(&mut self, a: i32, b: i32) -> Result<f64, MathsError> {
    let req = Request::new(DivRequest { a, b });

    let resp = match self.client.div(req).await {
        Ok(r) => r,
        Err(e) => match e.code() {
            Code::Internal => {
                return Err(e.try_into().expect("could not convert status to error"))
            }
            _ => panic!("error making rpc call: {e}"),
        },
    };

    Ok(resp.into_inner().result)
}
```

## Example

See the `tonic-error-example` subdirectory in this repo for a working
client/server example. 

## License

This is released under the Apache 2.0 license.