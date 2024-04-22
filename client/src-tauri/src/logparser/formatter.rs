use crate::frontend::emitter;

pub fn format_template_string(
    template_string: &str,
    arguments: &Vec<String>,
    data: Vec<Vec<u8>>,
) -> String {
    if arguments.len() != data.len() {
        log::error!("Preprocessed arguments are not equal to the number of received arguments");
        emitter::internal_error!(
            "Preprocessed arguments are not equal to the number of received arguments"
        );
    }

    let mut template: String = template_string.to_string();
    for (arg, raw_data) in arguments.iter().zip(data.iter()) {
        format_template(arg, raw_data, &mut template);
    }

    return template;
}

fn format_template(arg: &String, raw_data: &Vec<u8>, template: &mut String) {
    match arg.as_str() {
        "int" => {
            if raw_data.len() != 4 {
                log::error!("Received data for integer is not 4 bytes");
                emitter::internal_error!("Received data for integer is not 4 bytes");
                return;
            }
            replace_next(
                template,
                &i32::from_be_bytes(raw_data[..4].try_into().unwrap_or_else(|_| [0, 0, 0, 0]))
                    .to_string(),
            );
        }
        _ => {
            log::error!("No such argument type: {}", arg.as_str());
            emitter::internal_error!(&format!("No such argument type: {}", arg.as_str()));
        }
    }
}

fn replace_next(string: &mut String, data: &String) {
    if let Some(pos) = string.find("{}") {
        string.replace_range(pos..(pos + 2), data);
    }
}
