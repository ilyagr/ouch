use std::path::PathBuf;

use crate::{error::OuchResult, file::File};

pub enum CompressionResult {
    ZipArchive(Vec<u8>),
    TarArchive(Vec<u8>),
    FileInMemory(Vec<u8>)
}

pub trait Compressor {
    fn compress(&self, from: Vec<File>) -> OuchResult<CompressionResult>;
}

// 
//
//
//
//
//
//
//
//