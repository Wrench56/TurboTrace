use crate::{frontend::emitter, utils::concats};

pub fn format_template_string(
    template_string: &str,
    arguments: &Vec<String>,
    data: Vec<Vec<u8>>,
) -> String {
    if arguments.len() != data.len() {
        log_error("Preprocessed arguments are not equal to the number of received arguments");
    }

    let mut template: String = template_string.to_string();
    if arguments.len() != template.matches("{}").count() {
        log_error(
            "Preprocessed arguments are not equal to the number of template string arguments",
        );
    }

    for (arg, raw_data) in arguments.iter().zip(data.iter()) {
        format_template(arg, raw_data, &mut template);
    }

    return template;
}

fn format_template(arg: &String, raw_data: &Vec<u8>, template: &mut String) {
    match arg.as_str() {
        "short" => {
            if raw_data.len() != 2 {
                log_error("Received data for short is not 2 bytes");
                return;
            }
            replace_next(
                template,
                &i16::from_be_bytes(raw_data[..2].try_into().unwrap_or_else(|_| [0; 2])),
            );
        }
        "int" => {
            if raw_data.len() != 4 {
                log_error("Received data for integer is not 4 bytes");
                return;
            }
            replace_next(
                template,
                &i32::from_be_bytes(raw_data[..4].try_into().unwrap_or_else(|_| [0; 4])),
            );
        }
        "long" => {
            if raw_data.len() != 8 {
                log_error("Received data for long is not 8 bytes");
                return;
            }
            replace_next(
                template,
                &i64::from_be_bytes(raw_data[..8].try_into().unwrap_or_else(|_| [0; 8])),
            );
        }
        "bool" => {
            if raw_data.len() != 1 {
                log_error("Received data for bool is not 1 byte");
                return;
            }
            replace_next(
                template,
                &match raw_data[1] {
                    0 => "false",
                    _ => "true",
                },
            );
        }
        "str" => {
            replace_next(template, &String::from_utf8_lossy(raw_data));
        }
        "float" => {
            replace_next(
                template,
                &f32::from_be_bytes(raw_data[..4].try_into().unwrap_or_else(|_| [0; 4])),
            );
        }
        "double" => {
            replace_next(
                template,
                &f64::from_be_bytes(raw_data[..8].try_into().unwrap_or_else(|_| [0; 8])),
            );
        }
        /* For future reference: Turbotrace.java sends big-endian UTF16 */
        "char" => {
            replace_next(
                template,
                &String::from_utf16_lossy(&[
                    concats::concat_u8_to_u16(&raw_data[..2]).unwrap_or_else(|_| 0)
                ]),
            );
        }
        "byte" => {
            replace_next(template, &raw_data[0]);
        }
        _ => {
            log_error(&format!("Unknown argument type: {}", arg));
        }
    }
}

fn replace_next<T: ToString>(string: &mut String, data: &T) {
    if let Some(pos) = string.find("{}") {
        string.replace_range(pos..(pos + 2), &data.to_string());
    }
}

fn log_error(err_msg: &str) {
    log::error!("{}", err_msg);
    emitter::internal_error!(err_msg);
}
