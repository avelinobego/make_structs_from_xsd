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
        value: Option<Vec<Values>>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Values {
    Annotation(Documentation),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    #[serde(rename = "@base")]
    pub base: String,
}
