#[test]
fn command_creation() {
    let mut cmd = Command::new("echo");
    cmd.push_arg("hello");

    assert_eq!(cmd.program, "echo");
    assert_eq!(cmd.args, vec!["hello"]);
}

#[test]
fn pipeline_creation() {
    let mut pipe = Pipeline::new();
    pipe.push(Command::new("ls"));

    assert_eq!(pipe.len(), 1);
}

#[test]
fn redirect_creation() {
    let cmd = Command {
        program: "cat".into(),
        args: Vec::new(),
        redirects: vec![Redirect::stdout("out.txt")],
    };

    assert_eq!(cmd.redirects.len(), 1);
}
