#[repr(C)]
pub struct GPIO_s {
    MODER: u32,
    OTYPER: u32,
    OSPEEDR: u32,
    PUPDR: u32,
    IDR: u32,
    ODR: u32,
    BSRR: u32,
    LCKR: u32,
    AFRL: u32,
    AFRH: u32,
    BRR: u32,
}

#[repr(C)]
pub struct GPIO_config_s {
    pub mode: u32,
    pub pin: u32,
    pub pull: u32,
    pub speed: u32,
    pub alternate: u32,
}

#[repr(C)]
pub struct GPIO_pin_s {
    pub port: *mut GPIO_s,
    pub pin: u32,
}

pub const GPIO_MODE_INPUT: u32 = 0x00000000;
pub const GPIO_MODE_OUTPUT: u32 = 0x00000001;
pub const GPIO_MODE_ALTERNATE_FUNC: u32 = 0x00000002;
pub const GPIO_MODE_ANALOG: u32 = 0x00000003;

pub const GPIO_TYPE_PUSH_PULL: u32 = 0x00000000;
pub const GPIO_TYPE_OPEN_DRAIN: u32 = 0x00000001;

pub const GPIO_SPEED_LOW: u32 = 0x00000000;
pub const GPIO_SPEED_MEDIUM: u32 = 0x00000001;
pub const GPIO_SPEED_HIGH: u32 = 0x00000003;

pub const GPIO_NO_PUPD: u32 = 0x00000000;
pub const GPIO_PULL_UP: u32 = 0x00000001;
pub const GPIO_PULL_DOWN: u32 = 0x00000002;

pub const GPIOA: *mut GPIO_s = 0x48000000 as *mut GPIO_s;
pub const GPIOB: *mut GPIO_s = 0x48000400 as *mut GPIO_s;
pub const GPIOC: *mut GPIO_s = 0x48000800 as *mut GPIO_s;
pub const GPIOD: *mut GPIO_s = 0x48000C00 as *mut GPIO_s;
pub const GPIOE: *mut GPIO_s = 0x48001000 as *mut GPIO_s;
pub const GPIOF: *mut GPIO_s = 0x48001400 as *mut GPIO_s;

pub const GPIO_PIN_0: u32 = 0x0000_0000;
pub const GPIO_PIN_1: u32 = 0x0000_0001;
pub const GPIO_PIN_2: u32 = 0x0000_0002;
pub const GPIO_PIN_3: u32 = 0x0000_0003;
pub const GPIO_PIN_4: u32 = 0x0000_0004;
pub const GPIO_PIN_5: u32 = 0x0000_0005;
pub const GPIO_PIN_6: u32 = 0x0000_0006;
pub const GPIO_PIN_7: u32 = 0x0000_0007;
pub const GPIO_PIN_8: u32 = 0x0000_0008;
pub const GPIO_PIN_9: u32 = 0x0000_0009;
pub const GPIO_PIN_10: u32 = 0x0000_000A;
pub const GPIO_PIN_11: u32 = 0x0000_000B;
pub const GPIO_PIN_12: u32 = 0x0000_000C;

pub const AF0: u32 = 0x0;
pub const AF1: u32 = 0x1;
pub const AF2: u32 = 0x2;
pub const AF3: u32 = 0x3;
pub const AF4: u32 = 0x4;
pub const AF5: u32 = 0x5;
pub const AF6: u32 = 0x6;
pub const AF7: u32 = 0x7;

pub fn gpio_init(GPIOx: *mut GPIO_s, GPIO_init: &GPIO_config_s) {
    unsafe {
        (*GPIOx).MODER = set_bits_with_offset((*GPIOx).MODER, GPIO_init.pin * 2u32, 2, GPIO_init.mode);
        (*GPIOx).OSPEEDR = set_bits_with_offset((*GPIOx).OSPEEDR, GPIO_init.pin * 2u32, 2, GPIO_init.speed);
        (*GPIOx).PUPDR = set_bits_with_offset((*GPIOx).PUPDR, GPIO_init.pin * 2u32, 2, GPIO_init.pull);
    }
}

pub fn gpio_set(gpio: GPIO_pin_s, value: u8) {
    if value != 0 {
        unsafe {
            (*gpio.port).BSRR = 1 << gpio.pin;
        }
    } else {
        unsafe {
            (*gpio.port).BSRR = 1 << (gpio.pin + 16);
        }
    }
}

pub fn gpio_read(gpio: GPIO_pin_s) -> i32 {
    unsafe {
        if ((*gpio.port).IDR & (1 << gpio.pin)) > 0 {
            return 1;
        }
    }
    return 0;
}

pub fn gpio_select_alternate_function(gpio: GPIO_pin_s, function: u8) {
    let offset = (gpio.pin % 8) * 4;

    if gpio.pin <= 7 {
        unsafe {
            (*gpio.port).AFRL = set_bits_with_offset((*gpio.port).AFRL, offset, 4, function.into());
        }
    } else {
        unsafe {
            (*gpio.port).AFRH = set_bits_with_offset((*gpio.port).AFRH, offset, 4, function.into());
        }
    }
    unsafe {
        (*gpio.port).MODER = set_bits_with_offset((*gpio.port).MODER, gpio.pin * 2u32, 2, GPIO_MODE_ALTERNATE_FUNC);
    }
}

pub fn set_bits_with_offset(field: u32, offset: u32, width: u32, value: u32) -> u32 {
    let mask = (1 << width) - 1;
    let temp = field & !(mask << offset);
    temp | (value << offset)
}
