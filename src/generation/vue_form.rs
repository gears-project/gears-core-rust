use structure::form::*;

pub fn output_html(form: &FormDocument) -> String {
    output_components(&form.doc.components)
}

fn output_components(components: &Components) -> String {
    let mut out = Vec::<String>::new();

    for component in components {
        let res = match *component {
            Component::Row(ref c) => render_row(c),
            Component::Header1(ref c) => render_header1(c),
            Component::Header2(ref c) => render_header2(c),
            Component::Header3(ref c) => render_header3(c),
            Component::Column3(ref c) => render_column3(c),
            Component::Column6(ref c) => render_column6(c),
            Component::Column12(ref c) => render_column12(c),
            Component::Datatable(ref c) => render_datatable(c),
        };
        out.push(res)
    }
    out.join("")
}

fn render_row(el: &RowComponent) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='row'>
        {body}
    </div>
    "#,
        body = body
    );
    out
}
fn render_header1(el: &Header1Component) -> String {
    let out = format!("<h1>{}</h1>", el.config.text);
    out
}
fn render_header2(el: &Header2Component) -> String {
    let out = format!("<h2>{}</h2>", el.config.text);
    out
}
fn render_header3(el: &Header3Component) -> String {
    let out = format!("<h3>{}</h3>", el.config.text);
    out
}
fn render_column3(el: &Column3Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-3'>
        {}
    </div>
    "#,
        body
    );
    out
}
fn render_column6(el: &Column6Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-6'>
        {}
    </div>
    "#,
        body
    );
    out
}
fn render_column12(el: &Column12Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-12'>
        {}
    </div>
    "#,
        body
    );
    out
}
fn render_datatable(el: &DatatableComponent) -> String {
    let out = format!(
        r#"
    <div
      class='table'
      :entity="{entity}"
      :attributes="{attributes}"
      >
    </div>
    "#,
        entity = el.config.entity,
        attributes = el.config.attributes.join(",")
    );
    out
}
