#[allow(unused_imports)]
use crate::frontend::emitter;
use crate::utils::concats;

pub fn format_template_string(
    template_string: &str,
    arguments: &[String],
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

    template
}

fn format_template(arg: &String, raw_data: &[u8], template: &mut String) {
    match arg.as_str() {
        "short" => {
            if raw_data.len() != 2 {
                log_error("Received data for short is not 2 bytes");
                return;
            }
            replace_next(
                template,
                &i16::from_be_bytes(raw_data[..2].try_into().unwrap_or([0; 2])),
            );
        }
        "int" => {
            if raw_data.len() != 4 {
                log_error("Received data for integer is not 4 bytes");
                return;
            }
            replace_next(
                template,
                &i32::from_be_bytes(raw_data[..4].try_into().unwrap_or([0; 4])),
            );
        }
        "long" => {
            if raw_data.len() != 8 {
                log_error("Received data for long is not 8 bytes");
                return;
            }
            replace_next(
                template,
                &i64::from_be_bytes(raw_data[..8].try_into().unwrap_or([0; 8])),
            );
        }
        "bool" => {
            if raw_data.len() != 1 {
                log_error("Received data for bool is not 1 byte");
                return;
            }
            replace_next(
                template,
                &match raw_data[0] {
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
                &format_with_decimal(f32::from_be_bytes(
                    raw_data[..4].try_into().unwrap_or([0; 4]),
                )),
            );
        }
        "double" => {
            replace_next(
                template,
                &format_with_decimal(f64::from_be_bytes(
                    raw_data[..8].try_into().unwrap_or([0; 8]),
                )),
            );
        }
        /* For future reference: Turbotrace.java sends big-endian UTF16 */
        "char" => {
            replace_next(
                template,
                &String::from_utf16_lossy(
                    &[concats::concat_u8_to_u16(&raw_data[..2]).unwrap_or(0)],
                ),
            );
        }
        "byte" => {
            replace_next(template, &(raw_data[0] as i8));
        }
        _ => {
            log_error(&format!("Unknown argument type: {arg}"));
        }
    }
}

fn replace_next<T: ToString>(string: &mut String, data: &T) {
    if let Some(pos) = string.find("{}") {
        string.replace_range(pos..(pos + 2), &data.to_string());
    }
}

#[allow(unused_variables)]
fn log_error(err_msg: &str) {
    #[cfg(not(test))]
    {
        log::error!("{}", err_msg);
        emitter::internal_error!(err_msg);
    }
}

/* Format f32 & f64 types */
fn format_with_decimal<T>(value: T) -> String
where
    T: Into<f64> + std::fmt::Display,
{
    let formatted_value = format!("{value}");
    if formatted_value.contains('.') {
        formatted_value
    } else {
        format!("{formatted_value}.0")
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_template_string() {
        assert_eq!(
            format_template_string("Hello World", &[], vec!()),
            "Hello World"
        );
    }

    #[test]
    fn test_template_string_byte() {
        assert_eq!(
            format_template_string(
                "{}",
                &["byte".to_string()],
                vec![i8::MAX.to_be_bytes().to_vec()]
            ),
            i8::MAX.to_string()
        );
    }

    #[test]
    fn test_template_string_negative_byte() {
        assert_eq!(
            format_template_string(
                "{}",
                &["byte".to_string()],
                vec![i8::MIN.to_be_bytes().to_vec()]
            ),
            i8::MIN.to_string()
        );
    }

    #[test]
    fn test_template_string_short() {
        assert_eq!(
            format_template_string(
                "{}",
                &["short".to_string()],
                vec![i16::MAX.to_be_bytes().to_vec()]
            ),
            i16::MAX.to_string()
        );
    }

    #[test]
    fn test_template_string_negative_short() {
        assert_eq!(
            format_template_string(
                "{}",
                &["short".to_string()],
                vec![i16::MIN.to_be_bytes().to_vec()]
            ),
            i16::MIN.to_string()
        );
    }

    #[test]
    fn test_template_string_integer() {
        assert_eq!(
            format_template_string(
                "{}",
                &["int".to_string()],
                vec![i32::MAX.to_be_bytes().to_vec()]
            ),
            i32::MAX.to_string()
        );
    }

    #[test]
    fn test_template_string_negative_integer() {
        assert_eq!(
            format_template_string(
                "{}",
                &["int".to_string()],
                vec![i32::MIN.to_be_bytes().to_vec()]
            ),
            i32::MIN.to_string()
        );
    }

    #[test]
    fn test_template_string_long() {
        assert_eq!(
            format_template_string(
                "{}",
                &["long".to_string()],
                vec![i64::MAX.to_be_bytes().to_vec()]
            ),
            i64::MAX.to_string()
        );
    }

    #[test]
    fn test_template_string_negative_long() {
        assert_eq!(
            format_template_string(
                "{}",
                &["long".to_string()],
                vec![i64::MIN.to_be_bytes().to_vec()]
            ),
            i64::MIN.to_string()
        );
    }

    #[test]
    fn test_template_string_bool_true() {
        assert_eq!(
            format_template_string("{}", &["bool".to_string()], vec![vec![1]]),
            "true"
        );
    }

    #[test]
    fn test_template_string_bool_false() {
        assert_eq!(
            format_template_string("{}", &["bool".to_string()], vec![vec![0]]),
            "false"
        );
    }

    #[test]
    fn test_template_string_string() {
        assert_eq!(
            format_template_string(
                "{}",
                &["str".to_string()],
                vec!["Hello World".as_bytes().to_vec()]
            ),
            "Hello World"
        );
    }

    #[test]
    fn test_template_string_float() {
        assert_eq!(
            format_template_string(
                "{}",
                &["float".to_string()],
                vec![f32::MAX.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f32::MAX)
        );
    }

    #[test]
    fn test_template_string_negative_float() {
        assert_eq!(
            format_template_string(
                "{}",
                &["float".to_string()],
                vec![f32::MIN.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f32::MIN)
        );
    }

    #[test]
    fn test_template_string_smallest_positive_float() {
        assert_eq!(
            format_template_string(
                "{}",
                &["float".to_string()],
                vec![f32::MIN_POSITIVE.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f32::MIN_POSITIVE)
        );
    }

    #[test]
    fn test_template_string_double() {
        assert_eq!(
            format_template_string(
                "{}",
                &["double".to_string()],
                vec![f64::MAX.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f64::MAX)
        );
    }

    #[test]
    fn test_template_string_negative_double() {
        assert_eq!(
            format_template_string(
                "{}",
                &["double".to_string()],
                vec![f64::MIN.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f64::MIN)
        );
    }

    #[test]
    fn test_template_string_smallest_positive_double() {
        assert_eq!(
            format_template_string(
                "{}",
                &["double".to_string()],
                vec![f64::MIN_POSITIVE.to_be_bytes().to_vec()]
            ),
            format_with_decimal(f64::MIN_POSITIVE)
        );
    }

    #[test]
    fn test_template_string_capital_w_char() {
        assert_eq!(
            format_template_string("{}", &["char".to_string()], vec![vec![0x00, 0x57]]),
            "W"
        );
    }

    #[test]
    /* Lambda (λ) = 0x03BB */
    fn test_template_string_utf16_char() {
        assert_eq!(
            format_template_string("{}", &["char".to_string()], vec![vec![0x03, 0xBB]]),
            "λ"
        );
    }

    #[test]
    fn test_template_string_all_features() {
        assert_eq!(
            format_template_string(
                "{} = {} = {} = {} = {} = {} is always {} {} {}",
                &vec![
                    "byte".to_string(),
                    "short".to_string(),
                    "int".to_string(),
                    "long".to_string(),
                    "float".to_string(),
                    "double".to_string(),
                    "bool".to_string(),
                    "char".to_string(),
                    "str".to_string()
                ],
                vec![
                    vec![0x01],
                    vec![0x00, 0x01],
                    vec![0x00, 0x00, 0x00, 0x01],
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                    vec![0x3F, 0x80, 0x00, 0x00],
                    vec![0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                    vec![0x01],
                    vec![0x00, 0xB6],
                    "Hello World".as_bytes().to_vec()
                ]
            ),
            "1 = 1 = 1 = 1 = 1.0 = 1.0 is always true ¶ Hello World"
        );
    }
}
