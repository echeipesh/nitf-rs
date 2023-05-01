// RE 
// File Part Type.  This field shall contain the characters 
// “RE” to identify the subheader as a reserved extension. 
// 2 
// RESID 
// Unique RES Type Identifier.  This field shall contain a valid alphanumeric identifier properly registered with the GWG NTB. 
// 25 
// RESVER 
// Version of the Data Definition.  This field shall contain the alphanumeric version number of the use of the tag.  The version number is assigned as part of the registration process. 
// 2 
// RECLAS 
// Reserved Extension File Security Classification.  This field shall contain a valid value representing the classification level of the RES.  Valid values are T for Top Secret, S for Secret, C for Confidential, R for 
// Restricted, or U for Unclassified. 
// 1 
// RECLSY 
// RES Security Classification System.  This field shall contain valid values indicating the national or multinational security system used to classify the RES.  Country Codes per FIPS PUB 10-4 are used to indicate national security systems.  The designator "XN" is for classified data generated by a component using NATO security system marking guidance.  This code is outside the FIPS 10-4 document listing, and was selected to not duplicate that document's existing codes.  If this field is all ECS spaces (0x20), it shall imply that no security classification system applies to the RES. 
// 2 
// RECODE 
// RES Codewords.  This field shall contain a valid indicator of the security compartments associated with the RES.  Values include one or more of the digraphs found in the NITF Field Value Registry, specified in section C.2.1 Online Resources.  The selection of a relevant set of codewords is application specific.  If this field is all ECS spaces 0x20), it shall imply that no codewords apply to the RES. 
// 11 
// RECTLH 
// RES Control and Handling.  This field shall contain valid additional security control and/or handling instructions (caveats) associated with the RES.  Values include digraphs found in the NITF Field Value Registry, specified in section C.2.1 Online Resources.  
// The digraph may indicate single or multiple caveats.  The selection of a relevant caveat(s) is application specific.  If this field is all ECS spaces (0x20), it shall imply that no additional control and handling instructions apply to the RES. 
// 2 
// REREL 
// RES Releasing Instructions.  This field shall contain a valid list of countries to which the RES is authorized for release.  Typical values include one or more country codes as found in FIPS PUB 10-4 separated by a single BCS space (0x20).  If this field is all ECS spaces (0x20), it shall imply that no RES release instructions apply. 
// 20 
// REDCTP 
// RES Declassification Type.  This field shall contain a valid indicator of the type of security declassification or downgrading instructions which apply to the RES.  Valid values are DD for declassify on a specific date, DE for declassify upon occurrence of an event, GD for downgrade to a specified level on a specific date, GE for downgrade to a specified level upon occurrence of an event, O for OADR, and X for exempt from automatic declassification.  If this field is all ECS spaces (0x20), it shall imply that no RES security declassification or downgrading instructions apply. 
// 2 
// REDCDT 
// RES Declassification Date.  This field shall indicate the date on which a RES is to be declassified if the value in REDCTP is DD.  If this field is all ECS spaces (0x20), it shall imply that no RES declassification date applies. 
// 8 
// REDCXM 
// RES Declassification Exemption.  This field shall indicate the reason the RES is exempt from automatic declassification if the value in REDCTP is X.  Valid values are X1 to X8 and 25X1 to 25X9.  X1 to X8 correspond to the declassification exemptions found in DOD 5200.01-V1, paragraphs 4-202b(1) to (8) for material exempt from the 10-year rule.  25X1 to 25X9 correspond to the declassification exemptions found in DOD 5200.01-V1, paragraphs 4-301a(1) to (9) for permanently valuable material exempt from the 25year declassification system.  If this field is all ECS spaces (0x20), it shall imply that a file declassification exemption does not apply. 
// 4 
// REDG 
// RES Downgrade.  This field shall indicate the classification level to which a RES is to be downgraded if the values in REDCTP are GD or GE.  Valid values are S for Secret, C for Confidential, R for Restricted.  If this field contains an ECS space (0x20), it shall imply that RES security downgrading does not apply. 
// 1 
// REDGDT 
// RES Downgrade Date.  This field shall indicate the date on which a RES is to be downgraded if the value in REDCTP is GD.  If this field is all BCS spaces (0x20), it shall imply that a RES security downgrading date does not apply. 
// 8 
// RECLTX 
// RES Classification Text.  This field shall be used to provide additional information about the RES classification to include identification of a 
// declassification or downgrading event if the values in REDCTP are DE or GE.  It may also be used to identify multiple classification sources and/or any other special handling rules.  Values are user-defined free text.  If this field is all ECS spaces (0x20), it shall imply that additional information about RES classification does not apply. 
// 43 
// RECATP 
// RES Classification Authority Type.  This field shall indicate the type of authority used to classify the RES.  
// Valid values are O for original classification authority, D for derivative from a single source, and M for derivative from multiple sources.  If this field contains an ECS space (0x20), it shall imply RECATP does not apply. 
// 1 
// RECAUT 
// RES Classification Authority.  This field shall identify the classification authority for the RES dependent upon the value in RECATP.  Values are user-defined 
// free text which should contain the following information: original classification authority name and position or personal ID if the value in RECATP is O; title of the document or security classification guide used to classify the RES if the value in RECATP is D; and Deriv- Multiple if the RES classification was derived from multiple sources.  In the latter case, the RES originator will maintain a record of the sources used in accordance with existing security directives.  One of the multiple sources may also be identified in 
// RECLTX if desired.  If this field is all ECS spaces (0x20), it shall imply that no RES classification authority applies. 
// 40 
// RECRSN 
// RES Classification Reason.  This field shall contain values indicating the reason for classifying the RES.  Valid values are A to G.  These correspond to the reasons for original classification per E.O. 12958, Section 1.5.(a) to (g).  If this field contains an ECS space (0x20), it shall imply that no RES classification reason applies. 
// 1 
// RESRDT 
// RES Security Source Date.  This field shall indicate the date of the source used to derive the classification of the RES.  In the case of multiple sources, the date of the most recent source shall be used.  If this field is all ECS spaces (0x20), it shall imply that a RESRDT does not apply. 
// 8 
// RECTLN 
// RES Security Control Number.  This field shall contain a valid security control number associated with the RES.  The format of the security control number shall be in accordance with the regulations governing the appropriate security channel(s).  If this field is all ECS spaces (0x20), it shall imply that no RECTLN applies. 
// 15 
// RESSHL 
// RES User-defined Subheader Length.  This field shall contain the number of bytes in the field RESSHF.  If this field contains BCS zeros (0x30), RESSHF shall not appear in the RES subheader. 
// 4 
// RESSHF 
// RES User-Defined Subheader Fields.  This field shall contain user-defined fields.  Data in this field shall be alphanumeric, formatted according to user specification. 
// †9 
// RESDATA 
// RES User-Defined Data.  This field shall contain data of either binary or character types defined by and formatted according to the user’s specification.  The length of this field shall not cause any other NITF field length limits to be exceeded, but is otherwise fully user defined. 
// ††9 
use std::io::{Read, Seek};
use std::fmt::Display;

use crate::types::NitfField;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedExtensionSegment {
    // File Part Type
    pub RE: NitfField, // 2
    // Unique RES Type Identifier
    pub RESID: NitfField, // 25
    // Version of the Data Definition
    pub RESVER: NitfField, // 2
    // File Security Classification
    pub RECLAS: NitfField, // 1
    // Security Classification System
    pub RECLSY: NitfField, // 2
    // Codewords
    pub RECODE: NitfField, // 11
    // Control and Handling
    pub RECTLH: NitfField, // 2
    // Releasing Instructions
    pub REREL: NitfField, // 20
    // Declassification Type
    pub REDCTP: NitfField, // 2
    // Declassification Date
    pub REDCDT: NitfField, // 8
    // Declassification Exemption
    pub REDCXM: NitfField, // 4
    // Downgrade
    pub REDG: NitfField, // 1
    // Downgrade Date
    pub REDGDT: NitfField, // 8
    // Classification Text
    pub RECLTX: NitfField, // 43
    // Classification Authority Type
    pub RECATP: NitfField, // 1
    // Classification Authority
    pub RECAUT: NitfField, // 40
    // Classification Reason
    pub RECRSN: NitfField, // 1
    // Security Source Date
    pub RESRDT: NitfField, // 8
    // Security Control Number
    pub RECTLN: NitfField, // 15
    // User-defined Subheader Length
    pub RESSHL: NitfField, // 4
    // User-Defined Subheader Fields
    pub RESSHF: NitfField,
    // User-Defined Data
    pub RESDATA: NitfField,
}
impl Display for ReservedExtensionSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.RE).as_ref();
        out_str += format!("{}", self.RESID).as_ref();
        out_str += format!("{}", self.RESVER).as_ref();
        out_str += format!("{}", self.RECLAS).as_ref();
        out_str += format!("{}", self.RECLSY).as_ref();
        out_str += format!("{}", self.RECODE).as_ref();
        out_str += format!("{}", self.RECTLH).as_ref();
        out_str += format!("{}", self.REREL).as_ref();
        out_str += format!("{}", self.REDCTP).as_ref();
        out_str += format!("{}", self.REDCDT).as_ref();
        out_str += format!("{}", self.REDCXM).as_ref();
        out_str += format!("{}", self.REDG).as_ref();
        out_str += format!("{}", self.REDGDT).as_ref();
        out_str += format!("{}", self.RECLTX).as_ref();
        out_str += format!("{}", self.RECATP).as_ref();
        out_str += format!("{}", self.RECAUT).as_ref();
        out_str += format!("{}", self.RECRSN).as_ref();
        out_str += format!("{}", self.RESRDT).as_ref();
        out_str += format!("{}", self.RECTLN).as_ref();
        out_str += format!("{}", self.RESSHL).as_ref();
        out_str += format!("{}", self.RESSHF).as_ref();
        out_str += format!("{}", self.RESDATA).as_ref();
        write!(f, "ReservedExtensionSegment: [{}]", out_str)
    }
}




















