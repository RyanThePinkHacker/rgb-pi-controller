use std::default::Default;

use common::Color;
use rppal::gpio::{Gpio, OutputPin};

const PWD_FREQUENCY: f64 = 60.0;

pub struct LightController {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
    current_color: Color,
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
            current_color: Default::default(),
        })
    }

    fn set_channel(pin: &mut OutputPin, value: u8) -> anyhow::Result<()> {
        pin.set_pwm_frequency(PWD_FREQUENCY, value as f64 / 255.0)?;
        Ok(())
    }

    pub fn set_color(&mut self, color: Color) -> anyhow::Result<()> {
        let (red, green, blue) = color.clone().to_tuple();
        self.current_color = color;
        Self::set_channel(&mut self.red_pin, red)?;
        Self::set_channel(&mut self.green_pin, green)?;
        Self::set_channel(&mut self.blue_pin, blue)?;
        Ok(())
    }

    pub fn get_color(&self) -> &Color {
        &self.current_color
    }
}
