mod characteristic_led_state;
mod service_led;

use self::{characteristic_led_state::create_led_state, service_led::create_led};
use bluster::gatt::service::Service;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::current_thread::Runtime;

pub fn create_switch(runtime: &Arc<Mutex<Runtime>>) -> Service {
    create_led(true, {
        let mut characteristics = HashSet::new();
        characteristics.insert(create_led_state(runtime, HashSet::new(), (0,)));
        characteristics
    })
}
