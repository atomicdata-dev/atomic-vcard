use ical::property::Property;
use ical::{self};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() {
  print!(
    "{}",
    parse_contacts(BufReader::new(File::open("./vcards.vcf").unwrap()))
  )
}

fn parse_contacts(buf: BufReader<File>) -> String {
  let mut result = "[\r\n".to_owned();

  let reader = ical::VcardParser::new(buf);

  let mut contact;

  for maybe_contact in reader {
    match maybe_contact {
      Ok(x) => contact = x,
      Err(e) => {
        println!("{}", e);
        continue;
      }
    }

    result.push_str(&parse_contact(contact.properties));
  }

  result.push_str("]\r\n");
  return result;
}

fn parse_contact(properties: Vec<Property>) -> String {
  let mut result = "  {\r\n".to_owned();

  let mut grouped_properties: HashMap<String, Vec<Property>> = HashMap::new();

  for property in properties {
    grouped_properties
      .entry(property.name.clone())
      .or_insert(Vec::new())
      .push(property);
  }

  let id = to_key_value_pair(
    "    ".to_owned(),
    "https://atomicdata.dev/properties/localId".to_owned(),
    get_vcard_value("VERSION".to_owned(), &grouped_properties).expect("no version in VCard")
      + &get_vcard_value("FN".to_owned(), &grouped_properties).unwrap_or("".to_owned())
      + &get_vcard_value("N".to_owned(), &grouped_properties).unwrap_or("".to_owned()),
  );

  result.push_str(&id);

  for property in grouped_properties {
    result.push_str(&parse_property(property));
  }

  result.push_str("  },\r\n");
  return result;
}

fn get_vcard_value(name: String, hash_map: &HashMap<String, Vec<Property>>) -> Option<String> {
  return hash_map[&name].first()?.value.clone();
}

fn parse_property(tuple: (String, Vec<Property>)) -> String {
  // if tuple.1.len() == 1 {
  //   return to_key_value_pair(
  //     "    ".to_owned(),
  //     vcard_name_to_atomic_name(&tuple.0),
  //     tuple.1.first().unwrap().clone().value.unwrap(),
  //   );
  // }

  let mut result = "    \"".to_owned() + &vcard_name_to_atomic_name(&tuple.0) + "\": [\r\n";

  for property in tuple.1 {
    result.push_str(&parse_params(property));
  }
  result.push_str("    ],\r\n");
  return result;
}

fn parse_params(property: Property) -> String {
  let mut result = "      {\r\n".to_owned();

  result.push_str(&to_key_value_pair(
    "        ".to_owned(),
    "https://atomicdata.dev/properties/atom/value".to_owned(),
    property.value.unwrap(),
  ));

  for param in property.params.unwrap_or(Vec::from([])) {
    result.push_str(&to_key_value_pair(
      "        ".to_owned(),
      vcard_name_to_atomic_name(&param.0),
      param.1.join("-"),
    ));
  }
  result.push_str("      },\r\n");
  return result;
}

fn vcard_name_to_atomic_name(name: &str) -> String {
  match name {
    "EMAIL" => "https://atomicdata.dev/properties/email".to_owned(),
    "TYPE" => "https://atomicdata.dev/properties/name".to_owned(),
    "TEL" => "https://atomicdata.dev/properties/phoneNumber".to_owned(),
    _ => "https://atomicdata.dev/properties/vcard-".to_owned() + name,
  }
}

fn to_key_value_pair(indentation: String, key: String, value: String) -> String {
  return indentation + "\"" + &key + "\": \"" + &value + "\",\r\n";
}

// {
//     "https://atomicdata.dev/properties/localId": "john-doe-johnDoe@example.org", // Must be locally unique & deterministic
//     "https://atomicdata.dev/properties/phoneNumbers": [
//         {
//             "https://atomicdata.dev/properties/isA": ["https://atomcidata.dev/classes/PhoneNumber"],
//             "https://atomicdata.dev/properties/number": "+31636020942",
//             "https://atomicdata.dev/properties/name": "Home",
//         }
//     ]
// }

// ADR
// AGENT
// ANNIVERSARY
// BDAY
// BEGIN
// CALADRURI
// CALURI
// CATEGORIES
// CLASS
// CLIENTPIDMAP
// EMAIL
// END
// FBURL
// FN
// GENDER
// GEO
// IMPP
// KEY
// KIND
// LABEL
// LANG
// LOGO
// MAILER
// MEMBER
// N
// NAME
// NICKNAME
// NOTE
// ORG
// PHOTO
// PRODID
// PROFILE
// RELATED
// REV
// ROLE
// SORT_STRING
// SOUND
// SOURCE
// TEL
// TITLE
// TZ
// UID
// URL
// VERSION
// XML
