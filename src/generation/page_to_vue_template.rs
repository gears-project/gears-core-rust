use structure::page::*;

pub fn output_html(page: &PageDocument) -> String {
    output_components(&page.doc.components)
}

fn output_components(components: &[Component]) -> String {
    let mut out = Vec::<String>::new();

    for component in components {
        let res = match component {
            &Component::Row(ref c) => render_row(c),
            &Component::Header1(ref c) => render_header1(c),
            &Component::Header2(ref c) => render_header2(c),
            &Component::Header3(ref c) => render_header3(c),
            &Component::Column3(ref c) => render_column3(c),
            &Component::Column6(ref c) => render_column6(c),
            &Component::Column12(ref c) => render_column12(c),
            &Component::Datatable(ref c) => render_datatable(c),
            &Component::Form(ref c) => render_form(c),
            &Component::TextInput(ref c) => render_text_input(c),
            &Component::Label(ref c) => render_label(c),
            &Component::Button(ref c) => render_button(c),
            &Component::FormControlGroup(ref c) => render_form_control_group(c),
        };
        out.push(res)
    }
    out.join("")
}

fn render_row(el: &RowComponent) -> String {
    let body = output_components(&el.components);
    let out = format!("<div class='row'>{body}</div>", body = body);
    out
}
fn render_header1(el: &Header1Component) -> String {
    let out = format!("<h1>{text}</h1>", text = el.config.text.value);
    out
}
fn render_header2(el: &Header2Component) -> String {
    let out = format!("<h2>{text}</h2>", text = el.config.text.value);
    out
}
fn render_header3(el: &Header3Component) -> String {
    let out = format!("<h3>{text}</h3>", text = el.config.text.value);
    out
}
fn render_column3(el: &Column3Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-3'>
        {body}
    </div>
    "#,
        body = body
    );
    out
}
fn render_column6(el: &Column6Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-6'>
        {body}
    </div>
    "#,
        body = body
    );
    out
}
fn render_column12(el: &Column12Component) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <div class='col-md-12'>
        {body}
    </div>
    "#,
        body = body
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
fn render_form(el: &FormComponent) -> String {
    let body = output_components(&el.components);
    let out = format!(
        r#"
    <form
      class='form'
      :entity="{entity}"
      >
      {body}
    </div>
    "#,
        entity = el.config.entity,
        body = body
    );
    out
}
fn render_text_input(el: &TextInputComponent) -> String {
    let out = format!(
        r#"
    <input class='input'>
        {placeholder}
    </div>
    "#,
        placeholder = el.config.placeholder.value
    );
    out
}
fn render_label(el: &LabelComponent) -> String {
    let out = format!(
        r#"
    <label class='label'>
        {text}
    </div>
    "#,
        text = el.config.text.value
    );
    out
}
fn render_button(el: &ButtonComponent) -> String {
    let out = format!(
        r#"
    <button class='button'>
        {text}
    </div>
    "#,
        text = el.config.text.value
    );
    out
}
fn render_form_control_group(el: &FormControlGroupComponent) -> String {
    let out = format!(
        r#"
    <form>
        <FormGroup
            validationState="success"
        >
            <ControlLabel>{label}</ControlLabel>
            <FormControl
                type="{form_control_type}"
                placeholder="{placeholder}
				value="XXXvalueXXX"
            />
            <FormControl.Feedback />
            <HelpBlock>XXXvalidation_messageXXX</HelpBlock>
        </FormGroup>
    </form>
    "#,
        label = el.config.label.value,
        placeholder = el.config.placeholder.value,
        form_control_type = el.config.form_control_type.to_text()
    );
    out
}
