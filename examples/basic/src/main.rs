extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json as json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate purescript_waterslide;
#[macro_use] extern crate purescript_waterslide_codegen;

use purescript_waterslide::{PursModule, ToPursType};
use futures::stream::Stream;
use futures::Future;
use std::io::prelude::*;
use std::fs::File;
use hyper::server::*;

#[derive(Debug, ToPursType, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
enum FalafelBasis {
    FavaBean,
    Chickpea,
    Other(Option<String>),
}

#[derive(Debug, ToPursType, Serialize, Deserialize)]
struct Falafel {
    basis: FalafelBasis,
    parsley_percentage: u8,
}

#[derive(Debug, ToPursType, Serialize, Deserialize)]
struct Meal {
    falafels: Vec<Falafel>,
    with_salad: bool,
}

struct FalafelServer;

impl Service for FalafelServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let original = Meal {
            falafels: vec![
                Falafel {
                    basis: FalafelBasis::Chickpea,
                    parsley_percentage: 22,
                },
                Falafel {
                    basis: FalafelBasis::Other(None),
                    parsley_percentage: 8,
                },
                Falafel {
                    basis: FalafelBasis::Other(Some("mango".to_string())),
                    parsley_percentage: 84,
                },
            ],
            with_salad: true,
        };

        match *req.method() {
            hyper::Method::Post => {
                Box::new(
                    req.body()
                    .fold(Vec::<u8>::new(), |mut vec, chunk| {
                        vec.extend(&*chunk);
                        futures::future::ok::<_, hyper::Error>(vec)
                    })
                    .map(|bytes| {
                        let meal = json::from_slice::<Meal>(bytes.as_slice()).expect("could not parse json from frontend");
                        println!("Rust side:\n{:?}", meal);
                        Response::new()
                    })
                )
            },
            _=> Box::new(futures::future::ok(Response::new().with_body(json::to_string(&original).unwrap())))
        }
    }

}

fn falafel_server() {
    let addr = "127.0.0.1:8077".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(FalafelServer)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap()
}

fn main() {
    println!("Generating types...");
    let module = purs_module!("Data.Falafel".to_string() ; Falafel, FalafelBasis, Meal);
    println!("Writing types to frontend/src/Data/Falafel.purs...");
    let mut out = File::create("frontend/src/Data/Falafel.purs").expect("Could not create purescript file");
    write!(out, "{}", module).expect("Could not write purescript file");

    let _guard = ::std::thread::spawn(|| {
        println!("Starting Rust server");
        falafel_server()
    });

    println!("Building frontend...");
    ::std::process::Command::new("bower")
        .current_dir("frontend")
        .arg("install")
        .spawn()
        .expect("could not build the purescript project")
        .wait()
        .unwrap();

    ::std::process::Command::new("pulp")
        .current_dir("frontend")
        .arg("build")
        .arg("-t")
        .arg("out.js")
        .spawn()
        .expect("could not build the purescript project")
        .wait()
        .unwrap();

    println!("Running frontend...");
    ::std::process::Command::new("node")
        .current_dir("frontend")
        .arg("out.js")
        .spawn()
        .expect("could not start the purescript script")
        .wait()
        .unwrap();
}
