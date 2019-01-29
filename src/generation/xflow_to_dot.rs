use crate::structure::xflow::*;

pub fn output(doc: &XFlowDocument) -> String {
    build_graph(&doc)
}

fn build_graph(doc: &XFlowDocument) -> String {
    let out =
        format!(
        r#"
digraph G {{
{nodes}

{edges}
}}
"#,
        nodes = build_nodes(&doc),
        edges = build_edges(&doc),
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
        XFlowNodeType::Flox => {
            format!(
                "  node_{id}[label={label},style=filled,color=pink];",
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

fn build_nodes(doc: &XFlowDocument) -> String {
    let vars: Vec<String> = doc.body
        .nodes
        .iter()
        .map({
            |node| build_node(&node)
        })
        .collect();
    vars.join("\n")
}

fn build_edges(doc: &XFlowDocument) -> String {
    let vars: Vec<String> = doc.body
        .edges
        .iter()
        .map({
            |tup| {
                format!(
                    "  node_{start} -> node_{end};",
                    start = tup.0,
                    end = tup.1,
                )
            }
        })
        .collect();
    vars.join("\n")
}
