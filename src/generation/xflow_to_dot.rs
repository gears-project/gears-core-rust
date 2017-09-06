use structure::xflow::*;

pub fn output(xflow: &XFlowDocument) -> String {
    build_graph(&xflow)
}

fn build_graph(xflow: &XFlowDocument) -> String {
    let out =
        format!(
        r#"
graph G {{
{nodes}

{edges}
}}
"#,
        nodes = build_nodes(&xflow),
        edges = build_edges(&xflow),
    );
    out
}

fn build_node(node: &XFlowNode) -> String {
    match node.nodetype {
        XFlowNodeType::Flow => {
            format!(
                "  node_{id}[label={label},shape=octagon,style=filled,color=skyblue];",
                id = node.id,
                label = node.label,
                )
        }
        _ => {
            format!(
                "  node_{id}[label={label}];",
                id = node.id,
                label = node.label,
                )
        }
    }

}

fn build_nodes(xflow: &XFlowDocument) -> String {
    let vars: Vec<String> = xflow
        .doc
        .nodes
        .iter()
        .map({
            |node| build_node(&node)
        })
        .collect();
    vars.join("\n")
}

fn build_edges(xflow: &XFlowDocument) -> String {
    let vars: Vec<String> = xflow
        .doc
        .edges
        .iter()
        .map({
            |tup| {
                format!(
                    "  node_{start} -- node_{end};",
                    start = tup.0,
                    end = tup.1,
                )
            }
        })
        .collect();
    vars.join("\n")
}
