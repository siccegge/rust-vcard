use std::collections::HashMap;
use std::str::FromStr;

use ::parser::ParserError;

#[derive(Debug)]
struct VCard {
    version: Version,
    properties: Vec<Property>
}

impl VCard {
    /// Constructs a new VCard with all at least one FN element
    pub fn new(_fn: Vec<String>) -> Option<VCard> {
        if _fn.len() >= 1 {
            return None;
        }

        Some(VCard {
            version: Version::Four,
            properties: Vec::new()
        })
    }
}



#[derive(Debug)]
pub struct Property {
    group: Option<String>,
    ptype: PropertyType,
    params: Option<String>,
    value: ValueType
}

impl Property {
    pub fn new_from_strings(group: Option<&str>, name: &str, params: Option<&str>, value: &str) -> Result<Property,ParserError> {
        let group_string = if group.is_some() { Some(group.unwrap().to_string()) } else { None };

        let property = match PropertyType::from_str(name).unwrap() {
            PropertyType::Source => Property::new_source(group_string, params, value),
            PropertyType::Version => Property::new_version(group_string, params, value),
            PropertyType::N => Property::new_n(group_string, params, value),
            _ => Err(ParserError::new("Don't understand you".to_string()))
        };

        property
    }

    fn new_source(group_string: Option<String>, params: Option<&str>, value: &str) -> Result<Property,ParserError> {
        // the value of a source can only be an URI
        let vuri = ValueType::URI(value.to_string());
        Ok(Property {
            group: group_string,
            ptype: PropertyType::Source,
            params: if params.is_some() { Some(params.unwrap().to_string()) } else { None },
            value: vuri
        })
    }

    fn new_version(group_string: Option<String>, params: Option<&str>, value: &str) -> Result<Property,ParserError> {
        Ok(Property {
            group: group_string,
            ptype: Property::Version,
            params: if params.is_some() { Some(params.unwrap().to_string()) } else { None },
            value: ValueType::Text("4".to_string())
        })
    }

    fn new_n(group_string: Option<String>, params: Option<&str>, value: &str) -> Result<Property,ParserError> {
    }


}














// A VCard has this general shape:
//
//   vcard = "BEGIN:VCARD" CRLF
//           "VERSION:4.0" CRLF
//           1*contentline
//           "END:VCARD" CRLF

// the content line contains of:
// * an optional group
// * a name
// * an optional list of params
// * a value
//
// grammar:
//    contentline = [group "."] name *(";" param) ":" value CRLF

// There are four cardinalities used in the code this is represented with:
// 1  -> a field in the struct and it's an argument in the constructor
// *1 -> an Option field
// 1* -> a Vec field which is initally provided in the constructor
// *  -> a Vec field

type Name = String;
type Field = String;



////////////////////////////////////////////////////////////////////
/// Type of a property
////////////////////////////////////////////////////////////////////

#[derive(Debug,Eq,Hash,PartialEq)]
pub enum PropertyType {
    Begin,
    End,
    Source,
    Kind,
    Xml,
    Fn,
    N,
    Nickname,
    Photo,
    Bday,
    Anniversary,
    Gender,
    Adr,
    Tel,
    Email,
    Impp,
    Lang,
    Tz,
    Geo,
    Title,
    Role,
    Logo,
    Org,
    Member,
    Related,
    Categories,
    Note,
    Prodid,
    Rev,
    Sound,
    Uid,
    Clientpidmap,
    Url,
    Version,
    Key,
    Fburl,
    Caladruri,
    Caluri,
}

enum Cardinality {
    ExactlyOne,
    AtMostOne,
    AtLeastOne,
    Arbitrary
}

impl PropertyType {
    fn get_cardinality(&self) -> Cardinality {
        match *self {
            PropertyType::Begin | PropertyType::End | PropertyType::Version => Cardinality::ExactlyOne,
            _ => Cardinality::Arbitrary
        }
    }

    fn from_str(s: &str) -> Result<PropertyType,()> {
        match s {
            "BEGIN" => Ok(PropertyType::Begin),
            "END" => Ok(PropertyType::End),
            "SOURCE" => Ok(PropertyType::Source),
            "KIND" => Ok(PropertyType::Kind),
            "XML" => Ok(PropertyType::Xml),
            "FN" => Ok(PropertyType::Fn),
            "N" => Ok(PropertyType::N),
            "NICKNAME" => Ok(PropertyType::Nickname),
            "PHOTO" => Ok(PropertyType::Photo),
            "BDAY" => Ok(PropertyType::Bday),
            "ANNIVERSARY" => Ok(PropertyType::Anniversary),
            "GENDER" => Ok(PropertyType::Gender),
            "ADR" => Ok(PropertyType::Adr),
            "TEL" => Ok(PropertyType::Tel),
            "EMAIL" => Ok(PropertyType::Email),
            "IMPP" => Ok(PropertyType::Impp),
            "LANG" => Ok(PropertyType::Lang),
            "TZ" => Ok(PropertyType::Tz),
            "GEO" => Ok(PropertyType::Geo),
            "TITLE" => Ok(PropertyType::Title),
            "ROLE" => Ok(PropertyType::Role),
            "LOGO" => Ok(PropertyType::Logo),
            "ORG" => Ok(PropertyType::Org),
            "MEMBER" => Ok(PropertyType::Member),
            "RELATED" => Ok(PropertyType::Related),
            "CATEGORIES" => Ok(PropertyType::Categories),
            "NOTE" => Ok(PropertyType::Note),
            "PRODID" => Ok(PropertyType::Prodid),
            "REV" => Ok(PropertyType::Rev),
            "SOUND" => Ok(PropertyType::Sound),
            "UID" => Ok(PropertyType::Uid),
            "CLIENTPIDMAP" => Ok(PropertyType::Clientpidmap),
            "URL" => Ok(PropertyType::Url),
            "VERSION" => Ok(PropertyType::Version),
            "KEY" => Ok(PropertyType::Key),
            "FBURL" => Ok(PropertyType::Fburl),
            "CALADRURI" => Ok(PropertyType::Caladruri),
            "CALURI" => Ok(PropertyType::Caluri),
            _ => Err(())
        }
    }
}


////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////
/// Type of a value
////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ValueType {
    Text(String),
    TextList(Vec<String>),
    DateList(Vec<Date>),
    TimeList(Vec<Time>),
    DateTimeList(Vec<DateTime>),
    DateAndOrTimeList(Vec<DateAndOrTime>),
    TimestampList(Vec<Timestamp>),
    Boolean(bool),
    IntegerList(Vec<i64>),
    FloatList(Vec<f64>),
    URI(URI),
    UtcOffset(UtcOffset),
    LanguageTag(LanguageTag),
    IanaValuespec(IanaValuespec)
}

type Date = String;
type Time = String;
type DateTime = String;
type DateAndOrTime = String;
type Timestamp = String;
type URI = String;
type UtcOffset = String;
type LanguageTag = String;
type IanaValuespec = String;










// RFC 6350, 6.7.9
// TODO: there could be params...
#[derive(Copy,Clone,Debug)]
enum Version {
    Four
}






















// RFC 6350, 6.1.4
// TODO: there could be params...
enum Kind {
    Individual,
    Group,
    Org,
    Location,
    XName(String), // Todo maybe String is too general
    IanaToken(String) // Todo maybe String is too general
}


