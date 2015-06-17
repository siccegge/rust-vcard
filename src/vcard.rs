use std::collections::HashMap;

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

#[derive(Eq,Hash,PartialEq)]
enum Properties {
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

impl Properties {
    fn get_cardinality(&self) -> Cardinality {
        match *self {
            Properties::Begin | Properties::End | Properties::Version => Cardinality::ExactlyOne,
            _ => Cardinality::Arbitrary
        }
    }
}

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
    rfc_fields: HashMap<Properties, Field>
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

    pub fn get(&self, property: &Properties) -> Option<&Field> {
        self.rfc_fields.get(property)
    }

    pub fn get_mut(&mut self, property: &Properties) -> Option<&mut Field> {
        self.rfc_fields.get_mut(property)
    }
}



































// Just some placeholders
type Uri = String;
type Digit = String;
type Text = String;
type Timestamp = String;
type TextList = String;





// RFC 6350, 6.1.3
// TODO: there could be params...
struct Source {
    value: Uri
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

// RFC 6350, 6.1.5
// TODO: there could be params...
struct Xml {
    value: Text
}

// RFC 6350, 6.2.1
// TODO: there could be params...
struct Fn {
    value: Text
}

// RFC 6350, 6.2.2
// TODO: there could be params...
// TODO: the value here is list-component
struct N;

// RFC 6350, 6.2.3
// TODO: there could be params...
struct Nickname {
    value: TextList
}

// RFC 6350, 6.2.4
// TODO: there could be params...
struct Photo {
    value: Uri
}

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

// RFC 6350, 6.3.1
// Mixture of list-components
struct Adr;

// RFC 6350, 6.4.1
// TODO: there could be params with semantics
struct Tel;

// RFC 6350, 6.4.2
// TODO: there could be params...
struct Email {
    value: Text
}

// RFC 6350, 6.4.3
// TODO: there could be params...
struct Impp {
    value: Uri
}

// RFC 6350, 6.4.4
// TODO: there could be params... and a language tag
struct Lang;

// RFC 6350, 6.5.1
// TODO: there could be params... more strange things
struct Tz;

// RFC 6350, 6.5.2
// TODO: there could be params...
struct Geo {
    value: Uri
}

// RFC 6350, 6.6.1
// TODO: there could be params...
struct Title {
    value: Text
}

// RFC 6350, 6.6.2
// TODO: there could be params...
struct Role {
    value: Text
}

// RFC 6350, 6.6.3
// TODO: there could be params...
struct Logo {
    value: Uri
}

// RFC 6350, 6.6.4
// TODO: there could be params... and something with components
struct Org;

// RFC 6350, 6.6.5
// TODO: there could be params...
struct Member {
    value: Uri
}

// RFC 6350, 6.6.6
// TODO: there could be params...
struct Related;

// RFC 6350, 6.7.1
// TODO: there could be params...
struct Categories {
    value: TextList
}

// RFC 6350, 6.7.2
// TODO: there could be params...
struct Note {
    value: Text
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

// RFC 6350, 6.7.5
// TODO: there could be params...
struct Sound {
    value: Timestamp
}

// RFC 6350, 6.7.6
// TODO: there could be params... and stuff
struct Uid;

// RFC 6350, 6.7.7
// TODO: there could be params...
struct Clientpidmap {
    value: (Digit,Uri)
}

// RFC 6350, 6.7.8
// TODO: there could be params...
struct Url {
    value: Uri
}



// RFC 6350, 6.8.1
// TODO: there could be params... uri or text or..
struct Key;

// RFC 6350, 6.9.1
// TODO: there could be params...
struct Fburl {
    value: Uri
}

// RFC 6350, 6.9.2
// TODO: there could be params...
struct Caladruri {
    value: Uri
}

// RFC 6350, 6.9.3
// TODO: there could be params...
struct Caluri {
    value: Uri
}
