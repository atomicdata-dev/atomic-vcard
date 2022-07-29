use ical::property::Property;
use ical::{self};
use serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() {
  print!(
    "{}",
    serde_json::to_string_pretty(&parse_vcard_file(BufReader::new(
      File::open("./vcards.vcf").unwrap()
    )))
    .unwrap()
  )
}

fn parse_vcard_file(buf: BufReader<File>) -> serde_json::Value {
  let mut contacts = vec![];

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

    contacts.push(parse_contact(contact.properties));
  }

  return serde_json::Value::Array(contacts);
}

fn parse_contact(properties: Vec<Property>) -> serde_json::Value {
  let mut map = serde_json::Map::new();

  map.insert(
    "https://atomicdata.dev/properties/isA".into(),
    serde_json::Value::Array(vec!["https://atomicdata.dev/classes/Person".into()]),
  );

  let mut grouped_properties: HashMap<String, Vec<Property>> = HashMap::new();

  for property in properties {
    grouped_properties
      .entry(property.name.clone())
      .or_insert(Vec::new())
      .push(property);
  }

  let name = &get_vcard_value("FN".to_owned(), &grouped_properties)
    .unwrap_or(get_vcard_value("N".to_owned(), &grouped_properties).expect("no name in VCard"));

  map.insert(
    "https://atomicdata.dev/properties/name".into(),
    serde_json::Value::String(name.into()),
  );

  map.insert(
    "https://atomicdata.dev/properties/localId".into(),
    serde_json::Value::String(
      (get_vcard_value("VERSION".to_owned(), &grouped_properties).expect("no version in VCard")
        + name)
        .into(),
    ),
  );

  parse_grouped_properties(&mut map, grouped_properties);

  return serde_json::Value::Object(map);
}

fn get_vcard_value(name: String, hash_map: &HashMap<String, Vec<Property>>) -> Option<String> {
  return hash_map[&name].first()?.value.clone();
}

fn parse_grouped_properties(
  map: &mut serde_json::Map<String, serde_json::Value>,
  grouped_properties: HashMap<String, Vec<Property>>,
) {
  let mut unknown_properties: Vec<serde_json::Value> = vec![];

  for (property_name, property_group) in grouped_properties {
    if property_group.len() > 0 {
      match property_name.as_str() {
        "TEL" => drop(map.insert(
          "https://atomicdata.dev/properties/phoneNumbers".into(),
          parse_array_property(
            "PhoneNumber".to_owned(),
            "phoneNumber".to_owned(),
            property_group,
          ),
        )),
        "ADR" => drop(map.insert(
          "https://atomicdata.dev/properties/adresses".into(),
          parse_array_property("Adress".to_owned(), "adress".to_owned(), property_group),
        )),
        "EMAIL" => drop(map.insert(
          "https://atomicdata.dev/properties/emailAddresses".into(),
          parse_array_property(
            "EmailAddress".to_owned(),
            "emailAddress".to_owned(),
            property_group,
          ),
        )),
        "BDAY" => drop(map.insert(
          "https://atomicdata.dev/properties/birthdate".into(),
          parse_single_property(property_group).expect("birthday error"),
        )),
        "VERSION" | "FN" | "N" => (),
        _ => parse_unknown_property(property_group, &mut unknown_properties),
      }
    }
  }

  map.insert(
    "https://atomicdata.dev/properties/vCardUnknowns".into(),
    serde_json::Value::Array(unknown_properties),
  );
}

fn parse_single_property(property_group: Vec<Property>) -> Result<serde_json::Value, String> {
  if property_group.len() > 1 {
    return Err(format!("too many {}", property_group.first().unwrap().name));
  }

  match property_group.first() {
    None => {
      return Err(format!(
        "no value for {}",
        property_group.first().unwrap().name
      ))
    }
    Some(x) => {
      return Ok(serde_json::Value::String(
        x.value.as_ref().unwrap().to_string(),
      ))
    }
  }
}

fn parse_array_property(
  class_name: String,
  property_name: String,
  property_group: Vec<Property>,
) -> serde_json::Value {
  let mut result = vec![];

  for property in property_group {
    if let Some(x) = parse_params(
      property,
      &class_name,
      &("https://atomicdata.dev/properties/".to_owned() + &property_name),
    ) {
      result.push(x);
    }
  }

  return serde_json::Value::Array(result);
}

fn parse_unknown_property(property_group: Vec<Property>, vec: &mut Vec<serde_json::Value>) {
  for property in property_group {
    let mut map = serde_json::Map::new();
    if let Some(value) = property.value {
      map.insert(
        "https://atomicdata.dev/properties/isA".into(),
        serde_json::Value::Array(vec![serde_json::Value::String(format!(
          "https://atomicdata.dev/classes/VCardUnknown"
        ))]),
      );
      map.insert(
        "https://atomicdata.dev/properties/key".into(),
        serde_json::Value::String(property.name),
      );

      map.insert(
        "https://atomicdata.dev/properties/atom/value".into(),
        serde_json::Value::String(value),
      );

      if let Some(x) = make_description(property.params) {
        map.insert("https://atomicdata.dev/properties/description".into(), x);
      }

      vec.push(serde_json::Value::Object(map));
    }
  }
}

fn parse_params(
  property: Property,
  class_name: &String,
  property_name: &String,
) -> Option<serde_json::Value> {
  if let None = property.value {
    return None;
  }

  let mut map = serde_json::Map::new();

  map.insert(
    "https://atomicdata.dev/properties/isA".into(),
    serde_json::Value::Array(vec![serde_json::Value::String(format!(
      "https://atomicdata.dev/classes/{class_name}"
    ))]),
  );

  map.insert(
    property_name.into(),
    serde_json::Value::String(property.value.unwrap()),
  );

  if let Some(x) = make_description(property.params) {
    map.insert("https://atomicdata.dev/properties/description".into(), x);
  }
  return Some(serde_json::Value::Object(map));
}

fn make_description(
  params_option: Option<Vec<(String, Vec<String>)>>,
) -> Option<serde_json::Value> {
  if let Some(params) = params_option {
    if params.len() > 0 {
      let mut description = "".to_owned();
      for (param_name, param_values) in params {
        description.push_str(&(param_name + "=" + &param_values.join("-") + ","));
      }

      return Some(serde_json::Value::String(description));
    }
  }
  return None;
}

#[cfg(test)]
mod test;
