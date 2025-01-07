pub mod tipos;

use std::fs;

use quick_xml::de::from_str;
use serde::Serialize;
use tera::{Context, Tera};
use tipos::Schema;

#[derive(Serialize)]
struct ModuleTemplate {
    types: Vec<TypeTemplate>,
}

#[derive(Serialize)]
struct TypeTemplate {
    name: String,
    docs: Vec<String>,
}

fn main() {
    let file_path = "/home/avelinobego/projetos/docs/esocial/Layout 1.3/2025-01-02_esquemas_xsd_v_s_01_03_00-1/tipos.xsd";
    let arquivo = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let object: Schema = from_str(&arquivo).unwrap();
    // dbg!(object);

    let tera = match Tera::new("templates/**/*.txt") {
        Ok(t) => Some(t),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            None
        }
    };

    let module_template = ModuleTemplate {
        types: object
            .types
            .iter()
            .take_while(|t| t.has_name())
            .map(|t| TypeTemplate {
                name: t.name(),
                docs: t.docs(),
            })
            .collect(),
    };

    let mut context = Context::new();
    context.insert("modules", &module_template);
    // context.insert("vat_rate", &0.20);

    let mut texto = String::new();
    if let Some(t) = tera {
        texto = match t.render("module.txt", &context){
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                String::new()
            }
        }
    }

    println!("{}", texto);

    // let names = object
    //     .types
    //     .iter()
    //     .take_while(|t| t.has_name())
    //     .map(|t| t.to_owned())
    //     .collect();

    // let ctx = ModuleTemplate { types: names };

    // let path = "./output/module.rs";
    // let mut output_file = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .truncate(true)
    //     .open(path)
    //     .unwrap();

    // output_file
    //     .write_all(ctx.render_once().unwrap().as_bytes())
    //     .unwrap();
}
