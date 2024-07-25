# rust-easybit.io - Rust library for easybit.io exchange API.
This is a Rust library for the easybit.io exchange API. It is a work in progress and is not all functionalities are ready for production use **(KYC enforcement, order refunds)**. It was created to simplify my collaboration as a partner with easybit.io. I decided that contributing to the Rust community would be a good way to give back.

The `Client` struct implements `ZeroizeOnDrop` which shall overwrite the memory of where Client is stored when it goes out of scope. This is to prevent any sensitive information from being leaked on the heap. Thanks to the `zeroize` crate for this.

That being said, this is not an official library and is not endorsed by easybit.io. Therefore, **use it at your own risk**. I am not responsible for any loss of funds or any other damages that may occur from using this library. Please ensure that you have read and understood the easybit.io API documentation before using this library, as well as the source code.

If you have any questions, please feel free to open an issue.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
easybit = "0.0.1"
```

Then, you can use the library as follows:

```rust
use easybit::client::Client;
use std::env;

#[tokio::main]
async fn main() {
    let client = Client::new(env::var("URL").expect("URL must be set"),
    env::var("API_KEY").expect("API_KEY must be set"));
    let account = client.get_account().await.unwrap();
    println!("{:?}", account);
}
```

I suggest that you head over to the crate documentation to see all the available functions. You should also read the [easybit.io](https://easybit.com/en/apidocs) API documentation to get a better understanding.

## Contributing
If you would like to contribute, please feel free to fork the repository and submit a pull request. I will review it as soon as possible.

## License
This library is licensed under the MIT license.

