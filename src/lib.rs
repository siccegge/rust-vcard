#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

extern crate chrono;

mod parser;
mod vcard;

// type Text = String;
// type TextList = [String];
// type Date = 
// type DateList = 
// type DateTimeList = 



// As §4 says that there are the following data types for values
enum Value {
    Text,
    TextList,
    DateList,
    TimeList,
    DateTimeList,
    DateAndOrTimeList,
    TimestampList,
    Boolean,
    IntegerList,
    FloatList,
    URI,
    UtcOffset,
    LanguageTag,
    IanaValuespec
}


