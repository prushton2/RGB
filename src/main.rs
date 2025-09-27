use openrgb2::{OpenRgbResult, OpenRgbClient, Color};
use tokio;
// use openrgb2::ControllerIndex;

#[tokio::main]
async fn main() -> OpenRgbResult<()> {
    // connect to default server at localhost
    let client = OpenRgbClient::connect().await?;
    let controllers = client.get_all_controllers().await?;
    controllers.init().await?;

    for controller in controllers {
        let mut cmd = controller.cmd();
        
        println!("Controller ID: {}", controller.name());
        
        for led in 0..controller.num_leds() {

            let color: Color = Color::new((((led%3==0) as i32)*255) as u8, (((led%3==1) as i32)*255) as u8, (((led%3==2) as i32)*255) as u8);
            println!("Color {}", color);

            match cmd.set_led(led, color) {
                Ok(_) => {}
                Err(t) => panic!("{}", t)
            };

            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        cmd.execute().await?;

        // cmd.set_all_leds(Color::new(255, 0, 0)).await?;

    }

    Ok(())
}