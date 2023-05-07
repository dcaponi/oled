use embedded_hal::blocking::i2c::{Write, WriteRead};

const SSD1306_ADDR: u8 = 0x3C;
const WIDTH: u8 = 128;
const HEIGHT: u8 = 64;

pub const FONT_DATA: [[u8; 8]; 3] = [
    [0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000],
    [0b00000000, 0b00001000, 0b00011100, 0b00001000, 0b11111111, 0b00001000, 0b00011100, 0b00001000],
    [0b10000001, 0b01000010, 0b00100100, 0b00011000, 0b00011000, 0b00100100, 0b01000010, 0b10000001]
];

pub(crate) struct Ssd1306<I2C> {
    address: u8,
    i2c: I2C
}

impl<I2C0, E> Ssd1306<I2C0>
where I2C0: Write<Error = E> + WriteRead<Error = E> {
    pub fn new(i2c: I2C0, address: u8) -> Self {
        Self {
            i2c, 
            address
        }
    }

    pub fn write_command(&mut self, command: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, command])?;
        Ok(())
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<(), E> {
        let mut buf = [0; 17];
        for chunk in data.chunks(16) {
            buf[0] = 0x40; // set data start
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i+1] = byte;
            }
            self.i2c.write(self.address, &buf[..=chunk.len()])?;
        }
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), E> {
        for _ in 0..(16 * 8) {
            let bmp = FONT_DATA[0 as usize];
            self.write_data(&bmp)?;
        }
        self.zero_cursor()?;
        Ok(())
    }

    pub fn write_text(&mut self, text: &str) -> Result<(), E> {
        for c in text.chars() {
            let bmp = FONT_DATA[c as usize];
            self.write_data(&bmp)?;
        }
        Ok(())
    }

    pub fn draw_checkerboard(&mut self) -> Result<(), E> {
        for i in 0..(16 * 8) {
            let bmp = FONT_DATA[(i % 2)];
            self.write_data(&bmp)?;
        }
        Ok(())
    }

    // sets cursor to top left
    fn zero_cursor(&mut self) -> Result<(), E> {
        self.write_command(0x21)?;  // Set column address range to 0-95
        self.write_command(0x00)?;  // Set column start address to 0
        self.write_command(0x7F)?;  // Set column end address to 95
        self.write_command(0x22)?;  // Set page address range to 0-7
        self.write_command(0x00)?;  // Set page start address to 0
        self.write_command(0x07)?;  // Set page end address to 7
        Ok(())
    }

    pub fn setup(&mut self) -> Result<(), E> {
        self.write_command(0xAE)?; // display off

        self.write_command(0xD5)?; // set display clock divide ratio/oscillator frequency
        self.write_command(0x80)?; // the suggested ratio 0x80

        self.write_command(0xA8)?; // set multiplex
        self.write_command(0x3F)?; // 63 COM lines

        self.write_command(0xD3)?; // set display offset
        self.write_command(0x00)?; // no offset

        self.zero_cursor()?;

        self.write_command(0x8D)?; // charge pump setting
        self.write_command(0x14)?; // enable charge pump
        
        self.write_command(0x20)?; // memory mode
        self.write_command(0x00)?; // horizontal addressing mode

        self.write_command(0xA1)?; // segment remap

        self.write_command(0xC8)?; // COM scan direction

        self.write_command(0xDA)?; // set COM pins
        self.write_command(0x12)?; // alternative configuration

        self.write_command(0x81)?; // set contrast control
        self.write_command(0xCF)?; // 0-255

        self.write_command(0xD9)?; // set pre-charge period
        self.write_command(0xF1)?; // suggested by datasheet

        self.write_command(0xDB)?; // set VCOMH deselect level
        self.write_command(0x40)?; // 0.77xVcc

        self.write_command(0xA4)?;
        self.write_command(0xA6)?;
        self.write_command(0xAF)?; // display on

        self.clear_screen()?;
        Ok(())
    }
}
