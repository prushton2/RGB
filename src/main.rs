#[allow(non_snake_case)]
use std::env;
use std::fs;
use openrgb2::{OpenRgbResult, OpenRgbClient, Color, Controller};
use tokio;
use phf::{phf_map};
use serde::{Deserialize, Serialize};

/*
AMD Wraith Prism
ASUS TUF GAMING X570-PLUS
Corsair K95 RGB PLATINUM XT
Logitech G915 Wireless RGB Mechanical Gaming Keyboard
*/


// This stores each x position on the keyboard and all key ids at that position. 255 is none. 
// 1 is roughly equal to half a key. Gaps between keys are ignored
static KEYMAP: phf::Map<usize, [usize; 8]> = phf_map! {
    0  => [ 19, 20, 21, 22, 23, 24,255,255],
    2  => [ 25, 26, 27, 28, 29, 30,255,255],
    4  => [ 31, 34,255,255,255,255,255,  0],
    5  => [ 32, 33,255,255,255,255,255,255],
    6  => [ 35, 36, 39, 40,255,255, 41,  1],
    7  => [ 37, 38,255,255,255,255,255,255],
    8  => [ 42, 43, 46,255,255,255, 47,  2],
    9  => [ 44, 45,255,255,255,255,255,255],
    10 => [ 48, 49, 52,255,255,255, 53,  3],
    11 => [ 50, 51,255,255,255,255,255,255],
    12 => [ 54, 55, 58,255,255,255,255,  4],
    13 => [ 56, 57,255,255,255,255,255,255],
    14 => [ 64, 59, 62, 63,255,255,255,  5],
    15 => [ 60, 61,255,255,255,255,255,255],
    16 => [ 65, 68,255,255,255,255,255,  6],
    17 => [ 69, 66, 67,255,255,255,255,255],
    18 => [ 74, 70, 73,255,255,255,255,  7],
    19 => [ 71, 72,255,255,255,255,255,255],
    20 => [ 79, 75, 78,255,255,255,255,  8],
    21 => [ 76, 77,255,255,255,255,255,255],
    22 => [ 80, 83, 84,255,255,255,255,  9],
    23 => [ 81, 82,255,255,255,255,255,255],
    24 => [ 85, 86, 89, 90,255,255,255, 10],
    25 => [ 87, 88,255,255,255,255,255,255],
    26 => [ 91, 92,255,255,255,255,255, 11],
    27 => [ 95, 93, 94,255,255,255,255,255],
    28 => [ 97, 98, 99,100,101, 96,255, 12],
    29 => [255,255,255,255,255,255,255,255],
    30 => [102,103,104,105,255,255,255, 13],
    31 => [255,255,255,255,255,255,255,255],
    32 => [106,107,108,109,110,255,255, 14],
    33 => [255,255,255,255,255,255,255,255],
    34 => [111,112,113,114,255,255,255, 15],
    35 => [255,255,255,255,255,255,255,255],
    36 => [116,117,118,119,120,121,115, 16],
    37 => [255,255,255,255,255,255,255,255],
    38 => [122,123,124,125,126,255,255, 17],
    39 => [255,255,255,255,255,255,255,255],
    40 => [127,128,129,130,131,132,255, 18],
    41 => [255,255,255,255,255,255,255,255],
    42 => [133,134,135,136,255,255,255,255],
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    speed: f64,
    left_to_right: i64,
    blend: f64,  // lower = less blend, higher = more blend
    modulo: f64, // interval between matching color states. No idea why mine is 48.
}

#[tokio::main]
async fn main() -> OpenRgbResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path).expect("Failed to read the file");

    let config: Config = serde_yaml::from_str(&file_content)
        .expect("Failed to deserialize the file content into Config struct");

    // connect to default server at localhost
    let client = OpenRgbClient::connect().await?;
    let controllers = client.get_all_controllers().await?;
    controllers.init().await?;

    let mut offset: f64 = 0.0;
    loop {
        offset = (offset+config.speed)%config.modulo;

        for controller in &controllers {
            match controller.name() {
                "Corsair K95 RGB PLATINUM XT" => draw_rainbow_on_keyboard(&controller, if config.left_to_right != 0 {config.modulo-offset} else {offset}, config.blend).await?,
                _ => {}
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
}

async fn draw_rainbow_on_keyboard(controller: &Controller, offset: f64, blend: f64) -> OpenRgbResult<()> {

    let mut cmd = controller.cmd();

    for (key, leds) in KEYMAP.entries() {
        let color = calculate_rainbow((*key as f64 + offset)/blend);

        for led in *leds {
            if led == 255 {
                continue;
            }

            match cmd.set_led(led, color) {
                Ok(_) => {}
                Err(t) => panic!("{}", t)
            };
        }
    }

    cmd.execute().await?;
    Ok(())
}

fn calculate_rainbow(full_offset: f64) -> Color {
    let offset = full_offset%3.0;
    return Color::new(
        (255.0*( (offset-1.5).abs()-0.5)) as u8,
        (255.0*(-(offset-1.0).abs()+1.0)) as u8,
        (255.0*(-(offset-2.0).abs()+1.0)) as u8,
    )
}