#![allow(dead_code)]

use embassy_stm32::Peri;
use embassy_stm32::gpio::AnyPin;
use embassy_stm32::peripherals::*;

pub struct BargraphPins {
    pub led0: Peri<'static, AnyPin>,
    pub led1: Peri<'static, AnyPin>,
    pub led2: Peri<'static, AnyPin>,
    pub led3: Peri<'static, AnyPin>,
    pub led4: Peri<'static, AnyPin>,
    pub led5: Peri<'static, AnyPin>,
    pub led6: Peri<'static, AnyPin>,
    pub led7: Peri<'static, AnyPin>,
}

pub struct GpsPins {
    pub gps_enn: Peri<'static, AnyPin>,
}

pub struct GpioPins {
    pub ld2: Peri<'static, AnyPin>,
    pub btn: Peri<'static, AnyPin>,
}

pub struct GamepadPins {
    pub top: Peri<'static, AnyPin>,
    pub bottom: Peri<'static, AnyPin>,
    pub right: Peri<'static, AnyPin>,
    pub left: Peri<'static, AnyPin>,
    pub center: Peri<'static, AnyPin>,
}

pub struct MagnetoPins {
    pub status: Peri<'static, AnyPin>,
    pub int: Peri<'static, AnyPin>,
}

pub struct EncoderPins {
    pub button: Peri<'static, AnyPin>,
    pub ch_a:   Peri<'static, PA0>,
    pub ch_b:   Peri<'static, PA1>,
    pub timer:  Peri<'static, TIM2>,
}

pub struct StepperPins {
    pub dir: Peri<'static, AnyPin>,
    pub ms1: Peri<'static, AnyPin>,
    pub ms2: Peri<'static, AnyPin>,
    pub enable: Peri<'static, AnyPin>,
    pub step: Peri<'static, AnyPin>,
}

pub struct Usart1Pins {
    pub tx: Peri<'static, AnyPin>,
    pub rx: Peri<'static, AnyPin>,
    pub usart: Peri<'static, USART1>,
}

pub struct Usart2Pins {
    pub tx: Peri<'static, AnyPin>,
    pub rx: Peri<'static, AnyPin>,
    pub usart: Peri<'static, USART2>,
}

pub struct Spi2Pins {
    pub sck: Peri<'static, AnyPin>,
    pub mosi: Peri<'static, AnyPin>,
    pub miso: Peri<'static, AnyPin>,
    pub cs: Peri<'static, AnyPin>,
    pub spi: Peri<'static, SPI2>,
}

pub struct I2c1Pins {
    pub scl: Peri<'static, PB6>,
    pub sda: Peri<'static, PB7>,
    pub i2c: Peri<'static, I2C1>,
}

pub struct ConnectorPins {
    pub pc10: Peri<'static, AnyPin>,
    pub pc11: Peri<'static, AnyPin>,
    pub pc12: Peri<'static, AnyPin>,
    pub pb8: Peri<'static, AnyPin>,
    pub pb9: Peri<'static, AnyPin>,
    pub pd2: Peri<'static, AnyPin>,
}

pub struct Board {
    pub bargraph: BargraphPins,
    pub gps: GpsPins,
    pub gpio: GpioPins,
    pub gamepad: GamepadPins,
    pub magneto: MagnetoPins,
    pub encoder: EncoderPins,
    pub stepper: StepperPins,
    pub usart1: Usart1Pins,
    pub usart2: Usart2Pins,
    pub spi2: Spi2Pins,
    pub i2c1: I2c1Pins,
    pub connector: ConnectorPins,
}

impl Board {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            bargraph: BargraphPins {
                led0: p.PC7.into(),
                led1: p.PB2.into(),
                led2: p.PA8.into(),
                led3: p.PB1.into(),
                led4: p.PB15.into(),
                led5: p.PB4.into(),
                led6: p.PB14.into(),
                led7: p.PB5.into(),
            },
            gps: GpsPins {
                gps_enn: p.PB13.into(),
            },
            gpio: GpioPins {
                ld2: p.PA5.into(),
                btn: p.PC13.into(),
            },
            gamepad: GamepadPins {
                top: p.PC8.into(),
                bottom: p.PB11.into(),
                right: p.PC9.into(),
                left: p.PC6.into(),
                center: p.PC5.into(),
            },
            magneto: MagnetoPins {
                status: p.PC1.into(),
                int: p.PB0.into(),
            },
            encoder: EncoderPins {
                button: p.PA15.into(),
                ch_a:   p.PA0,
                ch_b:   p.PA1,
                timer:  p.TIM2,
            },
            stepper: StepperPins {
                dir: p.PA7.into(),
                ms1: p.PA11.into(),
                ms2: p.PB12.into(),
                enable: p.PA12.into(),
                step: p.PA6.into(),
            },
            usart1: Usart1Pins {
                tx: p.PA9.into(),
                rx: p.PA10.into(),
                usart: p.USART1,
            },
            usart2: Usart2Pins {
                tx: p.PA2.into(),
                rx: p.PA3.into(),
                usart: p.USART2,
            },
            spi2: Spi2Pins {
                sck: p.PB10.into(),
                mosi: p.PC3.into(),
                miso: p.PC2.into(),
                cs: p.PC0.into(),
                spi: p.SPI2,
            },
            i2c1: I2c1Pins {
                scl: p.PB6,
                sda: p.PB7,
                i2c: p.I2C1,
            },
            connector: ConnectorPins {
                pc10: p.PC10.into(),
                pc11: p.PC11.into(),
                pc12: p.PC12.into(),
                pb8: p.PB8.into(),
                pb9: p.PB9.into(),
                pd2: p.PD2.into(),
            },
        }
    }
}
