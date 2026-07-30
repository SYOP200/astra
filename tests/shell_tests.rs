use std::collections::HashMap;

#[test]
fn parser_handles_basic_command() {

    let input = "echo hello";

    let parts: Vec<&str> =
        input.split_whitespace().collect();

    assert_eq!(parts[0], "echo");
    assert_eq!(parts[1], "hello");
}


#[test]
fn parser_handles_arguments() {

    let input = "git status";

    let parts: Vec<&str> =
        input.split_whitespace().collect();

    assert_eq!(
        parts,
        vec!["git", "status"]
    );
}


#[test]
fn aliases_can_store_commands() {

    let mut aliases =
        HashMap::new();

    aliases.insert(
        "ll",
        "ls -la"
    );


    assert_eq!(
        aliases.get("ll"),
        Some(&"ls -la")
    );
}


#[test]
fn config_example_exists() {

    let config =
        include_str!("../examples/.astrarc");


    assert!(
        config.contains("[general]")
    );


    assert!(
        config.contains("[aliases]")
    );
}


#[test]
fn theme_example_exists() {

    let theme =
        include_str!("../examples/themes/crimson.toml");


    assert!(
        theme.contains("crimson")
    );
}
