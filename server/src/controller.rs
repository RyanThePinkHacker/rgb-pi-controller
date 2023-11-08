use rppal::gpio::{Gpio, OutputPin};

const PWD_FREQUENCY: f64 = 60.0;

pub struct LightController {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
}

impl LightController {
    pub fn new(
        gpio: Gpio,
        red_pin_number: u8,
        green_pin_number: u8,
        blue_pin_number: u8,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            red_pin: gpio.get(red_pin_number)?.into_output(),
            green_pin: gpio.get(green_pin_number)?.into_output(),
            blue_pin: gpio.get(blue_pin_number)?.into_output(),
        })
    }

    fn set_channel(pin: &mut OutputPin, value: u8) -> anyhow::Result<()> {
        pin.set_pwm_frequency(PWD_FREQUENCY, value as f64 / 255.0)?;
        Ok(())
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) -> anyhow::Result<()> {
        Self::set_channel(&mut self.red_pin, red)?;
        Self::set_channel(&mut self.green_pin, green)?;
        Self::set_channel(&mut self.blue_pin, blue)?;
        Ok(())
    }
}
