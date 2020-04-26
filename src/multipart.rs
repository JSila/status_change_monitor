use bytebuffer::ByteBuffer;
use rand::seq::SliceRandom;
use std::io::Cursor;

pub struct MultiPart {
    pub boundary: String,
    buffer: ByteBuffer,
    before_flag: bool,
}

impl MultiPart {
    pub fn new() -> Self {
        let mut rng = &mut rand::thread_rng();
        let postfix = String::from_utf8(
            "-_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                .as_bytes()
                .choose_multiple(&mut rng, 30)
                .cloned()
                .collect(),
        )
            .unwrap();
        Self {
            boundary: format!("---------------------------{}", postfix),
            buffer: ByteBuffer::new(),
            before_flag: true,
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        if self.before_flag {
            self.write_boundary(true);
            self.before_flag = false;
        }
        self.buffer.to_bytes()
    }

    pub fn add_string(&mut self, name: &str, data: &str) {
        self.write_boundary(false);
        self.write_header(name);
        self.buffer.write_bytes(&format!("{}\r\n", data).as_bytes());
    }

    pub fn add_cursor(&mut self, name: &str, cursor: Cursor<Vec<u8>>) {
        self.write_boundary(false);
        self.write_header(name);
        self.buffer.write_bytes(cursor.get_ref());
        self.buffer.write_bytes("\r\n".as_bytes());
    }

    fn write_header(&mut self, name: &str) {
        self.buffer.write_bytes(
            &format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name).as_bytes(),
        );
    }

    fn write_boundary(&mut self, last_flag: bool) {
        self.buffer.write_bytes("--".as_bytes());
        self.buffer.write_bytes(&self.boundary.as_bytes());
        if last_flag {
            self.buffer.write_bytes("--".as_bytes());
        } else {
            self.buffer.write_bytes("\r\n".as_bytes());
        }
    }
}