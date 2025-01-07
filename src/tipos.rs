use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "@attributeFormDefault")]
    pub attribute_form_default: String,
    #[serde(rename = "@xmlns:xs")]
    pub xmlns: String,
    #[serde(rename = "$value")]
    pub types: Vec<Types>,
}

/*----------------------------------------------------------------------------------------------*/

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Types {
    ComplexType(ComplexType),
    SimpleType(SimpleType),
}

impl Types {
    pub fn has_name(&self) -> bool {
        match self {
            Types::ComplexType(c) => c.name.is_some(),
            Types::SimpleType(s) => s.name.is_some(),
        }
    }

    pub fn name(&self) -> String {
        if self.has_name() {
            match self {
                Types::ComplexType(c) => c.name.clone().unwrap().to_case(Case::UpperCamel),
                Types::SimpleType(s) => s.name.clone().unwrap().to_case(Case::UpperCamel),
            }
        } else {
            "".into()
        }
    }

    pub fn has_doc(&self) -> bool {
        match self {
            Types::ComplexType(c) => c.annotation.is_some(),
            Types::SimpleType(s) => s.annotation.is_some(),
        }
    }

    pub fn docs(&self) -> Vec<String> {
        if self.has_doc() {
            match self {
                Types::ComplexType(c) => c.annotation.clone().unwrap().value,
                Types::SimpleType(s) => s.annotation.clone().unwrap().value,
            }
        } else {
            vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    pub annotation: Option<Documentation>,
    pub sequence: Sequence,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleType {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    pub annotation: Option<Documentation>,
    pub restriction: Option<Restriction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    #[serde(rename = "$value")]
    value: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub element: Vec<Ele>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Ele {
    Element {
        #[serde(rename = "@name")]
        name: String,
        #[serde(rename = "@minOccurs")]
        min_occurs: Option<i32>,
        #[serde(rename = "@maxOccurs")]
        max_occurs: Option<i32>,
        #[serde(rename = "@type")]
        type_name: Option<String>,
        #[serde(rename = "$value")]
        value: Option<Vec<ElementValues>>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ElementValues {
    Annotation(Documentation),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    #[serde(rename = "@base")]
    pub base: String,
    pub enumeration: Option<Vec<Enumeration>>,
    #[serde(rename = "$value")]
    pub values: Option<Vec<RestrictionValues>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]

pub enum RestrictionValues {
    TotalDigits {
        #[serde(rename = "@value")]
        value: u32,
    },
    MinLength {
        #[serde(rename = "@value")]
        value: u32,
    },
    Length {
        #[serde(rename = "@value")]
        value: u32,
    },
    Pattern {
        #[serde(rename = "@value")]
        value: String,
    },
    MaxLength {
        #[serde(rename = "@value")]
        value: u32,
    },
    FractionDigits {
        #[serde(rename = "@value")]
        value: u32,
    },
    MinExclusive {
        #[serde(rename = "@value")]
        value: f32,
    },
    MaxInclusive {
        #[serde(rename = "@value")]
        value: f32,
    },
    MinInclusive {
        #[serde(rename = "@value")]
        value: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enumeration {
    #[serde(rename = "@value")]
    pub value: String,
    pub annotation: Option<Documentation>,
}
