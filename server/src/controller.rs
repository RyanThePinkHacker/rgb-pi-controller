use std::default::Default;

use common::Color;
use rppal::gpio::{Gpio, OutputPin};

pub struct LightController {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
    color: Color,
    pwm_frequency: f64,
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
            color: Default::default(),
            pwm_frequency: 60.0,
        })
    }

    pub fn set_color(&mut self, color: Color) -> anyhow::Result<()> {
        self.color = color.clone();
        let (red, green, blue) = color.to_tuple();
        self.red_pin
            .set_pwm_frequency(self.pwm_frequency, red as f64 / 255.0)?;
        self.green_pin
            .set_pwm_frequency(self.pwm_frequency, green as f64 / 255.0)?;
        self.blue_pin
            .set_pwm_frequency(self.pwm_frequency, blue as f64 / 255.0)?;
        Ok(())
    }

    pub fn set_pwm_frequency(&mut self, frequency: f64) {
        self.pwm_frequency = frequency;
    }

    pub fn get_pwm_frequency(&self) -> f64 {
        self.pwm_frequency
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}
