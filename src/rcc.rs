#[repr(C)]
pub struct RCC_t {
    pub CR: u32,      /* Clock control register,                  Address offset 0x00 */
    pub CFGR: u32,    /* Clock configuration register,            Address offset 0x04 */
    pub CIR: u32,     /* Clock interrupt register,                Address offset 0x08 */
    pub APB2RSTR: u32,/* APB peripheral reset register 2,         Address offset 0x0c */
    pub APB1RSTR: u32,/* APB peripheral reset register 1,         Address offset 0x10 */
    pub AHBENR: u32,  /* AHB peripheral clock enable register,    Address offset 0x14 */
    pub APB2ENR: u32, /* APB peripheral clock enable register 2,  Address offset 0x18 */
    pub APB1ENR: u32, /* APB peripheral clock enable register 1,  Address offset 0x1c */
    pub BDCR: u32,    /* RTC domain control register,             Address offset 0x20 */
    pub CSR: u32,     /* Control/status register,                 Address offset 0x24 */
    pub AHBRSTR: u32, /* AHB peripheral reset register,           Address offset 0x28 */
    pub CFGR2: u32,   /* Clock configuration register 2,          Address offset 0x2c */
    pub CFGR3: u32,   /* Clock configuration register 3,          Address offset 0x30 */
    pub CR2: u32,     /* Clock control register 2,                Address offset 0x34 */
}

const RCC: *mut RCC_t = 0x40021000 as *mut RCC_t;

pub enum IoPort {
    A = 17,
    B,
    C,
    D,
    E,
    F,
}

pub enum UartPort {
    UartPort1 = 14,
    UartPort2 = 17,
    UartPort3 = 18,
    UartPort4 = 19,
    UartPort5 = 20,
    UartPort6 = 5,
    UartPort7 = 6,
    UartPort8 = 7,
}

pub enum RccTim {
    RccTim1 = 11,
    RccTim2 = 0,
    RccTim3 = 1,
    RccTim6 = 4,
    RccTim7 = 5,
    RccTim14 = 8,
    RccTim15 = 16,
    RccTim16 = 17,
    RccTim17 = 18,
}

pub const RCC_BASE: u32 = 0x4002_1000;
pub const AHBENR_OFFSET: u32 = 0x14;
pub const APB1ENR_OFFSET: u32 = 0x1C;
pub const APB2ENR_OFFSET: u32 = 0x18;
pub const CR_OFFSET: u32 = 0x00;
pub const CFGR_OFFSET: u32 = 0x04;
pub const CFGR3_OFFSET: u32 = 0x30;
pub const HSION_OFFSET: u32 = 0;
pub const HSITRIM_OFFSET: u32 = 3;
pub const HPRE_OFFSET: u32 = 4;
pub const PPRE_OFFSET: u32 = 8;
pub const USART2SW_OFFSET: u32 = 16;

pub fn rcc_init_clocks() {
    // Enable HSI oscillator
    unsafe {
        (*RCC).CR = set_bits_with_offset((*RCC).CR, HSION_OFFSET, 1, 1);
    }

    // Set PCLK and HCLK prescaler values
    unsafe {
        (*RCC).CFGR = set_bits_with_offset((*RCC).CFGR, PPRE_OFFSET, 3, 7);
        (*RCC).CFGR = set_bits_with_offset((*RCC).CFGR, HPRE_OFFSET, 4, 0);
    }

    // Set PCLK as USART2 clock
    unsafe {
        (*RCC).CFGR3 = set_bits_with_offset((*RCC).CFGR3, USART2SW_OFFSET, 2, 0);
    }
}


pub fn rcc_gpio_set(io_port: IoPort, enable: bool) {
    let offset = io_port as u32;
    unsafe {
        (*RCC).AHBENR = set_bits_with_offset((*RCC).AHBENR, offset, 1, enable as u32);
    }
}


pub fn rcc_uart_set(uart_port: UartPort, enable: bool) {
    match uart_port {
        UartPort::UartPort1 | UartPort::UartPort6 | UartPort::UartPort7 | UartPort::UartPort8 => {
            unsafe {
                (*RCC).APB2ENR = set_bits_with_offset((*RCC).APB2ENR, uart_port as u32, 1, enable as u32);
            }
        }
        UartPort::UartPort2 | UartPort::UartPort3 | UartPort::UartPort4 | UartPort::UartPort5 => {
            unsafe {
                (*RCC).APB1ENR = set_bits_with_offset((*RCC).APB1ENR, uart_port as u32, 1, enable as u32);
            }
        }
    }
}

pub fn rcc_tim_set(tim: RccTim, enable: bool) {
    match tim {
        RccTim::RccTim2 | RccTim::RccTim3 | RccTim::RccTim6 | RccTim::RccTim7 | RccTim::RccTim14 => {
            unsafe {
                (*RCC).APB1ENR = set_bits_with_offset((*RCC).APB1ENR, tim as u32, 1, enable as u32);
            }
        }
        RccTim::RccTim1 | RccTim::RccTim15 | RccTim::RccTim16 | RccTim::RccTim17 => {
            unsafe {
                (*RCC).APB2ENR = set_bits_with_offset((*RCC).APB2ENR, tim as u32, 1, enable as u32);
            }
        }
    }
}

pub fn set_bits_with_offset(field: u32, offset: u32, width: u32, value: u32) -> u32 {
    let mask = (1 << width) - 1;
    let temp = field & !(mask << offset);
    temp | (value << offset)
}
