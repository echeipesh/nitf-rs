//! Functions to interface with NITF version 2.1
pub mod types;
pub mod headers;

use std::fmt::Display;
use std::fs::File;

use headers::*;
use data_extension::DataExtensionHeader;
use graphic::GraphicHeader;
use image::ImageHeader;
use nitf_header::NitfHeader;
use reserved_extension::ReservedExtensionHeader;
use text::TextHeader;
use types::{Segment, DataSegment};

/// Top level NITF interface
///
/// Collection of [Segment] objects
///  
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header. See [NitfHeader] for `meta` fields
    pub nitf_header: Segment<NitfHeader>,
    /// Vector of image segments. See [ImageHeader] for `meta` fields
    pub image_segments: Vec<DataSegment<ImageHeader>>,
    /// Vector of graphics segments. See [GraphicHeader] for `meta` fields
    pub graphic_segments: Vec<Segment<GraphicHeader>>,
    /// Vector of text segments. See [TextHeader] for `meta` fields
    pub text_segments: Vec<Segment<TextHeader>>,
    /// Vector of data extension segments. See [DataExtensionHeader] for `meta` fields
    pub data_extension_segments: Vec<Segment<DataExtensionHeader>>,
    /// Vector of reserved extension segments. See [ReservedExtensionHeader] for `meta` fields
    pub reserved_extension_segments: Vec<DataSegment<ReservedExtensionHeader>>,
}

impl From<&mut File> for Nitf {
    fn from(value: &mut File) -> Self {
        Self::from_file(value)
    }
}
impl Nitf {
    pub fn from_file(reader: &mut File) -> Self {
        let mut nitf = Self::default();
        nitf.nitf_header.read_header(reader, 0, 0);

        let mut n_seg: usize = nitf.nitf_header.meta.NUMI.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.IMHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: DataSegment<ImageHeader> =
                DataSegment::from_file(reader, header_size, data_size).unwrap();
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMS.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.GRAPHHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<GraphicHeader> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMT.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.TEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<TextHeader> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMDES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.DEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<DataExtensionHeader> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMRES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.RESHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: DataSegment<ReservedExtensionHeader> =
                DataSegment::from_file(reader, header_size, data_size).unwrap();
            nitf.reserved_extension_segments.push(seg);
        }
        return nitf;
    }
}

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("[{}]", self.nitf_header).as_ref();
        for segment in &self.image_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.graphic_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.text_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.data_extension_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.reserved_extension_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        write!(f, "{}", out_str)
    }
}
