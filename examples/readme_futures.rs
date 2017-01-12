// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the MIT License, <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#![feature(conservative_impl_trait, plugin)]
#![plugin(tarpc_plugins)]

extern crate futures;
#[macro_use]
extern crate tarpc;
extern crate tokio_core;

use futures::Future;
use tarpc::client::future::{Connect, Options};
use tarpc::util::{FirstSocketAddr, Never};
use tokio_core::reactor;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
struct HelloServer;

impl FutureService for HelloServer {
    type HelloFut = futures::Finished<String, Never>;

    fn hello(&self, name: String) -> Self::HelloFut {
        futures::finished(format!("Hello, {}!", name))
    }
}

fn main() {
    let addr = "localhost:10000".first_socket_addr();
    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();
    HelloServer.listen_with(addr, core.handle()).unwrap();
    core.run(FutureClient::connect(addr, Options::default().handle(handle))
            .map_err(tarpc::Error::from)
            .and_then(|client| client.hello("Mom".to_string()))
            .map(|resp| println!("{}", resp)))
        .unwrap();
}
