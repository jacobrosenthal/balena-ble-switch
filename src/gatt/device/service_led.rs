use bluster::{
    gatt::{characteristic::Characteristic, service::Service},
    SdpShortUuid,
};
use std::collections::HashSet;
use uuid::Uuid;

pub fn create_led(primary: bool, characteristics: HashSet<Characteristic>) -> Service {
    Service::new(
        Uuid::from_sdp_short_uuid(0xA000 as u16),
        primary,
        characteristics,
    )
}
