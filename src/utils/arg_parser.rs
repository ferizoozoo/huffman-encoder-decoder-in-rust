pub fn filename_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Filename is not provided");
    }

    let filename = args[1].clone();
    return Ok(filename);
}

pub fn option_arg_parser(args: Vec<String>) -> Result<String, &'static str> {
    const OUTPUT_FILE_OPTION: &str = "-o";

    if args.len() < 4 {
        return Err("Output option and filename are not provided");
    }

    let index = args
        .iter()
        .position(|arg| arg == OUTPUT_FILE_OPTION)
        .unwrap();
    let filename = args[index + 1].clone();
    return Ok(filename);
}
