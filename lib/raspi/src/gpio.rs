use libc;
use model::Model;
use sysfs_gpio;

pub struct GpioBase(*mut u32);

pub enum Gpio {
    SysFsGpio {
        pin_mapping: Option<Vec<usize>>
    }
}

impl Gpio {
    pub fn new() -> Option<Gpio> {
        let model = Model::get();

        Some(Gpio::SysFsGpio {
            pin_mapping: model.pin_mapping()
        })

    }

    pub fn pin(&self, number: usize, direction: Direction) -> Box<dyn Pin> {
        match self {
            &Gpio::SysFsGpio { ref pin_mapping } => {
                let number = pin_mapping.as_ref().and_then(|mapping| {
                    mapping.get(number).map(|num| *num)
                }).unwrap_or(number);
                Box::new(SysFsGpioPin::new(number, direction))
            }
        }
    }
}

impl Drop for GpioBase {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.0 as *mut libc::c_void, 0x1000) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Input,
    Output
}

pub trait Pin {
    fn set_direction(&mut self, direction: Direction);
    fn set(&self, value: bool);
    fn read(&self) -> bool;

    fn set_high(&self) {
        self.set(true);
    }

    fn set_low(&self) {
        self.set(false);
    }
}

pub struct SysFsGpioPin {
    direction: Direction,
    pin: sysfs_gpio::Pin
}

impl SysFsGpioPin {
    pub fn new(number: usize, direction: Direction) -> SysFsGpioPin {
        let mut pin = SysFsGpioPin {
            pin: sysfs_gpio::Pin::new(number as u64),
            direction: direction
        };

        pin.pin.export().expect("Failed to export GPIO pin.");
        pin.set_direction(direction);
        pin
    }
}

impl Pin for SysFsGpioPin {
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;

        let direction = match self.direction {
            Direction::Input => sysfs_gpio::Direction::In,
            Direction::Output => sysfs_gpio::Direction::Out
        };

        self.pin.set_direction(direction).expect("Failed to set GPIO direction.");
    }

    fn set(&self, value: bool) {
        assert_eq!(self.direction, Direction::Output);
        self.pin.set_value(value as u8).ok();
    }

    fn read(&self) -> bool {
        assert_eq!(self.direction, Direction::Input);
        self.pin.get_value().map(|val| val != 0).unwrap_or(false)
    }
}
