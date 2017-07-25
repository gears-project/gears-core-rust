use structure::model::ModelDocument;
use structure::domain;

#[allow(dead_code)]
mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

#[derive(Debug, Eq, PartialEq)]
pub enum ModelComponent {
    XFlow,
    Page,
    Translation,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    Sync,
    Set(String, String),
    List(ModelComponent),
    Generate(ModelComponent, String),
    Destroy(ModelComponent, String),
    Dsl(DslCommand),
}

#[derive(Debug, Eq, PartialEq)]
pub enum DslCommand {
    Domain(DomainCommand),
    XFlow(XFlowCommand),
}

#[derive(Debug, Eq, PartialEq)]
pub enum DomainCommand {
    AddEntity(String),
    RemoveEntity(String),
    AddAttribute(String, String, String),
    RemoveAttribute(String, String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum XFlowCommand {
    AddNode(String),
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    match command_grammar::expression(&input) {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("{:?}", err)),
    }
}

pub fn run_commands(model: &mut ModelDocument, dsl_cmds: Vec<&DslCommand>) -> Result<(), String> {
    for cmd in &dsl_cmds {
        run_command(model, cmd);
    }

    Ok(())

}

pub fn run_command(model: &mut ModelDocument, dsl_cmd: &DslCommand) -> Result<(), String> {
    match *dsl_cmd {
        DslCommand::XFlow(ref xflow_command) => {
            use structure::xflow::*;
            println!("Unimplemented!");
            Ok(())
        }
        DslCommand::Domain(ref domain_command) => {

            info!("Domain command: {:?}", domain_command);

            match *domain_command {
                DomainCommand::AddEntity(ref entity) => {
                    model.doc.domain.add_entity(domain::Entity::new(&entity))
                }

                DomainCommand::RemoveEntity(ref entity) => model.doc.domain.remove_entity(&entity),

                DomainCommand::AddAttribute(ref entity, ref attribute, ref attribute_type) => {
                    let attribute = domain::Attribute {
                        name: attribute.to_string(),
                        vtype: attribute_type.to_string(),
                        default: "".to_owned(),
                        validations: Vec::<domain::Validation>::new(),
                    };
                    error!("NOT IMPLEMENTED");
                    Ok(())
                }

                DomainCommand::RemoveAttribute(ref entity, ref attribute) => {
                    error!("NOT IMPLEMENTED");
                    Ok(())
                }
            }
        }
    }
}
