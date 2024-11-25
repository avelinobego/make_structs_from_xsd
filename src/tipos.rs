use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "@attributeFormDefault")]
    pub attribute_form_default: String,
    #[serde(rename = "@xmlns:xs")]
    pub xmlns: String,
    pub complex_type: Vec<ComplexType>,
    pub simple_type: Vec<SimpleType>,
}

/*----------------------------------------------------------------------------------------------*/

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    #[serde(rename = "@name")]
    pub name: String,
    pub annotation: Option<Documentation>,
    pub sequence: Element,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    pub documentation: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub element: Vec<Complex>,
}

//Aqui será um enum pra contempla os tipos complexo e simples
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Complex {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub value_type: Option<String>,
}

/*----------------------------------------------------------------------------------------------*/

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleType {
    #[serde(rename = "@name")]
    pub name: String,
    pub annotation: Option<Documentation>,
    pub restriction: Restriction
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    #[serde(rename = "@base")]
    pub base: String,
}