use bluster::{
    gatt::{
        characteristic::{Characteristic, Properties, Read, Secure, Write},
        descriptor::Descriptor,
        event::{Event, Response},
    },
    SdpShortUuid,
};
use futures::{prelude::*, sync::mpsc::channel};
use rppal::gpio::Gpio;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;
use uuid::Uuid;

pub fn create_led_state(
    runtime: &Arc<Mutex<Runtime>>,
    descriptors: HashSet<Descriptor>,
    value_fields: (u8,),
) -> Characteristic {
    // Retrieve the GPIO pin and configure it as an output.
    let mut gate = Gpio::new().unwrap().get(18).unwrap().into_output();
    gate.set_low();
    let gate2 = Arc::new(Mutex::new(gate));

    let (led_state,) = value_fields;
    let value = vec![led_state];
    let value2 = Arc::new(Mutex::new(value.clone()));
    let value3 = value.clone();

    let (sender, receiver) = channel(1);
    runtime
        .lock()
        .unwrap()
        .spawn(receiver.for_each(move |event| {
            let value = Arc::clone(&value2);
            let gate = Arc::clone(&gate2);

            match event {
                Event::ReadRequest(read_request) => {
                    read_request
                        .response
                        .send(Response::Success(value.lock().unwrap().clone()))
                        .unwrap();
                }
                Event::WriteRequest(write_request) => {
                    gate.lock().unwrap().toggle();
                    *value.lock().unwrap() = write_request.data;
                    write_request
                        .response
                        .send(Response::Success(vec![]))
                        .unwrap();
                }
                _ => {}
            };
            Ok(())
        }));

    Characteristic::new(
        Uuid::from_sdp_short_uuid(0xA001 as u16),
        Properties::new(
            Some(Read(Secure::Secure(sender.clone()))),
            Some(Write::WithResponse(Secure::Secure(sender.clone()))),
            None,
            None,
        ),
        Some(value3),
        descriptors,
    )
}
