#[macro_use]
extern crate stdweb;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::Array;
use web_sys::{ MessageEvent,  Worker };
// use web_sys::Worker;
// use stdweb::private::wasm_bindgen::JsCast;
// use stdweb::private::wasm_bindgen::JsValue;
use std::sync::{Arc, Mutex};

#[wasm_bindgen]
pub fn main() {
    let message = "Hello, 世界!";
    js! {
        // alert( @{message} );
        console.log(@{message})
    }

    let test = Test::new(3);
    js!{
        console.log(@{test.data})
    }

    let mut  worker  = Worker::new("./worker.js")?;
    worker.post_message(&JsValue::from("from main.rs"));



    // let worker_pool = WorkerPool::new();

}

struct Test {
    data : i32
}

impl Test {
    fn new(i : i32) -> Test {
        Test { data : i * 10 }
    }
}

struct WorkerPool {
    workers:Vec<Worker>,
    callback:Closure<dyn FnMut(MessageEvent) -> Result<(), JsValue>>,
}

impl WorkerPool{
    fn new() -> Result<WorkerPool, JsValue>{
        let callback = Closure::wrap(Box::new(|event: MessageEvent| {
            // let Some(msg) = event.dyn_ref::<MessageEvent>();
            let data = event.data();
            let array = Array::from(&data);
                
            // shift / pop 出来的是JsValue
            let a = array.shift();
            let a = a.as_string();
            let b = array.shift().as_f64();
            let c = array.shift().as_f64();
            // let d = array.shift();
            js!{
                console.log("callback get Data", @{a},@{b},@{c})
            }

            let message = "Hello, callback!";
            js!{
                console.log("callbck",@{message})
            }
            Ok(())
        }) as Box<dyn FnMut(MessageEvent) -> Result<(), JsValue>>);

        let mut workers = Vec::new();

        
        let worker = Worker::new("./worker.js")?;

        let ptr_to_send = Arc::into_raw( Arc::new(Mutex::new(vec![0; 4 * 1024]))) as u32;
        let ptr_to_send2 = JsValue::from(ptr_to_send);

        js!{
            console.log(@{ptr_to_send})
        }

        worker.post_message(&ptr_to_send2)?;
        worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        worker.set_onerror(Some(callback.as_ref().unchecked_ref()));
        workers.push(worker);

        Ok(WorkerPool { workers, callback })
    }
}