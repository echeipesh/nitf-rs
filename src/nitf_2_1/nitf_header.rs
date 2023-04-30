//! Header definition
use std::io::{Read, Seek};
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::*;

// Struct definition
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfHeader {
    /// File Profile Name
    pub FHDR: NitfElement,
    /// File Version
    pub FVER: NitfElement,
    /// Complexity Level
    pub CLEVEL: NitfElement,
    /// Standard Type
    pub STYPE: NitfElement,
    /// Originating Station ID
    pub OSTAID: NitfElement,
    /// File Date and Time
    pub FDT: NitfElement,
    /// File Title
    pub FTITLE: NitfElement,
    /// File Security Classification
    pub FSCLAS: NitfElement,
    /// File Classification Security System
    pub FSCLSY: NitfElement,
    /// File Codewords
    pub FSCODE: NitfElement,
    /// File Control and Handling
    pub FSCTLH: NitfElement,
    /// File Releasing Instructions
    pub FSREL: NitfElement,
    /// File Declassification Type
    pub FSDCTP: NitfElement,
    /// File Declassification Date
    pub FSDCDT: NitfElement,
    /// File Declassification Exemption
    pub FSDCXM: NitfElement,
    /// File Downgrade
    pub FSDG: NitfElement,
    /// File Downgrade Date
    pub FSDGDT: NitfElement,
    /// File Classification Text
    pub FSCLTX: NitfElement,
    /// File Classification Authority Type
    pub FSCATP: NitfElement,
    /// File Classification Authority
    pub FSCAUT: NitfElement,
    /// File Classification Reason
    pub FSCRSN: NitfElement,
    /// File Security Source Date
    pub FSSRDT: NitfElement,
    /// File Security Control Number
    pub FSCTLN: NitfElement,
    /// File Copy Number
    pub FSCOP: NitfElement,
    /// File Number of Copies
    pub FSCPYS: NitfElement,
    /// Encryption
    pub ENCRYP: NitfElement,
    /// File Background Color
    pub FBKGC: NitfElement,
    /// Originator's Name
    pub ONAME: NitfElement,
    /// Originator's Phone Number
    pub OPHONE: NitfElement,
    /// File Length
    pub FL: NitfElement,
    /// NITF File Header Length
    pub HL: NitfElement,
    /// Number of Image Segments
    pub NUMI: NitfElement,
    /// Image Segments
    pub IMHEADERS: NitfSubHeader,
    /// Number of Graphics Segments
    pub NUMS: NitfElement,
    /// Graphic Segments
    pub GRAPHHEADERS: NitfSubHeader,
    /// Reserved for future use
    pub NUMX: NitfElement,
    /// Number of Text Files
    pub NUMT: NitfElement,
    /// Text Segments
    pub TEXTFILES: NitfSubHeader,
    /// Number of Data Extension Segments
    pub NUMDES: NitfElement,
    /// Data Extenstion Segments
    pub DEXTHEADERS: NitfSubHeader,
    /// Number of Reserved Extension Segments
    pub NUMRES: NitfElement,
    /// Reserved Extension Segments
    pub RESHEADERS: NitfSubHeader,
    /// User Defined Header Data Length
    pub UDHDL: NitfElement,
    /// Extended Header Data Length
    pub XHDL: NitfElement,
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\nFHDR: {}", self.FHDR).as_ref();
        out_str += format!("\nFVER: {}", self.FVER).as_ref();
        out_str += format!("\nCLEVEL: {}", self.CLEVEL).as_ref();
        out_str += format!("\nSTYPE: {}", self.STYPE).as_ref();
        out_str += format!("\nOSTAID: {}", self.OSTAID).as_ref();
        out_str += format!("\nFDT: {}", self.FDT).as_ref();
        out_str += format!("\nFTITLE: {}", self.FTITLE).as_ref();
        out_str += format!("\nFSCLAS: {}", self.FSCLAS).as_ref();
        out_str += format!("\nFSCLSY: {}", self.FSCLSY).as_ref();
        out_str += format!("\nFSCODE: {}", self.FSCODE).as_ref();
        out_str += format!("\nFSCTLH: {}", self.FSCTLH).as_ref();
        out_str += format!("\nFSREL: {}", self.FSREL).as_ref();
        out_str += format!("\nFSDCTP: {}", self.FSDCTP).as_ref();
        out_str += format!("\nFSDCDT: {}", self.FSDCDT).as_ref();
        out_str += format!("\nFSDCXM: {}", self.FSDCXM).as_ref();
        out_str += format!("\nFSDG: {}", self.FSDG).as_ref();
        out_str += format!("\nFSDGDT: {}", self.FSDGDT).as_ref();
        out_str += format!("\nFSCLTX: {}", self.FSCLTX).as_ref();
        out_str += format!("\nFSCATP: {}", self.FSCATP).as_ref();
        out_str += format!("\nFSCAUT: {}", self.FSCAUT).as_ref();
        out_str += format!("\nFSCRSN: {}", self.FSCRSN).as_ref();
        out_str += format!("\nFSSRDT: {}", self.FSSRDT).as_ref();
        out_str += format!("\nFSCTLN: {}", self.FSCTLN).as_ref();
        out_str += format!("\nFSCOP: {}", self.FSCOP).as_ref();
        out_str += format!("\nFSCPYS: {}", self.FSCPYS).as_ref();
        out_str += format!("\nENCRYP: {}", self.ENCRYP).as_ref();
        out_str += format!("\nFBKGC: {}", self.FBKGC).as_ref();
        out_str += format!("\nONAME: {}", self.ONAME).as_ref();
        out_str += format!("\nOPHONE: {}", self.OPHONE).as_ref();
        out_str += format!("\nFL: {}", self.FL).as_ref();
        out_str += format!("\nHL: {}", self.HL).as_ref();
        out_str += format!("\nNUMI: {}", self.NUMI).as_ref();
        out_str += format!("\nIMHEADERS: {}", self.IMHEADERS).as_ref();
        out_str += format!("\nNUMS: {}", self.NUMS).as_ref();
        out_str += format!("\nGRAPHHEADERS: {}", self.GRAPHHEADERS).as_ref();
        out_str += format!("\nNUMX: {}", self.NUMX).as_ref();
        out_str += format!("\nNUMT: {}", self.NUMT).as_ref();
        out_str += format!("\nTEXTFILES: {}", self.TEXTFILES).as_ref();
        out_str += format!("\nNUMDES: {}", self.NUMDES).as_ref();
        out_str += format!("\nDEXTHEADERS: {}", self.DEXTHEADERS).as_ref();
        out_str += format!("\nNUMRES: {}", self.NUMRES).as_ref();
        out_str += format!("\nRESHEADERS: {}", self.RESHEADERS).as_ref();
        out_str += format!("\nUDHDL: {}", self.UDHDL).as_ref();
        out_str += format!("\nXHDL: {}", self.XHDL).as_ref();
        write!(f, "NitfHeader: [{}]", out_str)
    }
}

// Struct functions
impl NitfHeader {
    pub fn from_reader(
        reader: &mut impl Read
    ) -> Result<Self, FromUtf8Error> {
        let mut hdr = Self::default();
        hdr.FHDR.read(reader, 4);
        hdr.FVER.read(reader, 5);
        hdr.CLEVEL.read(reader, 2);
        hdr.STYPE.read(reader, 4);
        hdr.OSTAID.read(reader, 10);
        hdr.FDT.read(reader, 14);
        hdr.FTITLE.read(reader, 80);
        hdr.FSCLAS.read(reader, 1);
        hdr.FSCLSY.read(reader, 2);
        hdr.FSCODE.read(reader, 11);
        hdr.FSCTLH.read(reader, 2);
        hdr.FSREL.read(reader, 20);
        hdr.FSDCTP.read(reader, 2);
        hdr.FSDCDT.read(reader, 8);
        hdr.FSDCXM.read(reader, 4);
        hdr.FSDG.read(reader, 1);
        hdr.FSDGDT.read(reader, 8);
        hdr.FSCLTX.read(reader, 43);
        hdr.FSCATP.read(reader, 1);
        hdr.FSCAUT.read(reader, 40);
        hdr.FSCRSN.read(reader, 1);
        hdr.FSSRDT.read(reader, 8);
        hdr.FSCTLN.read(reader, 15);
        hdr.FSCOP.read(reader, 5);
        hdr.FSCPYS.read(reader, 5);
        hdr.ENCRYP.read(reader, 1);
        hdr.FBKGC.read(reader, 3);
        hdr.ONAME.read(reader, 24);
        hdr.OPHONE.read(reader, 18);
        hdr.FL.read(reader, 12);
        hdr.HL.read(reader, 6);
        hdr.NUMI.read(reader, 3);
        hdr.IMHEADERS.read(
            reader, 
            &hdr.NUMI, 
            6, 
            10);
        hdr.NUMS.read(reader, 3);
        hdr.GRAPHHEADERS.read(
            reader, 
            &hdr.NUMS, 
            4, 
            6);
        hdr.NUMX.read(reader, 3);
        hdr.NUMT.read(reader, 3);
        hdr.TEXTFILES.read(
            reader, 
            &hdr.NUMT, 
            4, 
            5);
        hdr.NUMDES.read(reader, 3);
        hdr.DEXTHEADERS.read(
            reader, 
            &hdr.NUMDES, 
            4, 
            9);
        hdr.NUMRES.read(reader, 3);
        hdr.RESHEADERS.read(
            reader, 
            &hdr.NUMRES, 
            4, 
            7);
        hdr.UDHDL.read(reader, 5);
        hdr.XHDL.read(reader, 5);
        Ok(hdr)
    }
}


