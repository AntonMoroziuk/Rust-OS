#[repr(C)]
pub struct UART_t {
    pub CR1: volatile u32,   /* Control register 1,                  Address offset 0x00 */
    pub CR2: volatile u32,   /* Control register 2,                  Address offset 0x04 */
    pub CR3: volatile u32,   /* Control register 3,                  Address offset 0x08 */
    pub BRR: volatile u32,   /* Baud rate register,                  Address offset 0x0c */
    pub GTPR: volatile u32,  /* Guard time and prescaler register,   Address offset 0x10 */
    pub RTOR: volatile u32,  /* Receiver timeout register,           Address offset 0x14 */
    pub RQR: volatile u32,   /* Request register,                    Address offset 0x18 */
    pub ISR: volatile u32,   /* Interrupt and status register,       Address offset 0x1c */
    pub ICR: volatile u32,   /* Interrupt flag clear register,       Address offset 0x20 */
    pub RDR: volatile u32,   /* Receive data register,               Address offset 0x24 */
    pub TDR: volatile u32,   /* Transmit data register,              Address offset 0x28 */
}

pub struct Writer<'a> {
    pub write: fn(&Writer<'a>, &[u8]),
    pub data: &'a mut dyn Any,
}

pub enum WORD_LENGTH {
    EIGHT_BITS,
    NINE_BITS,
    SEVEN_BITS,
}

pub enum STOP_BITS {
    ONE_BIT,
    HALF_BIT,
    TWO_BITS,
    ONE_AND_HALF_BIT,
}

pub const DEFAULT_BRR_VALUE: u32 = 0x34;

#[repr(C)]
pub struct UART_config {
    pub word_length: WORD_LENGTH,
    pub baud_rate: u32,
    pub stop_bits: STOP_BITS,
}

// UART objects
pub const UART1: *mut UART_t = 0x4001_3800 as *mut UART_t;
pub const UART2: *mut UART_t = 0x4000_4400 as *mut UART_t;
pub const UART3: *mut UART_t = 0x4000_4800 as *mut UART_t;
pub const UART4: *mut UART_t = 0x4000_4C00 as *mut UART_t;
pub const UART5: *mut UART_t = 0x4000_5000 as *mut UART_t;
pub const UART6: *mut UART_t = 0x4001_1400 as *mut UART_t;
pub const UART7: *mut UART_t = 0x4001_1800 as *mut UART_t;
pub const UART8: *mut UART_t = 0x4001_1C00 as *mut UART_t;

pub const UE_OFFSET: u8 = 0;
pub const RE_OFFSET: u8 = 2;
pub const TE_OFFSET: u8 = 3;
pub const M0_OFFSET: u8 = 12;
pub const M1_OFFSET: u8 = 28;

/* CR2 register bit offsets */
pub const STOP_OFFSET: u8 = 12;

/* ISR register bit offsets */
pub const RXNE_OFFSET: u8 = 5;
pub const TC_OFFSET: u8 = 6;
pub const TXE_OFFSET: u8 = 7;

pub fn uart_configure(uart: *mut UART_t, config: *mut UART_config) {
    // TODO: add all UART ports
    if uart == UART1 {
        // Clock
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_8 }, AF1);

        // RX
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_9 }, AF1);

        // TX
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_10 }, AF1);
    } else if uart == UART2 {
        // TX
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_2 }, AF1);

        // RX
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_3 }, AF1);

        // Clock
        gpio_select_alternate_function(GPIO_pin{ port: GPIOA, pin: GPIO_PIN_4 }, AF1);
    }

    /* Disable UART */
    set_bits_with_offset(&mut (*uart).CR1, UE_OFFSET, 1, 0);

    match unsafe { *config }.word_length {
        EIGHT_BITS => {
            set_bits_with_offset(&mut (*uart).CR1, M1_OFFSET, 1, 0);
            set_bits_with_offset(&mut (*uart).CR1, M0_OFFSET, 1, 0);
        }
        NINE_BITS => {
            set_bits_with_offset(&mut (*uart).CR1, M1_OFFSET, 1, 0);
            set_bits_with_offset(&mut (*uart).CR1, M0_OFFSET, 1, 1);
        }
        SEVEN_BITS => {
            set_bits_with_offset(&mut (*uart).CR1, M1_OFFSET, 1, 1);
            set_bits_with_offset(&mut (*uart).CR1, M0_OFFSET, 1, 0);
        }
    }

    unsafe { (*uart).BRR = DEFAULT_BRR_VALUE };

    set_bits_with_offset(&mut (*uart).CR2, STOP_OFFSET, 2, unsafe { (*config).stop_bits });

    /* Enable both receive and transmit */
    set_bits_with_offset(&mut (*uart).CR1, RE_OFFSET, 1, 1);
    set_bits_with_offset(&mut (*uart).CR1, TE_OFFSET, 1, 1);

    /* Enable UART */
    set_bits_with_offset(&mut (*uart).CR1, UE_OFFSET, 1, 1);
}

pub fn uart_write(uart: *mut UART_t, buf: &[u8]) {
    for &byte in buf {
        /* Wait until data is transfered to the shift register */
        while unsafe { (*uart).ISR } & (1 << TXE_OFFSET) == 0 {}

        unsafe { (*uart).TDR = byte };
    }

    /* Wait until data is sent */
    while unsafe { (*uart).ISR } & (1 << TC_OFFSET) == 0 {}
}

pub fn uart_read(uart: *mut UART_t, buf: &mut [u8]) {
    for i in 0..buf.len() {
        while unsafe { (*uart).ISR } & (1 << RXNE_OFFSET) == 0 {}
        buf[i] = unsafe { (*uart).RDR };
    }
}

pub fn writer_uart_write(self_: &Writer, buf: &[u8]) {
    let uart = self_.data as *mut UART_t;
    uart_write(uart, buf, buf.len());
}

pub fn uart_writer(uart: *mut UART_t) -> *mut Writer {
    let mut res = Box::new(Writer {
        write: writer_uart_write,
        data: uart as *mut c_void,
    });
    let ptr = &mut *res as *mut Writer;
    Box::into_raw(res);
    ptr
}

unsafe fn uart_delete_writer(to_delete: *mut Writer) {
    Box::from_raw(to_delete);
}

