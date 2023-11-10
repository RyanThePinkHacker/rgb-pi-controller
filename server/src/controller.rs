use common::Color;
use rppal::gpio::{Gpio, OutputPin};

const PWD_FREQUENCY: f64 = 60.0;

pub struct LightController {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
}

impl LightController {
    pub fn new(gpio: Gpio, color: Color) -> anyhow::Result<Self> {
        let (red, green, blue) = color.to_tuple();
        Ok(Self {
            red_pin: gpio.get(red)?.into_output(),
            green_pin: gpio.get(green)?.into_output(),
            blue_pin: gpio.get(blue)?.into_output(),
        })
    }

    fn set_channel(pin: &mut OutputPin, value: u8) -> anyhow::Result<()> {
        pin.set_pwm_frequency(PWD_FREQUENCY, value as f64 / 255.0)?;
        Ok(())
    }

    pub fn set_color(&mut self, color: Color) -> anyhow::Result<()> {
        let (red, green, blue) = color.to_tuple();
        Self::set_channel(&mut self.red_pin, red)?;
        Self::set_channel(&mut self.green_pin, green)?;
        Self::set_channel(&mut self.blue_pin, blue)?;
        Ok(())
    }
}
