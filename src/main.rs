use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::Write,
};

fn to_txl(node: &Value) -> String {
    match node.as_object().unwrap()["type"].as_str().unwrap() {
        "SYMBOL" => {
            format!(
                "[{}]",
                node["name"].as_str().unwrap()
            )
        }
        _ => {
            "".to_string()
        }
    }
}

fn tree_sitter_to_txl() -> Result<(), std::io::Error> {
    let data = json!(fs::read_to_string("examples/rust.json")?);
    let v: Value = serde_json::from_str(data.as_str().unwrap())?;
    let language = &format!("{}.grm", v["name"].as_str().unwrap());
    let mut file = File::create(language)?;
    writeln!(file, "include \"tree-sitter.grm\"")?;
    let start_rule_name = v["start_rule"].as_str().unwrap();
    writeln!(
        file,
        "\nredefine program\n    [{}] [NL]\nend define",
        start_rule_name
    )?;
    let rules = v["rules"].as_object().unwrap();
    for (k, v) in rules {
        let rule = v.as_object().unwrap();
        let t = rule["type"].as_str().unwrap();
        match t {
            "REPEAT" => {
                writeln!(
                    file,
                    "\ndefine {}\n    '( '{} [Range]\n        [{}*] ')\nend define",
                    k,
                    k,
                    rule["content"].as_object().unwrap()["name"]
                        .as_str()
                        .unwrap()
                )?;
            }
            "CHOICE" => {
                writeln!(file, "\ndefine {}", k)?;
                rule["members"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(true, |first, elem| {
                        write!(file, "    ").ok();
                        let elem_type = elem.as_object().unwrap()["type"].as_str().unwrap();
                        if !first && elem_type != "ALIAS" && elem_type != "PATTERN" {
                            write!(file, "| ").ok();
                        } else {
                            write!(file, "  ").ok();
                        }
                        if elem_type != "ALIAS" {
                            match elem_type {
                                "SYMBOL" => {
                                    writeln!(
                                        file,
                                        "[{}]",
                                        elem.as_object().unwrap()["name"].as_str().unwrap()
                                    )
                                    .ok();
                                }
                                "STRING" => {
                                    writeln!(
                                        file,
                                        "'{}",
                                        elem.as_object().unwrap()["value"].as_str().unwrap()
                                    )
                                    .ok();
                                }
                                "PREC_LEFT" | "PREC" => {
                                    let obj =
                                        elem.as_object().unwrap()["content"].as_object().unwrap();
                                    match obj["type"].as_str().unwrap() {
                                        "SYMBOL" => {
                                            writeln!(file, "[{}]", obj["name"].as_str().unwrap())
                                                .ok();
                                        }
                                        "STRING" => {
                                            writeln!(file, "'{}", obj["value"].as_str().unwrap())
                                                .ok();
                                        }
                                        "SEQ" => {
                                            let members = obj["members"].as_array().unwrap();
                                            members.iter().fold(true, |first, next| {
                                                if !first {
                                                    write!(file, " ").ok();
                                                }
                                                match next["type"].as_str().unwrap() {
                                                    "SYMBOL" => {
                                                        write!(
                                                            file,
                                                            "[{}]",
                                                            next["name"].as_str().unwrap()
                                                        )
                                                        .ok();
                                                    }
                                                    "STRING" => {
                                                        write!(
                                                            file,
                                                            "'{}",
                                                            next["value"].as_str().unwrap()
                                                        )
                                                        .ok();
                                                    }
                                                    "REPEAT" => {
                                                        write!(
                                                            file,
                                                            "[{}*]",
                                                            next["content"].as_object().unwrap()
                                                                ["name"]
                                                                .as_str()
                                                                .unwrap()
                                                        )
                                                        .ok();
                                                    }
                                                    "FIELD" => {
                                                        writeln!(
                                                            file,
                                                            "[IN] [NL] [SPOFF] '{} ': [SPON] {}",
                                                            next["name"].as_str().unwrap(),
                                                            to_txl(&next["content"])
                                                        )
                                                        .ok();
                                                    }
                                                    _ => {
                                                        write!(file, "[{:?}]", next).ok();
                                                    }
                                                }
                                                false
                                            });
                                            writeln!(file, "").ok();
                                        }
                                        "FIELD" => {
                                            write!(file, "[{}]", elem_type).ok();
                                        }
                                        _ => {
                                            write!(file, "[{}]", elem_type).ok();
                                            writeln!(file, "[{:?}]", obj["type"]).ok();
                                        }
                                    }
                                }
                                "SEQ" => {
                                    let members =
                                        elem.as_object().unwrap()["members"].as_array().unwrap();
                                    members.iter().fold(true, |first, next| {
                                        if !first {
                                            write!(file, " ").ok();
                                        }
                                        match next["type"].as_str().unwrap() {
                                            "SYMBOL" => {
                                                write!(
                                                    file,
                                                    "[{}]",
                                                    next["name"].as_str().unwrap()
                                                )
                                                .ok();
                                            }
                                            "STRING" => {
                                                write!(
                                                    file,
                                                    "'{}",
                                                    next["value"].as_str().unwrap()
                                                )
                                                .ok();
                                            }
                                            "REPEAT" => {
                                                write!(
                                                    file,
                                                    "[{}*]",
                                                    next["content"].as_object().unwrap()["name"]
                                                        .as_str()
                                                        .unwrap()
                                                )
                                                .ok();
                                            }
                                            "FIELD" => {
                                                let field_name = next["name"].as_str().unwrap();
                                                let field_content =
                                                    next["content"].as_object().unwrap();
                                                writeln!(
                                                    file,
                                                    "'{} ': [{:?}]",
                                                    field_name, field_content
                                                )
                                                .ok();
                                            }
                                            _ => {
                                                write!(file, "[{:?}]", next).ok();
                                            }
                                        }
                                        false
                                    });
                                    writeln!(file, "").ok();
                                }
                                "ALIAS" | "PATTERN" => {
                                    // ignore
                                }
                                _ => {
                                    writeln!(file, "[{:?}]", elem.as_object()).ok();
                                }
                            }
                        }
                        false
                    });
                writeln!(file, "end define")?;
            }
            "SEQ" => {}
            _ => {}
        }
    }

    writeln!(file, "\nredefine Tree",)?;
    for k in rules.keys() {
        writeln!(file, "    [{}] |", k)?;
    }
    writeln!(file, "   ...\nend define")?;
    Ok(())
}

fn main() {
    tree_sitter_to_txl().ok();
}
