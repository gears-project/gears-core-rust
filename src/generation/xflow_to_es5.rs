use structure::xflow::*;

use ratel::{transformer, parser, codegen};

// partof: #SPC-artifact-generation-xflow

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
        id = &xflow.id,
        local_variables = local_variables(&xflow),
        nodes = build_nodes(&xflow)
    );
    out
}

fn method_name_for_node_id(id: &i32) -> String {
    format!("node_{id}", id = id)
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
                     format!(r#" {fn_id}() {{ {body} }}  "#,
                             fn_id = method_name_for_node_id(&node.id),
                             body = build_function_body(&node, &xflow))
                 }
             })
        .collect();
    vars.join("\r")

}

fn build_function_body(node: &XFlowNode, xflow: &XFlowDocument) -> String {
    let body = match node.nodetype {
        XFlowNodeType::Flow => build_xflow_body(&node, &xflow),
        XFlowNodeType::Flox => build_flox_body(&node, &xflow),
        XFlowNodeType::Call => "call_xflow();".to_owned(),
    };

    format!("console.log('node-{id} called'); {body}",
            id = node.id,
            body = body)
}

fn build_xflow_body(node: &XFlowNode, xflow: &XFlowDocument) -> String {

    match node.action.as_ref() {
        "start" => build_node_body_call_next_node(node, xflow),
        "end" => format!("this.finalize(); this.callback(this.output_vars);"),
        "branch" => build_node_body_branch(node, xflow),
        _ => format!("unimplemented();"),

    }
}

fn build_flox_body(node: &XFlowNode, xflow: &XFlowDocument) -> String {
    format!(
        r#"
    console.log('flox node');
    {call_next_node}
    "#,
        call_next_node = build_node_body_call_next_node(node, xflow)
    )
}

fn build_node_body_call_next_node(node: &XFlowNode, xflow: &XFlowDocument) -> String {
    let edges = xflow.doc.get_out_edges(node);
    match edges.len() {
        0 => {
            error!("build_node_body_call_next_node: Unable to find a connecting node");
            format!("")
        }
        1 => {
            format!("this.{fn_id}();",
                    fn_id = method_name_for_node_id(&edges[0].1))
        }
        _ => {
            error!("build_node_body_call_next_node: Multiple connecting nodes from non-branch node ");
            format!("")
        }
    }
}

fn build_node_body_branch(node: &XFlowNode, xflow: &XFlowDocument) -> String {
    let edges = xflow.doc.get_out_edges(node);
    match edges.len() {
        0 => {
            error!("build_node_body_branch: Unable to find a connecting node");
            format!("")
        }
        1 => {
            error!("build_node_body_branch: Only one connecting node for a branch");
            format!("this.{fn_id}();",
                    fn_id = method_name_for_node_id(&edges[0].1))
        }
        _ => {
            let mut res: Vec<String> = xflow
                .doc
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
