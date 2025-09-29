// use std::f64::consts;
use openrgb2::{OpenRgbResult, OpenRgbClient, Color, ControllerGroup};
use tokio;
// use openrgb2::ControllerIndex;

/*
AMD Wraith Prism
ASUS TUF GAMING X570-PLUS
Corsair K95 RGB PLATINUM XT
Logitech G915 Wireless RGB Mechanical Gaming Keyboard
*/

use phf::{phf_map};

static KEYMAP: phf::Map<usize, [usize; 6]> = phf_map! {
    0  => [19,20,21,22,23,24],
    2  => [25,26,27,28,29,30,],
    4  => [31,34,0,0,0,0],
    5  => [32,33,0,0,0,0],
    6  => [35,36,39,40,0,0],
    7  => [37,38,0,0,0,0],
    8  => [42,43,46,0,0,0],
    9  => [44,45,0,0,0,0],
    10 => [48,49,52,0,0,0],
    11 => [50,51,0,0,0,0],
    12 => [54,55,58,0,0,0],
    13 => [56,57,0,0,0,0],
    14 => [64,59,62,63,0,0],
    15 => [60,61,0,0,0,0],
    16 => [65,68,0,0,0,0],
    17 => [69,66,67,0,0,0],
    18 => [74,70,73,0,0,0],
    19 => [71,72,0,0,0,0],
    20 => [0,0,0,0,0,0],
    21 => [0,0,0,0,0,0],
    22 => [0,0,0,0,0,0],
    23 => [0,0,0,0,0,0],
    24 => [0,0,0,0,0,0],
    25 => [0,0,0,0,0,0],
    26 => [0,0,0,0,0,0],
    27 => [0,0,0,0,0,0],
    28 => [0,0,0,0,0,0],
    30 => [0,0,0,0,0,0],
    32 => [0,0,0,0,0,0],
    34 => [0,0,0,0,0,0],
    36 => [0,0,0,0,0,0],
    38 => [0,0,0,0,0,0],
    40 => [0,0,0,0,0,0],
    42 => [0,0,0,0,0,0],
};

#[tokio::main]
async fn main() -> OpenRgbResult<()> {
    // connect to default server at localhost
    let client = OpenRgbClient::connect().await?;
    let controllers = client.get_all_controllers().await?;
    controllers.init().await?;

    // let keyboard_container: Controller;
    let mut offset: f64 = 1.0;
    // loop {
    offset += 0.5;
    println!("Offset: {}", offset);
    draw_rainbow(&controllers, offset).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    // }

    Ok(())
}

async fn draw_rainbow(controllers: &ControllerGroup, _offset: f64) -> OpenRgbResult<()> {
    for controller in controllers {
        let mut cmd = controller.cmd();
        if controller.name() != "Corsair K95 RGB PLATINUM XT" {
            continue
        }
        for led in controller.led_iter() {
            match cmd.set_led(led.id(), Color::new(255,255,255)) {
                Ok(_) => {}
                Err(t) => panic!("{}", t)
            };
        }
        cmd.execute().await?;
    }

    for controller in controllers {
        let mut cmd = controller.cmd();
        
        // println!("{}", controller.name());

        if controller.name() != "Corsair K95 RGB PLATINUM XT" {
            continue
        }

        for (key, leds) in KEYMAP.entries() {
            let color = calculate_rainbow(((*key) as f64)/8.0);

            for led in *leds {
                // println!("{}: {}", led, color);
                match cmd.set_led(led, color) {
                    Ok(_) => {}
                    Err(t) => panic!("{}", t)
                };
            }

        }
        
        // for led in controller.led_iter() {

        //     println!("{}: {}", led.id(), led.name());

        //     match cmd.set_led(led.id(), CalculateRainbow((led.id()/8) as f64)) {
        //         Ok(_) => {}
        //         Err(t) => panic!("{}", t)
        //     };
        // }
        cmd.execute().await?;

        // cmd.set_all_leds(Color::new(255, 0, 0)).await?;

    }

    Ok(())
}

fn calculate_rainbow(full_offset: f64) -> Color {
    let offset = full_offset%3.0;
    return Color::new(
        (255.0*(-(offset    ).abs()+1.0)) as u8,
        (255.0*(-(offset-1.0).abs()+1.0)) as u8,
        (255.0*(-(offset-2.0).abs()+1.0)) as u8,
    )
}