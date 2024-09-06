use embassy_stm32::gpio::Pin;
use embassy_stm32::adc::{AdcPin, Resolution, Adc, Instance};
use libm::logf;

const B_COEFFICIENT: f32 = 3950.0;
const T0_KELVIN: f32 = 298.15; // room temp in Kelvin
const R_AT_T0: f32 = 10_000.0; // resistance of the thermistor at room temp
const R_DIVIDER_FIXED: f32 = 10_000.0; // resistance of the fixed resistor in the voltage divider


pub struct Thermistor<'a, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> {

    adc_pin: AdcReadPin,
    adc: Adc<'a, AdcInstance>,
    resolution: Resolution

}

impl<'a, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> Thermistor<'a, AdcReadPin, AdcInstance> {
    pub fn new(adc_pin: AdcReadPin, mut adc: Adc<'a, AdcInstance>) -> Self {
        
        let resolution = Resolution::TwelveBit;
        adc.set_resolution(resolution);
        
        Thermistor {
            adc_pin,
            adc,
            resolution
        }
    }

    fn get_resistance(&mut self) -> f32 {
        let adc_reading = self.adc.read(&mut self.adc_pin) as f32;
        let max_count = self.resolution.to_max_count() as f32;
        R_DIVIDER_FIXED / (max_count / adc_reading - 1.0)
    }

    pub fn temp_c(&mut self) -> f32 {
        let resistance = self.get_resistance();
        let temperature = 1.0 / (1.0 / T0_KELVIN + 1.0 / B_COEFFICIENT * logf(resistance / R_AT_T0));
        temperature - 273.15
    }
}