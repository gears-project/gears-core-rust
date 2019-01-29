use crate::structure::xflow::*;
use crate::util::naming::uuid_to_label;

use ratel::{transformer, parser, codegen};

// partof: #SPC-artifact-generation-xflow

pub fn output(doc: &XFlowDocument) -> String {
    let res = build_class(&doc);
    output_js(&res)
}

pub fn output_es5(doc: &XFlowDocument) -> String {
    let res = build_class(&doc);
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

fn build_class(doc: &XFlowDocument) -> String {

    let out = format!(
        r#"
class XFlow_{id} {{
    /* xflow label : {name} */

    local_vars = {{}}
    input_vars = {{}}
    output_vars = {{}}

    constructor(input_variables) {{
        input_variables.forEach((i)=> {{
            this.local_vars[i.name] = i.value;
        }}, this);
        {local_variables}
    }}
    run(callback) {{
        this.callback = callback;
    }}
    exists(x) {{
        return ((x != null) && (x != undefined));
    }}
    finalize(){{
        for (name in this.output_vars) {{
            if (this.exists(this.local_vars[name])) {{
                this.output_vars[name] = this.local_vars[name];
            }} else if (this.exists(this.input_vars[name])) {{
                this.output_vars[name] = this.input_vars[name];
            }} else {{
                throw new Error("No var found");
            }}
        }}
    }}
    {nodes}
}}
"#,
        id = uuid_to_label(&doc.id),
        name = &doc.name,
        local_variables = local_variables(&doc),
        nodes = build_nodes(&doc)
    );
    out
}

fn method_name_for_node_id(id: &i32) -> String {
    format!("node_{id}", id = id)
}

fn local_variables(doc: &XFlowDocument) -> String {
    let vars: Vec<String> = doc.body
        .variables
        .local
        .iter()
        .map({
            |v| {
                format!(
                    "this.input_vars.{name} = {value};",
                    name = v.name,
                    value = v.value.string_value()
                )
            }
        })
        .collect();
    vars.join("\r")

}

fn build_nodes(doc: &XFlowDocument) -> String {
    let vars: Vec<String> = doc.body
        .nodes
        .iter()
        .map({
            |node| {
                format!(
                    r#" {fn_id}() {{ {body} }}  "#,
                    fn_id = method_name_for_node_id(&node.id),
                    body = build_function_body(&node, &doc)
                )
            }
        })
        .collect();
    vars.join("\r")

}

fn build_function_body(node: &XFlowNode, doc: &XFlowDocument) -> String {
    let body = match node.nodetype {
        XFlowNodeType::Flow => build_xflow_body(&node, &doc),
        XFlowNodeType::Flox => build_flox_body(&node, &doc),
        XFlowNodeType::Call => "call_xflow();".to_owned(),
    };

    format!(
        "console.log('node-{id} called'); {body}",
        id = node.id,
        body = body
    )
}

fn build_xflow_body(node: &XFlowNode, doc: &XFlowDocument) -> String {

    match node.action.as_ref() {
        "start" => build_node_body_call_next_node(node, doc),
        "end" => format!("this.finalize(); this.callback(this.output_vars);"),
        "branch" => build_node_body_branch(node, doc),
        _ => format!("unimplemented();"),

    }
}

fn build_flox_body(node: &XFlowNode, doc: &XFlowDocument) -> String {
    format!(
        r#"
    console.log('flox node');
    {call_next_node}
    "#,
        call_next_node = build_node_body_call_next_node(node, doc)
    )
}

fn build_node_body_call_next_node(node: &XFlowNode, doc: &XFlowDocument) -> String {
    let edges = doc.body.get_out_edges(node);
    match edges.len() {
        0 => {
            error!("build_node_body_call_next_node: Unable to find a connecting node");
            format!("")
        }
        1 => {
            format!(
                "this.{fn_id}();",
                fn_id = method_name_for_node_id(&edges[0].1)
            )
        }
        _ => {
            error!(
                "build_node_body_call_next_node: Multiple connecting nodes from non-branch node "
            );
            format!("")
        }
    }
}

fn build_node_body_branch(node: &XFlowNode, doc: &XFlowDocument) -> String {
    let edges = doc.body.get_out_edges(node);
    match edges.len() {
        0 => {
            error!("build_node_body_branch: Unable to find a connecting node");
            format!("")
        }
        1 => {
            error!("build_node_body_branch: Only one connecting node for a branch");
            format!(
                "this.{fn_id}();",
                fn_id = method_name_for_node_id(&edges[0].1)
            )
        }
        _ => {
            let mut res: Vec<String> = doc.body
                .get_out_branches(node.id)
                .iter()
                .map({
                    |branch| {
                        format!(
                            r#"
                            if ({var} == {condition}) {{
                                this.{fn_id}();
                            }}
                             "#,
                            var = branch.xvar.name,
                            condition = branch.xvar.value.string_value(),
                            fn_id = method_name_for_node_id(&branch.edge.1)
                        )
                    }
                })
                .collect();
            res.push(format!("throw new Error('Unhandled branch');"));
            res.join(";")
        }
    }
}
