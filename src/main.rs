mod gatt;

use self::gatt::{create_device_info, create_switch};
use bluster::Peripheral;
use futures::{future, prelude::*};
use std::sync::{Arc, Mutex};
use tokio::runtime::current_thread::Runtime;
use futures_timer::FutureExt;
use std::time::Duration;
use std::{thread, time};

const ADVERTISING_NAME: &str = "walk don't walk";

fn main() {
    println!("delay start up");

    thread::sleep(Duration::new(5, 0));

    println!("starting up again");

    let runtime = Arc::new(Mutex::new(Runtime::new().unwrap()));

    // Create peripheral
    let peripheral_future = Peripheral::new(&runtime);
    let peripheral = Arc::new({ runtime.lock().unwrap().block_on(peripheral_future).unwrap() });
    peripheral
        .add_service(&create_device_info(&runtime))
        .unwrap();
    peripheral.add_service(&create_switch(&runtime)).unwrap();

    // Create advertisement
    let advertisement = future::loop_fn(Arc::clone(&peripheral), |peripheral| {
        peripheral.is_powered().and_then(move |is_powered| {
            if is_powered {
                println!("Peripheral powered on");
                Ok(future::Loop::Break(peripheral))
            } else {
                println!("Peripheral off");
                Ok(future::Loop::Continue(peripheral))

            }
        })
    })
    .timeout(Duration::from_secs(3))
    .and_then(|peripheral| {
        println!("start advertising");

        let peripheral2 = Arc::clone(&peripheral);
        peripheral
            .start_advertising(ADVERTISING_NAME, &[])
            .and_then(move |advertising_stream| Ok((advertising_stream, peripheral2)))
    })
    .and_then(|(advertising_stream, peripheral)| {
        println!("check advertising advertising");

        let handled_advertising_stream = advertising_stream.for_each(|_| Ok(()));

        let advertising_check = future::loop_fn(Arc::clone(&peripheral), move |peripheral| {
            peripheral.is_advertising().and_then(move |is_advertising| {
                if is_advertising {
                    println!("Peripheral started advertising \"{}\"", ADVERTISING_NAME);
                    Ok(future::Loop::Break(peripheral))
                } else {
                    println!("not advertising");
                    Ok(future::Loop::Continue(peripheral))
                }
            })
        });

        advertising_check.fuse().join(handled_advertising_stream)
    })
    .then(|_| {
        println!("something happened");

        Ok(())
    });

    // Spawn never ending process
    let mut runtime = runtime.lock().unwrap();
    runtime.spawn(advertisement);
    runtime.run().unwrap();
}
