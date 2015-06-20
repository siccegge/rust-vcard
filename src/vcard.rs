use std::collections::HashMap;
use std::str::FromStr;



struct VCard {
    // Every VCard has a version
    version: Version,
    // Every VCard at most one of these fields:
    kind: Option<Kind>,
    n: Option<N>,
    bday: Option<Bday>,
    anniversary: Option<Anniversary>,
    gender: Option<Gender>,
    prodid: Option<Prodid>,
    rev: Option<Rev>,
    uid: Option<Uid>,

    // The arbitrary many fields
    rfc_fields: HashMap<RfcProperty, Field>
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

#[derive(Debug,Eq,Hash,PartialEq)]
pub enum RfcProperty {
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

impl RfcProperty {
    fn get_cardinality(&self) -> Cardinality {
        match *self {
            RfcProperty::Begin | RfcProperty::End | RfcProperty::Version => Cardinality::ExactlyOne,
            _ => Cardinality::Arbitrary
        }
    }

    fn from_str(s: &str) -> Result<RfcProperty,()> {
        match s {
            "BEGIN" => Ok(RfcProperty::Begin),
            "END" => Ok(RfcProperty::End),
            "SOURCE" => Ok(RfcProperty::Source),
            "KIND" => Ok(RfcProperty::Kind),
            "XML" => Ok(RfcProperty::Xml),
            "FN" => Ok(RfcProperty::Fn),
            "N" => Ok(RfcProperty::N),
            "NICKNAME" => Ok(RfcProperty::Nickname),
            "PHOTO" => Ok(RfcProperty::Photo),
            "BDAY" => Ok(RfcProperty::Bday),
            "ANNIVERSARY" => Ok(RfcProperty::Anniversary),
            "GENDER" => Ok(RfcProperty::Gender),
            "ADR" => Ok(RfcProperty::Adr),
            "TEL" => Ok(RfcProperty::Tel),
            "EMAIL" => Ok(RfcProperty::Email),
            "IMPP" => Ok(RfcProperty::Impp),
            "LANG" => Ok(RfcProperty::Lang),
            "TZ" => Ok(RfcProperty::Tz),
            "GEO" => Ok(RfcProperty::Geo),
            "TITLE" => Ok(RfcProperty::Title),
            "ROLE" => Ok(RfcProperty::Role),
            "LOGO" => Ok(RfcProperty::Logo),
            "ORG" => Ok(RfcProperty::Org),
            "MEMBER" => Ok(RfcProperty::Member),
            "RELATED" => Ok(RfcProperty::Related),
            "CATEGORIES" => Ok(RfcProperty::Categories),
            "NOTE" => Ok(RfcProperty::Note),
            "PRODID" => Ok(RfcProperty::Prodid),
            "REV" => Ok(RfcProperty::Rev),
            "SOUND" => Ok(RfcProperty::Sound),
            "UID" => Ok(RfcProperty::Uid),
            "CLIENTPIDMAP" => Ok(RfcProperty::Clientpidmap),
            "URL" => Ok(RfcProperty::Url),
            "VERSION" => Ok(RfcProperty::Version),
            "KEY" => Ok(RfcProperty::Key),
            "FBURL" => Ok(RfcProperty::Fburl),
            "CALADRURI" => Ok(RfcProperty::Caladruri),
            "CALURI" => Ok(RfcProperty::Caluri),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct PropertyError;


#[derive(Debug)]
pub struct Property {
    group: Option<String>,
    ptype: RfcProperty,
    params: Option<String>,
    value: String
}


impl Property {
    pub fn new_from_strings(group: Option<&str>, name: &str, params: Option<&str>, value: &str) -> Property {
        Property {
            group: if group.is_some() { Some(group.unwrap().to_string()) } else { None },
            ptype: RfcProperty::from_str(name).unwrap(),
            params: if params.is_some() { Some(params.unwrap().to_string()) } else { None },
            value: value.to_string()
        }
    }
}



// RFC 6350, 6.7.9
// TODO: there could be params...
#[derive(Copy,Clone)]
enum Version {
    Four
}

impl VCard {
    /// Constructs a new VCard with all at least one FN element
    pub fn new(_fn: Vec<String>) -> Option<VCard> {
        if _fn.len() >= 1 {
            return None;
        }

        Some(VCard {
            version: Version::Four,
            kind: None,
            n: None,
            bday: None,
            anniversary: None,
            gender: None,
            prodid: None,
            rev: None,
            uid: None,
            rfc_fields: HashMap::new()
        })
    }

    pub fn get_version(&self) -> Version {
        self.version.clone()
    }

    pub fn get(&self, property: &RfcProperty) -> Option<&Field> {
        self.rfc_fields.get(property)
    }

    pub fn get_mut(&mut self, property: &RfcProperty) -> Option<&mut Field> {
        self.rfc_fields.get_mut(property)
    }
}



































// Just some placeholders
type Uri = String;
type Digit = String;
type Text = String;
type Timestamp = String;
type TextList = String;






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




























// RFC 6350, 6.2.2
// TODO: there could be params...
// TODO: the value here is list-component
struct N;

// RFC 6350, 6.2.5
// TODO: there could be params...
// TODO: Some strange format
struct Bday;

// RFC 6350, 6.2.6
// TODO: there could be params...
// TODO: more strange format thingy
struct Anniversary;


// RFC 6350, 6.2.7
// TODO: there could be params...
struct Gender {
    gender: Gender_,
    value: Option<String>
}

enum Gender_ {
    M,
    F,
    O,
    N,
    U
}

// RFC 6350, 6.7.3
// TODO: there could be params...
struct Prodid {
    value: Text
}

// RFC 6350, 6.7.4
// TODO: there could be params...
struct Rev {
    value: Timestamp
}

// RFC 6350, 6.7.6
// TODO: there could be params... and stuff
struct Uid;

