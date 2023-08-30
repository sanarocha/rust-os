// enum que especifica o numero de cada cor
// por causa do repr(u8) -> cada variavel é armazenada em u8
// allow(dead_code) desabilita warning pra cada variavel nao usada
// derive -> printable e comparable
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// contem o byte completo da cor (foreground e background)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// garante que os fields da struct serao exatamente como uma struct em C -> garante ordem correta
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// repr(transparent) garente que vai ter o mesmo layout de memoria
use volatile::Volatile;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// para escrever na tela
pub struct Writer {
    column_position: usize, // mantem qual foi a última posição na última linha
    color_code: ColorCode, // cores do foreground e background, referencia do VGA buffer armazenada no buffer
    buffer: &'static mut Buffer, // necessario deixar explicito o tempo de vida da referencia
    // static lifetime -> referencia é valida por toda a execucao do programa
}

impl Writer {
     // ao printar o byte -> writer olha se a linha atual esta cheia
     // se sim -> chama método new_line
     // entao, escreve um novo ScreenChar -> coluna da current position avança
     pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// vga text buffer só suporta ascii
// strings rust são utf-8, entao podem conter bytes que não são suportados pelo VGA text buffer
// usando o match byte diferenciamos ascii printáveis de não printáveis
// caso for não printável, é colocado um ■ (0xfe)
impl Writer {
    // converte cada parte da string em byte e escreve um a um 
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII byte printável ou nova linha
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // não é parte do escopo printável do ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// função temporaria de teste
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}