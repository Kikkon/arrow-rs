// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Integration test for "mid level" Client


use tonic_web_wasm_client::Client;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use arrow_flight_wasm::{
    FlightClient, FlightDescriptor
    ,
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_wasm_client() {
    let base_url = "http://localhost:50051"; // URL of the gRPC-web server
    let mut client = FlightClient::new(Client::new(base_url.parse().unwrap()));

    client.add_header("foo-header", "bar-header-value").unwrap();
    let request = FlightDescriptor::new_cmd(b"My Command".to_vec());

    let response = client.get_flight_info(request.clone()).await;


    println!("1");
}
