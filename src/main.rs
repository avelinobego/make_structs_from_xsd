pub mod tipos;

use std::fs;

use quick_xml::de::from_str;
use tipos::Schema;

fn main() {
    //TODO: Ler o arquivo de tipos XSD

    let file_path = "/home/avelinobego/projetos/docs/esocial/Layout 1.3/2025-01-02_esquemas_xsd_v_s_01_03_00-1/tipos.xsd";

    let arquivo = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("Quantidade de linhas: {}", &arquivo.lines().count());
    // println!("{arquivo}");

    let object: Schema = from_str(&arquivo).unwrap();

    dbg!(object);
}
