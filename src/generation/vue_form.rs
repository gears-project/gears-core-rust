use structure::form;

pub struct VueForm {}

impl VueForm {
    pub fn output_html(form: &form::FormDocument) -> String {
        Self::output_components(&form.doc.components)
    }

    pub fn output_components(components: &form::Components) -> String {
        let mut out = Vec::<String>::new();
        for ref component in components {
            out.push("".to_owned());
        }
        out.join("")

    }
}
