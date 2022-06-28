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
  result.push_str(
    "    \"https://atomicdata.dev/properties/isA\": [\"https://atomcidata.dev/classes/Person\"],\r\n",
  );

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

  for (name, values) in grouped_properties {
    if values.len() > 0 {
      result.push_str(&parse_property(&name, values));
    }
  }

  result.push_str("  },\r\n");
  return result;
}

fn get_vcard_value(name: String, hash_map: &HashMap<String, Vec<Property>>) -> Option<String> {
  return hash_map[&name].first()?.value.clone();
}

fn parse_property(property_name: &str, property_values: Vec<Property>) -> String {
  match property_name {
    "TEL" => parse_array_property(
      "https://atomicdata.dev/properties/phoneNumbers".to_owned(),
      "PhoneNumber".to_owned(),
      "https://atomicdata.dev/properties/phoneNumber".to_owned(),
      property_values,
    ),
    "ADR" => parse_array_property(
      "https://atomicdata.dev/properties/adresses".to_owned(),
      "Adress".to_owned(),
      "https://atomicdata.dev/properties/adress".to_owned(),
      property_values,
    ),
    "EMAIL" => parse_array_property(
      "https://atomicdata.dev/properties/emails".to_owned(),
      "Email".to_owned(),
      "https://atomicdata.dev/properties/email".to_owned(),
      property_values,
    ),
    "BDAY" => parse_single_property(
      "https://atomicdata.dev/properties/birthdate".to_owned(),
      property_values,
    )
    .expect("birthday error"),
    _ => parse_array_property(
      "https://atomicdata.dev/properties/vCard-".to_owned() + property_name + "s",
      "VCard-".to_owned() + property_name,
      "https://atomicdata.dev/properties/vCard-".to_owned() + property_name,
      property_values,
    ),
  }
}

fn parse_single_property(
  property_name: String,
  property_values: Vec<Property>,
) -> Result<String, String> {
  if property_values.len() > 1 {
    return Err("too many ".to_owned() + &property_name);
  }

  match property_values.first() {
    None => return Err("no value for ".to_owned() + &property_name),
    Some(x) => {
      return Ok(to_key_value_pair(
        "    ".to_owned(),
        property_name,
        x.value.as_ref().unwrap().to_string(),
      ))
    }
  }
}

fn parse_array_property(
  properties_name: String,
  class_name: String,
  property_name: String,
  property_values: Vec<Property>,
) -> String {
  let mut result = "    \"".to_owned() + &properties_name + "\": [\r\n";

  for property in property_values {
    result.push_str(&parse_params(property, &class_name, &property_name));
  }
  result.push_str("    ],\r\n");
  return result;
}

fn parse_unknown_property() {}

fn parse_params(property: Property, class_name: &String, property_name: &String) -> String {
  if let None = property.value {
    return "".to_owned();
  }

  let mut result = "      {\r\n".to_owned();

  result.push_str(
    &("        \"https://atomicdata.dev/properties/isA\": [\"https://atomcidata.dev/classes/"
      .to_owned()
      + &class_name
      + "\"],\r\n"),
  );

  result.push_str(&to_key_value_pair(
    "        ".to_owned(),
    property_name.to_owned(),
    property.value.unwrap(),
  ));

  if let Some(params) = property.params {
    if params.len() > 0 {
      let mut description = "".to_owned();
      for (param_name, param_values) in params {
        description.push_str(&(param_name + "=" + &param_values.join("-") + ","));
      }

      result.push_str(&to_key_value_pair(
        "        ".to_owned(),
        "https://atomicdata.dev/properties/description".to_owned(),
        description,
      ))
    }
  }
  result.push_str("      },\r\n");
  return result;
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
