extern crate core;

use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::{Data, resource, route, service};


mod controllers;
use controllers::{
    wallet_controller

};
mod models;
use models::{response};
mod blockchain;
use blockchain::wallet;
use blockchain::transfer;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::node::Node;
use crate::models::block::{Block, Chain};
use crate::req_models::wallet_requests::CreateWalletReq;

mod utils;
mod req_models;
mod middlewares;





#[get("/")]
async fn index() -> impl Responder {
    "Hello, Bread!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

// std::io::Result<()>
#[actix_web::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let kv_store =match  KvStore::create_db("chain".to_string(),r"\data\tomas\".to_string()){
        Ok(kv_store)=>{kv_store},
        Err(err)=>{return println!("{}",err.to_string())}
    };
    let chain = Chain{ chain: vec![Block{
        id: "hcb d n".to_string(),
        sender_address: "sender".to_string(),
        receiver_address: "".to_string(),
        date_created: "".to_string(),
        hash: "jndljvnkfj".to_string(),
        amount: 0.0,
        public_key: "".to_string(),
    }] };
    match kv_store.save(Some(chain)){
        Ok(_)=>{println!("successful ")},
        Err(err)=>{return println!("{}",err.to_string())}
    }

    //Node::serve();
    // wallet::Wallet::create_wallet("Vcd0e7061eb04343c31118725afa6853603db77a0658deeb1667523336211efbe6".to_string(),
    // "nMCgCIQDmYZuKCBMCGX8ApVNzV3v6fn8IyTghmWe1mBTK8Y5LOwIDAQAB".to_string());

    // match transfer::Transfer::transfer("David".to_string(), "john".to_string(), 10.0) {
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         println!("{:?}", err)
    //     }
    // }
    // match transfer::Transfer::validate("Vcfd8019b4ace8b3b2ee8a7da662a1baaf2bf3f2201001fc0d1757d04cd31980d9".to_string(),
    //                                    "Vcd0e7061eb04343c31118725afa6853603db77a0658deeb1667523336211efbe6".to_string(),
    //
    // ) { }

    // match transfer::Transfer::sign_messafe("LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQ0KTUlHckFnRUFBaUVBeS9Ub05MVW5pakR4NytjZk8yZ3pnVEM5ZGdqbENncDV5eVlCR0tJejlCa0NBd0VBQVFJZw0KQXk0NGlNbmlhZmRGYnBaT0dHRVJuSVVBaUFEKy9IOXQ3Tk55V1R6c3VBRUNFUURYbHJqTk8wekQwaVlpRmM1UQ0KUlZhWkFoRUE4akFCZkFxT0ZITjZsdW1kcWgxNWdRSVJBS0V5N3ExdVdLRFhBbkpjRWE4Tm1Za0NFQ3lBR2FXNw0KSXNRck85bEM3ODkwUHdFQ0VRQ1JIME1Ga1N5RW9MWU94R1d1aWJVRw0KLS0tLS1FTkQgUlNBIFBSSVZBVEUgS0VZLS0tLS0NCg".to_string(),
    // "Hello".to_string()) {
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         println!("{:?}", err)
    //     }
    // }

    // match transfer::Transfer::verify("LS0tLS1CRUdJTiBSU0EgUFVCTElDIEtFWS0tLS0tDQpNQ2dDSVFETDlPZzB0U2VLTVBIdjV4ODdhRE9CTUwxMkNPVUtDbm5MSmdFWW9qUDBHUUlEQVFBQg0KLS0tLS1FTkQgUlNBIFBVQkxJQyBLRVktLS0tLQ0K".to_string(),
    //                                  "M8EnvyXyi9fI_FmDPjChXRLlCQxpkZRVQec1WsPYXqU".to_string(),
    //                                  "Hello".to_string()){
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         println!("{:?}", err.to_string())
    //     }
    // };
    // match wallet::Wallet::get_balance(&"David".to_string()){
    //     Ok(data)=>{println!("{:?}", data)},
    //     Err(err)=>{
    //         println!("{:?}", err)
    //     }
    // }
    // transfer::Transfer::edd_sign("MFMCAQEwBQYDK2VwBCIEIN-TKDHHhxdhe1HgXuoqhBf4AV2gL5mMU5B1tDzrXT3aoSMDIQBOwJ1dFomg3tbRbXRnGZja545SWrbtvHq7hezTbH3h6Q".to_string(),
    // "hello this is some cary shit man i am teling you".to_string());
    // transfer::Transfer::edd_verify("TsCdXRaJoN7W0W10ZxmY2ueOUlq27bx6u4Xs02x94ek".to_string(),
    //     "z0XcX93FV7vmo6V643uxCiSQr7hAAamHma5qoyUZYaVv66EfGwfO2xXE1Scr2QW7O2CaNKxmdTlCqsYf0KXEAQ".to_string(),
    // "hello".to_string());
    // transfer::Transfer::generate_wallet()
    // wallet::Wallet::generate_key();
    // wallet::Wallet::generate_key();
    // HttpServer::new(move|| {
    //     App::new()
    //
    //
    //         // USER CONTROLLERS
    //
    //         .service(
    //             // all authenticated endpoints
    //             web::scope("api/v1/")
    //                 .service(wallet_controller::create_wallet)
    //         )
    //         // .service(user_controller::create_user)
    //         // .service(user_controller::login_user)
    //         // .service(power_ups_controller::use_power_up)
    //         // .service(user_controller::get_code)
    //         //
    // })
    //     .bind(("127.0.0.1", 80))?
    //     .run()
    //     .await
}