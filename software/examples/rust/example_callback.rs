use std::{error::Error, io, thread};
use tinkerforge::{ip_connection::IpConnection, temperature_bricklet::*};

const HOST: &str = "localhost";
const PORT: u16 = 4223;
const UID: &str = "XYZ"; // Change XYZ to the UID of your Temperature Bricklet.

fn main() -> Result<(), Box<dyn Error>> {
    let ipcon = IpConnection::new(); // Create IP connection.
    let t = TemperatureBricklet::new(UID, &ipcon); // Create device object.

    ipcon.connect((HOST, PORT)).recv()??; // Connect to brickd.
                                          // Don't use device before ipcon is connected.

    let temperature_receiver = t.get_temperature_callback_receiver();

    // Spawn thread to handle received callback messages.
    // This thread ends when the `t` object
    // is dropped, so there is no need for manual cleanup.
    thread::spawn(move || {
        for temperature in temperature_receiver {
            println!("Temperature: {} °C", temperature as f32 / 100.0);
        }
    });

    // Set period for temperature receiver to 1s (1000ms).
    // Note: The temperature callback is only called every second
    //       if the temperature has changed since the last call!
    t.set_temperature_callback_period(1000);

    println!("Press enter to exit.");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    ipcon.disconnect();
    Ok(())
}
