use std::io::{Read, Seek};
use std::fs::File;
use std::fmt::Display;
use std::slice::SliceIndex;
use std::string::FromUtf8Error;
use memmap2::Mmap;

/// Inidividual element type
/// 
///     // Vector of bytes
///     pub val: Vec<u8>, 
///     // Byte offset in file
///     pub offset: u64,
///     // String representation of field
///     pub string: String,
///     // Length of byte vector
///     length: usize,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfField{
    /// Vector of bytes
    pub bytes: Vec<u8>, 
    /// Byte offset in file
    pub offset: u64,
    /// String representation of field
    pub string: String,
    /// Length of byte vector
    length: usize,
}
impl NitfField{
    pub fn read(&mut self, reader: &mut (impl Read + Seek), n_bytes: usize) {
        self.length = n_bytes;
        for _ in 0..n_bytes {
            self.bytes.push(0u8)
        }
        self.offset = reader.stream_position().unwrap();    
        reader.read(&mut self.bytes).unwrap();
        let result = String::from_utf8(self.bytes.to_vec());
        match result {
            Ok(str) => self.string = str,
            Err(err) => {
                self.string = String::from("Error parsing string");
                println!("{}", err)
            }
        }
    }
}
impl Display for NitfField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.string)
    }
}


/// Element vector type
/// 
///     // Vector of fields
///     pub val: Vec<NitfField>,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfFieldVec {
    /// Vector of fields
    pub val: Vec<NitfField>,
}
impl NitfFieldVec{
    pub fn read_vec(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        n_field: &NitfField,
        n_bytes: usize) {
        let n_elem_str = String::from_utf8(n_field.bytes.to_vec()).unwrap();
        let n_elem: usize = match n_elem_str.parse() {
            Ok(uint) => uint,
            Err(e) => panic!("{}: {}", e, n_field),
        };  
        for _ in 0..n_elem {
            let mut elem = NitfField::default();
            elem.read(reader, n_bytes);
            self.val.push(elem);
        }
    }
}
impl Display for NitfFieldVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("{}, ", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}


/// Subheader element type
/// 
/// Used within the NITF header to denote the subheader segments contained in the file
/// 
///     /// Bytes of header description
///     pub subheader_size: Vec<u8>,
///     /// Bytes of the data
///     pub item_size: Vec<u8>,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeader {
    /// Bytes of header description
    pub subheader_size: NitfField,
    /// Bytes of the data
    pub item_size: NitfField,
}
impl NitfSubHeader {
    pub fn read(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        sh_size: usize,
        item_size: usize) {
        self.subheader_size.read(reader, sh_size);
        self.item_size.read(reader, item_size);
    }
}
impl Display for NitfSubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[{}, {}]", &self.subheader_size.string, &self.item_size.string)
    }
}


/// Subheader vector type
/// 
///     /// Vector of subheader info
///     pub val: Vec<NitfSubHeader>, 
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeaderVec { 
    /// Vector of subheader info
    pub val: Vec<NitfSubHeader>, 
}
impl NitfSubHeaderVec{
    pub fn read(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        n_subheader: &NitfField,
        sh_size: usize,
        item_size: usize) {
        let n_seg: usize = n_subheader.string.parse().unwrap();    
        for _ in 0..n_seg {
            let mut seg = NitfSubHeader::default();
            seg.read(reader, sh_size, item_size);
            self.val.push(seg);
        }
    }
}
impl Display for NitfSubHeaderVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}


/// Nitf segment header interface definition
/// 
/// Provide implementation for `read()`, `from_reader` defined automatically.
pub trait NitfSegmentHeader where Self: Sized + Default {
    
    /// Read the segment info from stream
    /// 
    /// # Parameters
    /// 
    /// reader: Stream from which to read header information
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {todo!("Didn't implement read() method")}
    
    fn from_reader(reader: &mut (impl Read + Seek)) -> Self {
        let mut hdr = Self::default();
        hdr.read(reader);
        return hdr
    }
}

/// Nitf segment data interface definition
pub trait NitfSegmentData where Self: Sized {
    fn read(&mut self, reader: &mut (impl Read + Seek));
}

impl NitfSegmentData for Vec<u8> {
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!()
    }
}
impl NitfSegmentData for Mmap {
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!()
    }
}

/// Segment structure definition
/// 
///     // Header metadata fields defined in module
///     pub meta: T
///     // Segment data
///     data: Vec<u8>
///     // Byte offset of header start
///     pub header_offset: u64
///     // Byte size of header
///     pub header_size: usize
///     // Data byte offset
///     pub data_offset: u64
///     // Data size in bytes
///     pub data_size: usize
#[derive(Default, Debug)]
pub struct Segment<T, U> {
    /// Header fields defined in module
    pub meta: T,
    /// Segment data, must define function interface for access
    data: U,
    /// Byte offset of header start
    pub header_offset: u64,
    /// Byte size of header
    pub header_size: usize,
    /// Data byte offset
    pub data_offset: u64,
    /// Data size in bytes
    pub data_size: usize,
} 
impl<T, U> Display for Segment<T, U> 
where 
    T: NitfSegmentHeader + Display,
    U: NitfSegmentData
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
} 
impl<T> Segment<T, Vec<u8>> 
where 
    T: NitfSegmentHeader + Default,
{
    pub fn from_reader(reader: &mut (impl Read + Seek), header_size: usize, data_size: usize,) -> Result<Self, FromUtf8Error> {
        let mut seg = Self::default();
        seg.header_size = header_size;
        seg.data_size = data_size;
        seg.header_offset = reader.stream_position().unwrap();
        seg.data_offset = seg.header_offset + (header_size as u64);

        seg.meta.read(reader);
        // seg.data = 
        Ok(seg)
    }
    pub fn read(&mut self, reader: &mut (impl Read + Seek), header_size: usize, data_size: usize) {
        self.header_size = header_size;
        self.data_size = data_size;
        self.header_offset = reader.stream_position().unwrap();
        self.data_offset = self.header_offset + (header_size as u64);
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = (reader.stream_position().unwrap() - self.header_offset) as usize; 
        }
    }
}
impl<T> Segment<T, Mmap> 
where 
    T: NitfSegmentHeader + Default,
{  
    pub fn from_file(reader: &mut File, header_size: usize, data_size: usize,) -> Result<Self, FromUtf8Error> {
        let header_offset = reader.stream_position().unwrap();
        let data_offset = header_offset + (header_size as u64);
        let seg = Self { 
            meta: T::from_reader(reader), 
            data: Self::make_mmap(reader), 
            header_offset, 
            header_size, 
            data_offset, 
            data_size, 
        };
        return Ok(seg)
    }
    pub fn read(&mut self, reader: &mut (impl Read + Seek), header_size: usize, data_size: usize) {
        self.header_size = header_size;
        self.data_size = data_size;
        self.header_offset = reader.stream_position().unwrap();
        self.data_offset = self.header_offset + (header_size as u64);
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = (reader.stream_position().unwrap() - self.header_offset) as usize; 
            self.data_offset = reader.stream_position().unwrap(); 
        }
    }
    fn make_mmap(file: &File) -> Mmap {
        unsafe {Mmap::map(file).unwrap()}
    }
    pub fn read_data(&self) -> Vec<u8> {
        let data_start = self.data_offset as usize;
        let data_end = self.data_size + data_start;
        let data_range = data_start..data_end;
        self.data[data_range].to_vec()
    }
}

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Security {
    /// File Security Classification
    pub CLAS: NitfField,
    /// File Classification Security System
    pub CLSY: NitfField,
    /// File Codewords
    pub CODE: NitfField,
    /// File Control and Handling
    pub CTLH: NitfField,
    /// File Releasing Instructions
    pub REL: NitfField,
    /// File Declassification Type
    pub DCTP: NitfField,
    /// File Declassification Date
    pub DCDT: NitfField,
    /// File Declassification Exemption
    pub DCXM: NitfField,
    /// File Downgrade
    pub DG: NitfField,
    /// File Downgrade Date
    pub DGDT: NitfField,
    /// File Classification Text
    pub CLTX: NitfField,
    /// File Classification Authority Type
    pub CATP: NitfField,
    /// File Classification Authority
    pub CAUT: NitfField,
    /// File Classification Reason
    pub CRSN: NitfField,
    /// File Security Source Date
    pub SRDT: NitfField,
    /// File Security Control Number
    pub CTLN: NitfField,
}
impl Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("CLAS: {}, ", self.CLAS).as_ref();
        out_str += format!("CLSY: {}, ", self.CLSY).as_ref();
        out_str += format!("CODE: {}, ", self.CODE).as_ref();
        out_str += format!("CTLH: {}, ", self.CTLH).as_ref();
        out_str += format!("REL: {}, ", self.REL).as_ref();
        out_str += format!("DCTP: {}, ", self.DCTP).as_ref();
        out_str += format!("DCDT: {}, ", self.DCDT).as_ref();
        out_str += format!("DCXM: {}, ", self.DCXM).as_ref();
        out_str += format!("DG: {}, ", self.DG).as_ref();
        out_str += format!("DGDT: {}, ", self.DGDT).as_ref();
        out_str += format!("CLTX: {}, ", self.CLTX).as_ref();
        out_str += format!("CATP: {}, ", self.CATP).as_ref();
        out_str += format!("CAUT: {}, ", self.CAUT).as_ref();
        out_str += format!("CRSN: {}, ", self.CRSN).as_ref();
        out_str += format!("SRDT: {}, ", self.SRDT).as_ref();
        out_str += format!("CTLN: {}", self.CTLN).as_ref();
        return write!(f, "{}", out_str)
    }
}
impl Security {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.CLAS.read(reader, 1);
        self.CLSY.read(reader, 2);
        self.CODE.read(reader, 11);
        self.CTLH.read(reader, 2);
        self.REL.read(reader, 20);
        self.DCTP.read(reader, 2);
        self.DCDT.read(reader, 8);
        self.DCXM.read(reader, 4);
        self.DG.read(reader, 1);
        self.DGDT.read(reader, 8);
        self.CLTX.read(reader, 43);
        self.CATP.read(reader, 1);
        self.CAUT.read(reader, 40);
        self.CRSN.read(reader, 1);
        self.SRDT.read(reader, 8);
        self.CTLN.read(reader, 15);
    }
}