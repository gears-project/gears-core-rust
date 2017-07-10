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
    codegen::generate_code(&ast, false)
}

fn output_js_es5(input_program: &str) -> String {
    let mut ast = parser::parse(input_program.to_string()).expect("Must compile");
    transformer::transform(&mut ast, transformer::Settings::target_es5());
    codegen::generate_code(&ast, false)
}

fn build_class(xflow: &XFlowDocument) -> String {

    let out = format!(
        r#"
class {id} {{

    local_vars = {{}}
    input_vars = {{}}
    output_vars = {{}}

    constructor(input_variables) {{
        input_variables.forEach((i)=> {{
            this.local_vars[i.name] = i.value;
        }}, this);
        {local_variables}
    }}
    {nodes}
}}
"#,
        id = &xflow.id,
        local_variables = local_variables(&xflow),
        nodes = build_nodes(&xflow)
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
                     format!("this.input_vars.{name} = {value};",
                             name = v.name,
                             value = v.value.string_value())
                 }
             })
        .collect();
    vars.join("\r")

}

fn build_nodes(xflow: &XFlowDocument) -> String {
    let vars: Vec<String> = xflow
        .doc
        .nodes
        .iter()
        .map({
                 |node| {
                     format!(r#" node_{id}() {{ {body} }}  "#,
                             id = node.id,
                             body = build_function_body(&node, &xflow))
                 }
             })
        .collect();
    vars.join("\r")

}

fn build_function_body(node: &XFlowNode, xflow: &XFlowDocument) -> String {
    let body = match node.nodetype {
        XFlowNodeType::Flow => build_xflow_body(&node, &xflow),
        XFlowNodeType::Flox => "flox_node();".to_owned(),
        XFlowNodeType::Call => "call_xflow();".to_owned(),
    };

    format!("console.log('node-{id} called'); {body}",
            id = node.id,
            body = body)
}

fn build_xflow_body(node: &XFlowNode, xflow: &XFlowDocument) -> String {

    match node.action.as_ref() {
        "start" => {
            let edges = xflow.doc.get_out_edges(node);
            match edges.len() {
                0 => {
                    error!("build_xflow_body: Unable to find a connecting node");
                    format!("")
                }
                1 => format!("this.node_{id}();", id = edges[0].1),
                _ => {
                    error!("build_xflow_body: Multiple connecting nodes from start node ");
                    format!("")
                }
            }
        }
        "end" => format!("end();"),
        "branch" => format!("branch();"),
        _ => format!("unimplemented();"),

    }

}
