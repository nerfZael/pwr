// use std::env;

// use polywrap_client::builder::helpers::{add_default, build_resolver};
// use polywrap_client::client::PolywrapClient;
// use polywrap_client::builder::types::{BuilderConfig, ClientBuilder};
// use polywrap_client::core::uri::Uri;

fn main() {
    println!("Version 0.1.7");

    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     return;
    // }

    // let uri = &args[1];
    // let app_args = &args[2..];

    // let invoke_uri = Uri::try_from(uri.to_string()).unwrap();

    // let mut builder = BuilderConfig::new(Some(add_default()));
    // builder.add(BuilderConfig { 
    //     interfaces: None,
    //     envs: None,
    //     wrappers: None,
    //     packages: None,
    //     redirects: None,
    //     resolvers: None
    // });
    // let config = build_resolver(builder);
    
    // let client = PolywrapClient::new(config);

    // let ser_result = polywrap_msgpack::serialize(app_args);

    // match ser_result {
    //     Ok(serialized) => {
    //         let result = client
    //             .invoke::<i32>(&invoke_uri, "main", Some(&serialized), None, None)
    //             .unwrap();

    //         std::process::exit(result);
    //     }
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         std::process::exit(1);
    //     }
    // }
}
