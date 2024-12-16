pub mod tipos;

use std::fs;

use quick_xml::de::from_str;
use sailfish::TemplateSimple;
use tipos::{Schema, Types};

fn main() {
    let file_path = "/home/avelinobego/projetos/docs/esocial/Layout 1.3/2025-01-02_esquemas_xsd_v_s_01_03_00-1/tipos.xsd";
    let arquivo = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let object: Schema = from_str(&arquivo).unwrap();
    // dbg!(object);

    let path = "./output";
    if let Ok(false) = fs::exists(path) {
        println!("Criando o diretório {path}");
        fs::create_dir(path).expect("Deu zica!");
    } else {
        println!("O diretório {path} já existe");
    }

    let names = object
        .types
        .iter()
        .take_while(|t| t.has_name())
        // .map(|t| t.to_owned())
        .collect();

    let ctx = ModuleTemplate { types: names };


    // Now render templates with given data
    println!("{}", ctx.render_once().unwrap());
}

#[derive(TemplateSimple)] // automatically implement `TemplateSimple` trait
#[template(path = "module.stpl")] // specify the path to template
pub struct ModuleTemplate<'a> {
    // data to be passed to the template
    pub types: Vec<&'a Types>,
}
