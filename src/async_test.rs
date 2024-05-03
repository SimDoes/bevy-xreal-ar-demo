// Copyright (C) 2023, Alex Badics
// This file is part of ar-drivers-rs
// Licensed under the MIT license. See LICENSE file in the project root for details.

mod ar_drivers {
    pub mod lib;
}

use crate::ar_drivers::lib::any_glasses;

fn main() {

    // async implementation

    async_io::block_on(async {
        async_main().await;
    });

    return;

    // sync implementation

    let mut glasses = any_glasses().unwrap();
    println!("Got glasses, serial={}", glasses.serial().unwrap());

    loop {
        let event = glasses.read_event().unwrap();
        println!("Event: {:#?}", event);
    }
    
}

async fn async_main() {

    const NREAL_VID: u16 = 0x3318;

    // async implementation

    use bevy::tasks::futures_lite::StreamExt;
    use async_hid::{AccessMode, DeviceInfo, HidResult};

    let mut device_stream = DeviceInfo::enumerate().await.unwrap();

    while let Some(device) = device_stream.next().await {

        // no way to check for interface_number unfortunately

        // no clue what usage_id means, seems to be 0 or 1

        if device.vendor_id == NREAL_VID && device.usage_id == 0x0 {

            println!("Found device: {:?}", device);

            let device_r = device.open(AccessMode::Read).await.unwrap();

            let mut buffer = [0u8; 0x80];
            let size = device_r.read_input_report(&mut buffer).await.unwrap();
            println!("{:?}", &buffer[..size]);

            std::process::exit(0x0);
        }

        //     let model = match AirModel::try_from(device.product_id()) {
        //         Ok(m) => m,
        //         Err(_) => continue,
        //     };
        //     return Ok((model, device.open_device(AccessMode::ReadWrite).await?));
        // }
    }


    // sync implementation, from nreal_air.rs

    // let hidapi = HidApi::new()?;
    // for device in hidapi.device_list() {
    //     if device.vendor_id() == NREAL_VID && device.interface_number() == interface {
    //         let model = match AirModel::try_from(device.product_id()) {
    //             Ok(m) => m,
    //             Err(_) => continue,
    //         };
    //         return Ok((model, device.open_device(&hidapi)?));
    //     }
    // }
    // Err(Error::NotFound)


    return;
}
