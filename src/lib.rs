extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

#[test]
#[should_panic]
fn test_short() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
}

#[test]
#[should_panic]
fn test_short_with_one_param() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
}

#[test]
fn test_short_with_two_params() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
	assert_eq!(args.arg_type, vec!["bool".to_owned()]);
	assert_eq!(args.arg_param, vec!["true".to_owned()]);
}

#[test]
#[should_panic]
fn test_short_with_three_params() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true", "string"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
}

#[test]
#[should_panic]
fn test_short_with_four_params() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true", "string", "hello"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
}

#[test]
#[should_panic]
fn test_two_shorts_with_three_params() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true", "-v", "string"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
}

#[test]
fn test_two_shorts_with_four_params() {
    #[derive(Deserialize)]
    struct Args {
        arg_type: Vec<String>,
        arg_param: Vec<String>,
    }

    const USAGE: &'static str = "
    Usage:
        encode [-v <type> <param>]...
    ";

    let argv = &["encode", "-v", "bool", "true", "-v", "string", "hello"];
    let args: Args = Docopt::new(USAGE).unwrap().argv(argv).deserialize().unwrap();
	assert_eq!(args.arg_type, vec!["bool".to_owned(), "string".to_owned()]);
	assert_eq!(args.arg_param, vec!["true".to_owned(), "hello".to_owned()]);
}
