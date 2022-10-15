use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::Write, sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize { COUNTER.fetch_add(1, Ordering::Relaxed) }

fn to_txl(mut file: &File, node: &Value) -> String {
    let node_type = node.as_object().unwrap()["type"].as_str().unwrap();
    match node_type {
        "REPEAT" => {
            let sub_node = &node.as_object().unwrap()["content"];
            if sub_node.as_object().unwrap()["type"] != "SEQ" && sub_node.as_object().unwrap()["type"] != "CHOICE"{
                let s = to_txl(file, sub_node);
                let u = &s[1..s.len() - 1];
                format!("[{}*]", u)    
            } else {
                let s = to_txl(file, sub_node);
                let kk = format!("seq_{}", get_id());
                write!(file, "\ndefine {}", kk).ok();
                write!(file, "\n    {}", s).ok();
                writeln!(file, "\nend define").ok();
                format!("[{}*]", kk)    
            }
        }
        "SYMBOL" => {
            let mut name = node["name"].as_str().unwrap();
            match name {
                "identifier" => {
                    name = "id"
                }
                "comment" => {
                    name = "COMMENT"
                }
                _ => {}
            }
            format!("[{}]", name)
        }
        "TOKEN" => {
            format!("{:?}", to_txl(file, &node.as_object().unwrap()["content"]))
        }
        "IMMEDIATE_TOKEN" => {
            format!("{:?}", to_txl(file, &node.as_object().unwrap()["content"]))
        }
        "STRING" => {
            format!("'{}", node.as_object().unwrap()["value"].as_str().unwrap())
        }
        "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
            let obj = &node.as_object().unwrap()["content"];
            to_txl(file, obj)
        }
        "SEQ" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            members
                .iter()
                .map(|x| to_txl(file, x))
                .collect::<Vec<String>>()
                .join(" ")
        }
        "CHOICE" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            format!(
                "({})",
                members
                    .iter()
                    .map(|x| { to_txl(file, x) })
                    .collect::<Vec<String>>()
                    .join("|")
            )
        }
        "ALIAS" => "".to_string(),
        "BLANK" => "[SP]".to_string(),
        "PATTERN" => node.as_object().unwrap()["value"].as_str().unwrap().to_string(),
        "FIELD" => {
            format!(
                "[IN] [NL] [SPOFF] '{} ': [SPON] {:?}",
                node.as_object().unwrap()["name"].as_str().unwrap(),
                to_txl(file, &node.as_object().unwrap()["content"])
            )
        }
        _ => {
            format!("[{:?}]", node.as_object())
        }
    }
}

fn tree_sitter_to_txl() -> Result<(), std::io::Error> {
    let data = json!(fs::read_to_string("examples/rust.json")?);
    let v: Value = serde_json::from_str(data.as_str().unwrap())?;
    let language = &format!("{}.grm", v["name"].as_str().unwrap());
    let mut file = File::create(language)?;
    let file2 = File::create(&format!("{}-seq.grm", v["name"].as_str().unwrap()))?;

    // writeln!(file, "include \"tree-sitter.grm\"")?;
    writeln!(file, "include \"{}\"", &format!("{}-seq.grm", v["name"].as_str().unwrap()))?;
    let start_rule_name = v["start_rule"].as_str().unwrap();
    writeln!(
        file,
        "\ndefine program\n    [{}] [NL]\nend define",
        start_rule_name
    )?;
    let rules = v["rules"].as_object().unwrap();
    for (k, v) in rules {
        let rule = v.as_object().unwrap();
        let t = rule["type"].as_str().unwrap();
        match t {
            "REPEAT" => {
                let s = to_txl(&file2, &rule["content"]);
                let d = &s[1..s.len()-1];
                writeln!(
                    file,
                    "\ndefine {}\n    [{}*]\nend define",
                    k,
                    d
                )?;
            }
            "CHOICE" => {
                let mut kk = String::from(k);
                if let "comment" = k.as_str() {
                    kk = String::from("COMMENT");
                }
                writeln!(file, "\ndefine {}", kk)?;
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
                            writeln!(file, "{}", to_txl(&file2, &elem)).ok();
                        }
                        false
                    });
                writeln!(file, "end define")?;
            }
            "SEQ" => {
                let mut kk = String::from(k);
                if let "comment" = k.as_str() {
                    kk = String::from("COMMENT");
                }
                writeln!(file, "\ndefine {}", kk)?;
                rule["members"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(true, |first, elem| {
                        write!(file, "    ").ok();
                        let elem_type = elem.as_object().unwrap()["type"].as_str().unwrap();
                        if !first && elem_type != "ALIAS" && elem_type != "PATTERN" {
                            write!(file, "").ok();
                        } else {
                            write!(file, "  ").ok();
                        }
                        if elem_type != "ALIAS" {
                            writeln!(file, "{}", to_txl(&file2, &elem)).ok();
                        }
                        false
                    });
                writeln!(file, "end define")?;
            }
            "FIELD" => {
                let mut kk = String::from(k);
                if let "comment" = k.as_str() {
                    kk = String::from("COMMENT");
                }
                writeln!(file, "\ndefine {}", kk)?;
                writeln!(file, "{}", to_txl(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }
            "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "{}", to_txl(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }    
            "STRING" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "'{}", &rule["value"].as_str().unwrap()).ok();
                writeln!(file, "end define")?;
            }
            "IMMEDIATE_TOKEN" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "{}", to_txl(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }
            _ => {}
        }
    }

    // writeln!(file, "\nredefine Tree",)?;
    // for k in rules.keys() {
    //     writeln!(file, "    [{}] |", k)?;
    // }
    // writeln!(file, "   ...\nend define")?;
    Ok(())
}

fn main() {
    tree_sitter_to_txl().ok();
}
