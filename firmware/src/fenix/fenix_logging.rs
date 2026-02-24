use crate::fenix::fenix_sd::{
    FenixDirectory,
    FenixFile,
};

pub struct FenixLogger<'a> {
    _file: FenixFile<'a>,
}

impl<'a> FenixLogger<'a> {
    pub fn new(dir: &'a FenixDirectory, fname: &str) -> Self {
        let file = dir
            .open_file_in_dir(fname, embedded_sdmmc::Mode::ReadWriteCreateOrAppend)
            .unwrap();

        file.write(b"Opened SD\n").unwrap();
        Self { _file: file }
    }
}
