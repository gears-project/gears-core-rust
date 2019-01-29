use crate::structure::domain::*;

pub fn output(doc: &DomainDocument) -> String {
    build_graph(&doc)
}

fn build_graph(doc: &DomainDocument) -> String {
    let (nodes, edges) = build_nodes_and_edges(&doc);
    let out =
        format!(
        r#"
digraph G {{
{nodes}

{edges}
}}
"#,
        nodes = nodes.join("\n"),
        edges = edges.join("\n"),
    );
    out
}

fn build_nodes_and_edges(doc: &DomainDocument) -> (Vec<String>, Vec<String>) {
    let mut nodes = Vec::<String>::new();
    let mut edges = Vec::<String>::new();

    for entity in &doc.body.entities {
        nodes.push(build_entity_node(&entity));
        for attr in &entity.attributes {
            nodes.push(build_attribute_node(&attr));
            edges.push(build_edge(id_for_entity(&entity), id_for_attribute(&attr)));

            for val in &attr.validations {
                nodes.push(build_validation_node(&val));
                edges.push(build_edge(id_for_attribute(&attr), id_for_validation(&val)));
            }
        }
        for reference in &entity.references {
            nodes.push(build_reference_node(&reference));
            edges.push(build_edge(
                id_for_entity(&entity),
                id_for_reference(&reference),
            ));
        }
    }

    (nodes, edges)
}

fn build_edge(start: String, end: String) -> String {
    format!("  {start} -> {end}", start = start, end = end)
}

fn build_entity_node(el: &Entity) -> String {
    format!(
        "  {id}[label={name},shape=octagon,style=filled,color=skyblue];",
        id = id_for_entity(&el),
        name = el.name
    )
}

fn build_attribute_node(el: &Attribute) -> String {
    format!(
        "  {id}[label={name},style=filled];",
        id = id_for_attribute(&el),
        name = el.name
    )
}

fn build_reference_node(el: &Reference) -> String {
    format!(
        "  {id}[label={name},style=filled,color=green];",
        id = id_for_reference(&el),
        name = el.name,
    )
}

fn build_validation_node(el: &Validation) -> String {
    format!(
        "  {id}[label={name},style=filled,color=pink];",
        id = id_for_validation(&el),
        name = el.message.key
    )
}

fn id_for_entity(el: &Entity) -> String {
    format!("entity_{name}", name = el.name)
}

fn id_for_attribute(el: &Attribute) -> String {
    format!("attribute_{name}", name = el.name)
}

fn id_for_reference(el: &Reference) -> String {
    format!("reference_{name}", name = el.name)
}

fn id_for_validation(el: &Validation) -> String {
    format!("validation_{name}", name = el.message.key)
}

/*
fn build_node(node: &DomainNode) -> String {
    match node.nodetype {
        DomainNodeType::Flow => {
            format!(
                "  node_{id}[label={label},shape=octagon,style=filled,color=skyblue];",
                id = node.id,
                label = node.label,
                )
        }
        DomainNodeType::Flox => {
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

fn build_nodes(domain: &DomainDocument) -> String {
    let vars: Vec<String> = domain
        .doc
        .nodes
        .iter()
        .map({
            |node| build_node(&node)
        })
        .collect();
    vars.join("\n")
}

fn build_edges(domain: &DomainDocument) -> String {
    let vars: Vec<String> = domain
        .doc
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
*/
