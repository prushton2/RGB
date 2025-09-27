use std::f64::consts;
use openrgb2::{OpenRgbResult, OpenRgbClient, Color};
use tokio;
// use openrgb2::ControllerIndex;

/*
AMD Wraith Prism
ASUS TUF GAMING X570-PLUS
Corsair K95 RGB PLATINUM XT
Logitech G915 Wireless RGB Mechanical Gaming Keyboard
*/


#[tokio::main]
async fn main() -> OpenRgbResult<()> {
    // connect to default server at localhost
    let client = OpenRgbClient::connect().await?;
    let controllers = client.get_all_controllers().await?;
    controllers.init().await?;

    for controller in controllers {
        let mut cmd = controller.cmd();
        
        println!("{}", controller.name());
        
        for led in 0..controller.num_leds() {

            // let mut color: Color = Color::new((led/4) as u8,0,0);

            // if led <= 42 {
            //     color = Color::new(
            //         (((led%3==0) as i32)*255) as u8,
            //         (((led%3==1) as i32)*255) as u8,
            //         (((led%3==2) as i32)*255) as u8
            //     );
            // }

            match cmd.set_led(led, CalculateRainbow((led/8) as f64)) {
                Ok(_) => {}
                Err(t) => panic!("{}", t)
            };
        }
        cmd.execute().await?;

        // cmd.set_all_leds(Color::new(255, 0, 0)).await?;

    }

    Ok(())
}

fn CalculateRainbow(offset: f64) -> Color {
    return Color::new(
        (255.0*((offset                       ).cos())) as u8,
        (255.0*((offset + (2.0*consts::PI)/3.0).cos())) as u8,
        (255.0*((offset + (4.0*consts::PI)/3.0).cos())) as u8,
    )
}