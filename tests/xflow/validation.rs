#[cfg(test)]
mod test {

    extern crate xfdocs;
    use self::xfdocs::xflow::xfstruct::*;
    use self::xfdocs::xflow::validation::*;

    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn read_json_file(filename:&str) -> String {
        // Create a path to the desired file
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!(
                "couldn't open {}: {}",
                display,
                Error::description(&why)
                ),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_)    => {}, // print!("{} contains:\n{}", display, s),
        };

        s
    }

#[test]
    fn test_init_validation() {
        let json_string = read_json_file("data/flows/10_steps.json");
        let xfs = XFlowStruct::from_json(&json_string);
        assert_eq!(xfs.nodes.len(), 10);
    }

#[test]
    fn test_validations_ok() {
        let json_string = read_json_file("data/flows/10_steps.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::all_edges_have_nodes(&xfs);
        assert_eq!(res_a.len(), 0);

        let res_a = Validation::has_one_entry_node(&xfs);
        assert_eq!(res_a.len(), 0);

        let res_a = Validation::has_terminal_nodes(&xfs);
        assert_eq!(res_a.len(), 0);

        let res_a = Validation::all_nodes_have_at_least_one_edge(&xfs);
        assert_eq!(res_a.len(), 0);

        let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);
        assert_eq!(res_a.len(), 0);
    }

#[test]
    fn test_validations_edges_have_nodes() {
        let json_string = read_json_file("data/bad_flows/edges_without_nodes.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::all_edges_have_nodes(&xfs);

        assert_eq!(res_a.len(), 3);
        assert_eq!(res_a[0].paths[0], "/edges/(1, 4)");

    }

#[test]
    fn test_validations_has_one_entry_node() {
        let json_string = read_json_file("data/bad_flows/multiple_entry_nodes.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::has_one_entry_node(&xfs);

        assert_eq!(res_a.len(), 1);
        assert_eq!(res_a[0].paths[0], "/nodes");

    }

#[test]
    fn test_validations_has_one_entry_node_ii() {
        let json_string = read_json_file("data/bad_flows/no_entry_nodes.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::has_one_entry_node(&xfs);

        assert_eq!(res_a.len(), 1);
        assert_eq!(res_a[0].paths[0], "/nodes");

    }

#[test]
    fn test_validations_has_terminal_nodes() {
        let json_string = read_json_file("data/bad_flows/no_terminal_nodes.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::has_terminal_nodes(&xfs);

        assert_eq!(res_a.len(), 1);
        assert_eq!(res_a[0].paths[0], "/nodes");

    }

#[test]
    fn test_all_node_actions_have_matching_requirements() {
        let json_string = read_json_file("data/bad_flows/bad_capabilities.json");
        let xfs = XFlowStruct::from_json(&json_string);

        let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);

        assert_eq!(res_a.len(), 2);
        assert_eq!(res_a[0].paths[0], "/nodes/1");
        assert_eq!(res_a[1].paths[0], "/nodes/3");

    }

}

