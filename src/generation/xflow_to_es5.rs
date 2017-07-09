use structure::xflow::*;

use ratel::{transformer, parser, codegen};

pub fn output(xflow: &XFlowDocument) -> String {
    let res = build_class(&xflow);
    output_js(&res)
}

pub fn output_es5(xflow: &XFlowDocument) -> String {
    let res = build_class(&xflow);
    output_js_es5(&res)
}

fn output_js(input_program: &str) -> String {
    let mut ast = parser::parse(input_program.to_string()).expect("Must compile");
    transformer::transform(&mut ast, transformer::Settings::no_transform());
    codegen::generate_code(&ast, true)
}

fn output_js_es5(input_program: &str) -> String {
    let mut ast = parser::parse(input_program.to_string()).expect("Must compile");
    transformer::transform(&mut ast, transformer::Settings::target_es5());
    codegen::generate_code(&ast, true)
}

fn build_class(xflow: &XFlowDocument) -> String {

    let out = format!(
        r#"
class Zork {{
    constructor(input_variables) {{
        input_variables.forEach((i)=> {{
            this[i.name] = i.value;
        }}, this);
        {local_variables}
    }}

    node_id_X() {{
    }}

    node_id_Y() {{
    }}
}}
"#,
        local_variables = local_variables(&xflow)
    );
    out
}

fn local_variables(xflow: &XFlowDocument) -> String {
    let vars: Vec<String> = xflow
        .doc
        .variables
        .local
        .iter()
        .map({
                 |v| {
                     format!("this.{name} = {value:?};",
                             name = v.name,
                             value = v.value.string_value())
                 }
             })
        .collect();
    vars.join("\r")

}
