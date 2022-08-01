use serde_json;

use super::*;

const EXPECTED: &str = r#"[
  {
    "https://atomicdata.dev/properties/emailAddresses": [
      {
        "https://atomicdata.dev/properties/description": "TYPE=INTERNET,TYPE=WORK,TYPE=pref,",
        "https://atomicdata.dev/properties/emailAddress": "johnDoe@example.org",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/EmailAddress"
        ]
      }
    ],
    "https://atomicdata.dev/properties/isA": [
      "https://atomicdata.dev/classes/Person"
    ],
    "https://atomicdata.dev/properties/localId": "3.0John Doe",
    "https://atomicdata.dev/properties/name": "John Doe",
    "https://atomicdata.dev/properties/phoneNumbers": [
      {
        "https://atomicdata.dev/properties/description": "TYPE=WORK,TYPE=pref,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/PhoneNumber"
        ],
        "https://atomicdata.dev/properties/phoneNumber": "+1 617 555 1212"
      },
      {
        "https://atomicdata.dev/properties/description": "TYPE=WORK,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/PhoneNumber"
        ],
        "https://atomicdata.dev/properties/phoneNumber": "+1 (617) 555-1234"
      },
      {
        "https://atomicdata.dev/properties/description": "TYPE=CELL,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/PhoneNumber"
        ],
        "https://atomicdata.dev/properties/phoneNumber": "+1 781 555 1212"
      },
      {
        "https://atomicdata.dev/properties/description": "TYPE=HOME,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/PhoneNumber"
        ],
        "https://atomicdata.dev/properties/phoneNumber": "+1 202 555 1212"
      }
    ],
    "https://atomicdata.dev/properties/vCardUnknowns": [
      {
        "https://atomicdata.dev/properties/atom/value": "5AD380FD-B2DE-4261-BA99-DE1D1DB52FBE\\:ABPerson",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "X-ABUID"
      },
      {
        "https://atomicdata.dev/properties/atom/value": ";;2 Enterprise Avenue;Worktown;NY;01111;USA",
        "https://atomicdata.dev/properties/description": "TYPE=WORK,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item1.ADR"
      },
      {
        "https://atomicdata.dev/properties/atom/value": ";;3 Acacia Avenue;Hoemtown;MA;02222;USA",
        "https://atomicdata.dev/properties/description": "TYPE=HOME,TYPE=pref,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item2.ADR"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Example.com Inc.;",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "ORG"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "FOAF",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item4.X-ABLabel"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Imaginary test person",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "TITLE"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Jane Doe",
        "https://atomicdata.dev/properties/description": "TYPE=pref,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item5.X-ABRELATEDNAMES"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "John Doe has a long and varied history\\, being documented on more police files that anyone else. Reports of his death are alas numerous.",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "NOTE"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Work,Test group",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "CATEGORIES"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "_$!<Friend>!$_",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item5.X-ABLabel"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "_$!<HomePage>!$_",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item3.X-ABLabel"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "http\\://www.example.com/Joe/foaf.df",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item4.URL"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "http\\://www.example/com/doe",
        "https://atomicdata.dev/properties/description": "TYPE=pref,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item3.URL"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "us",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item1.X-ABADR"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "us",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item2.X-ABADR"
      }
    ]
  },
  {
    "https://atomicdata.dev/properties/birthdate": "19910101",
    "https://atomicdata.dev/properties/emailAddresses": [
      {
        "https://atomicdata.dev/properties/description": "TYPE=INTERNET,TYPE=WORK,",
        "https://atomicdata.dev/properties/emailAddress": "john@example.com",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/EmailAddress"
        ]
      }
    ],
    "https://atomicdata.dev/properties/isA": [
      "https://atomicdata.dev/classes/Person"
    ],
    "https://atomicdata.dev/properties/localId": "3.0John Examplara",
    "https://atomicdata.dev/properties/name": "John Examplara",
    "https://atomicdata.dev/properties/phoneNumbers": [
      {
        "https://atomicdata.dev/properties/description": "TYPE=HOME,",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/PhoneNumber"
        ],
        "https://atomicdata.dev/properties/phoneNumber": "00 31 6 12345678"
      }
    ],
    "https://atomicdata.dev/properties/vCardUnknowns": [
      {
        "https://atomicdata.dev/properties/atom/value": ";;Randomstreet 31;Amsterdam;;1234 AB;NL;Randomstreet 31\\n1234 ABAmsterdam\\nNL",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item1.ADR"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "ACME inc.",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item2.ORG"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Chief of Staff",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "item3.TITLE"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "Really cool guy",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "NOTE"
      },
      {
        "https://atomicdata.dev/properties/atom/value": "myContacts",
        "https://atomicdata.dev/properties/isA": [
          "https://atomicdata.dev/classes/VCardUnknown"
        ],
        "https://atomicdata.dev/properties/key": "CATEGORIES"
      }
    ]
  }
]"#;

#[test]
fn parse_vcard() {
  let output = parse_vcard_file(BufReader::new(File::open("./vcards.vcf").unwrap()));
  let expected: serde_json::Value = serde_json::from_str(&EXPECTED).unwrap();
  assert_eq!(output, expected);
}
