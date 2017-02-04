use std::fs::{ File };

pub struct Reader {
    filename: String,
    content: String,
}

impl Reader {
    pub fn new(filename: String) {
        Reader {
            filename: filename,
        }
    }

    pub fn readFile(&mut self) {
        let mut file = try!(File::open(self.filename));

        try!(file.read_to_string(&mut self.content));
    }
}
