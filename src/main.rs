use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn to_txl_1(mut file: &File, node: &Value) -> String {
    let node_type = node.as_object().unwrap()["type"].as_str().unwrap();
    match node_type {
        "REPEAT" => {
            let sub_node = &node.as_object().unwrap()["content"];
            if sub_node.as_object().unwrap()["type"] != "SEQ"
                && sub_node.as_object().unwrap()["type"] != "CHOICE"
            {
                let s = to_txl_1(file, sub_node);
                let u = &s[1..s.len() - 1];
                format!("[{}*]", u)
            } else {
                let s = to_txl_1(file, sub_node);
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
                "identifier" => name = "id",
                "comment" => name = "COMMENT",
                _ => {}
            }
            format!("[{}]", name)
        }
        "TOKEN" => {
            format!(
                "{:?}",
                to_txl_1(file, &node.as_object().unwrap()["content"])
            )
        }
        "IMMEDIATE_TOKEN" => {
            format!(
                "{:?}",
                to_txl_1(file, &node.as_object().unwrap()["content"])
            )
        }
        "STRING" => {
            format!("'{}", node.as_object().unwrap()["value"].as_str().unwrap())
        }
        "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
            let obj = &node.as_object().unwrap()["content"];
            to_txl_1(file, obj)
        }
        "SEQ" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            members
                .iter()
                .map(|x| to_txl_1(file, x))
                .collect::<Vec<String>>()
                .join(" ")
        }
        "CHOICE" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            format!(
                "({})",
                members
                    .iter()
                    .map(|x| { to_txl_1(file, x) })
                    .collect::<Vec<String>>()
                    .join("|")
            )
        }
        "ALIAS" => "".to_string(),
        "BLANK" => "[empty]".to_string(),
        "PATTERN" => node.as_object().unwrap()["value"]
            .as_str()
            .unwrap()
            .to_string(),
        "FIELD" => {
            format!(
                "[IN] [NL] [SPOFF] '{} ': [SPON] {:?}",
                node.as_object().unwrap()["name"].as_str().unwrap(),
                to_txl_1(file, &node.as_object().unwrap()["content"])
            )
        }
        _ => {
            format!("[{:?}]", node.as_object())
        }
    }
}

fn tree_sitter_to_txl_1() -> Result<(), std::io::Error> {
    let data = json!(fs::read_to_string("examples/rust.json")?);
    let v: Value = serde_json::from_str(data.as_str().unwrap())?;
    let language = &format!("{}1.grm", v["name"].as_str().unwrap());
    let mut file = File::create(language)?;
    let file2 = File::create(&format!("{}-seq1.grm", v["name"].as_str().unwrap()))?;

    writeln!(
        file,
        "include \"{}\"",
        &format!("{}-seq1.grm", v["name"].as_str().unwrap())
    )?;
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
                let s = to_txl_1(&file2, &rule["content"]);
                let d = &s[1..s.len() - 1];
                writeln!(file, "\ndefine {}\n    [{}*]\nend define", k, d)?;
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
                            writeln!(file, "{}", to_txl_1(&file2, &elem)).ok();
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
                            writeln!(file, "{}", to_txl_1(&file2, &elem)).ok();
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
                writeln!(file, "{}", to_txl_1(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }
            "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "{}", to_txl_1(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }
            "STRING" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "'{}", &rule["value"].as_str().unwrap()).ok();
                writeln!(file, "end define")?;
            }
            "IMMEDIATE_TOKEN" => {
                writeln!(file, "\ndefine {}", k)?;
                writeln!(file, "{}", to_txl_1(&file2, &rule["content"])).ok();
                writeln!(file, "end define")?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn to_txl_2(mut file: &File, node: &Value) -> String {
    let node_type = node.as_object().unwrap()["type"].as_str().unwrap();
    match node_type {
        "REPEAT" | "REPEAT1" => {
            let sub_node = &node.as_object().unwrap()["content"];
            if sub_node.as_object().unwrap()["type"] != "SEQ"
                && sub_node.as_object().unwrap()["type"] != "CHOICE"
            {
                let s = to_txl_2(file, sub_node);
                if s.len() > 1 {
                    let u = &s[1..s.len() - 1];
                    format!("[{}*]", u)
                } else {
                    "".to_string()
                }
            } else {
                let s = to_txl_2(file, sub_node);
                if s != "" {
                    let kk = format!("seq_{}", get_id());
                    write!(file, "\ndefine {}", kk).ok();
                    write!(file, "\n    {}", s).ok();
                    writeln!(file, "\nend define").ok();
                    format!("[{}*]", kk)
                } else {
                    "".to_string()
                }
            }
        }
        "SYMBOL" => {
            let mut name = node["name"].as_str().unwrap();
            match name {
                "comment" => name = "COMMENT",
                _ => {}
            }
            if name.starts_with("_") {
                "".to_string()
            } else {
                format!("[{}]", name)
            }
        }
        "TOKEN" => {
            format!(
                "{:?}",
                to_txl_2(file, &node.as_object().unwrap()["content"])
            )
        }
        "IMMEDIATE_TOKEN" => {
            format!(
                "{:?}",
                to_txl_2(file, &node.as_object().unwrap()["content"])
            )
        }
        "STRING" => {
            // format!("'{}", node.as_object().unwrap()["value"].as_str().unwrap())
            "".to_string()
        }
        "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
            let obj = &node.as_object().unwrap()["content"];
            to_txl_2(file, obj)
        }
        "SEQ" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            members
                .iter()
                .map(|x| to_txl_2(file, x))
                .collect::<Vec<String>>()
                .join("")
        }
        "CHOICE" => {
            let members = node.as_object().unwrap()["members"].as_array().unwrap();
            let branches = members
                .iter()
                .map(|x| to_txl_2(file, x))
                .filter(|x| x != "")
                .collect::<Vec<String>>();
            if branches.len() > 1 {
                format!("({})", branches.join("|"))
            } else if branches.len() == 1 {
                format!("{}", branches[0])
            } else {
                "".to_string()
            }
        }
        "ALIAS" => "".to_string(),
        "BLANK" => "[empty]".to_string(),
        "PATTERN" => {
            // node.as_object().unwrap()["value"].as_str().unwrap().to_string()
            "".to_string()
        }
        "FIELD" => {
            let mut content = to_txl_2(file, &node.as_object().unwrap()["content"]);
            if content == "" {
                content = String::from("[Tree]");
            }
            format!(
                "[IN] [NL] [SPOFF] '{} ': [SPON] {}\n",
                node.as_object().unwrap()["name"].as_str().unwrap(),
                content
            )
        }
        _ => {
            format!("[{:?}]", node.as_object())
        }
    }
}

fn tree_sitter_to_txl_2() -> Result<(), std::io::Error> {
    let data = json!(fs::read_to_string("examples/rust.json")?);
    let v: Value = serde_json::from_str(data.as_str().unwrap())?;
    let language = &format!("{}2.grm", v["name"].as_str().unwrap());
    let mut file = File::create(language)?;
    let file2 = File::create(&format!("{}-seq2.grm", v["name"].as_str().unwrap()))?;

    writeln!(file, "include \"tree-sitter.grm\"")?;
    writeln!(
        file,
        "include \"{}\"",
        &format!("{}-seq2.grm", v["name"].as_str().unwrap())
    )?;
    let extras = v["extras"].as_array().unwrap();
    for v in extras {
        let rule = v.as_object().unwrap();
        let t = rule["type"].as_str().unwrap();
        match t {
            "SYMBOL" => {
                let k = rule["name"].as_str().unwrap();
                writeln!(file, "\nredefine {}", k)?;
                writeln!(file, "    '( '{} [Range] ')", k)?;
                writeln!(file, "end define")?;
            }
            _ => {}
        }
    }
    let externals = v["externals"].as_array().unwrap();
    for v in externals {
        let rule = v.as_object().unwrap();
        let t = rule["type"].as_str().unwrap();
        match t {
            "SYMBOL" => {
                let k = rule["name"].as_str().unwrap();
                writeln!(file, "\nredefine {}", k)?;
                writeln!(file, "    '( '{} [Range] ')", k)?;
                writeln!(file, "end define")?;
            }
            _ => {}
        }
    }

    let rules = &mut v["rules"].as_object().unwrap();
    for (k, v) in rules.iter() {
        if !k.starts_with("_") {
            let rule = v.as_object().unwrap();
            let t = rule["type"].as_str().unwrap();
            match t {
                "REPEAT" | "REPEAT1" => {
                    let s = to_txl_2(&file2, &rule["content"]);
                    writeln!(file, "\nredefine {}", k)?;
                    write!(file, "    ").ok();
                    if s != "" && s.contains("|") {
                        let kk = format!("seq_{}", get_id());
                        write!(&file2, "\nredefine {}", kk).ok();
                        write!(&file2, "\n    {}", s).ok();
                        writeln!(&file2, "\nend define").ok();
                        writeln!(file, "[{}*]", kk)?;
                    } else if s != "" {
                        let d = &s[1..s.len() - 1];
                        writeln!(file, "[{}*]", d)?;
                    } else {
                        writeln!(file, "    '( '{} [Range] ')", k)?;
                    }
                    writeln!(file, "end define")?;
                }
                "CHOICE" => {
                    let mut kk = String::from(k);
                    if let "comment" = k.as_str() {
                        kk = String::from("COMMENT");
                    }
                    writeln!(file, "\nredefine {}", kk)?;
                    let mut generated_branches: HashSet<String> = HashSet::new();
                    rule["members"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .fold(true, |first, elem| {
                            let elem_type = elem.as_object().unwrap()["type"].as_str().unwrap();
                            let generated = to_txl_2(&file2, &elem);
                            if generated != "" && !generated_branches.contains(&generated) {
                                write!(file, "    ").ok();
                                if !first && elem_type != "ALIAS" && elem_type != "PATTERN" {
                                    write!(file, "| ").ok();
                                } else {
                                    write!(file, "  ").ok();
                                }
                                if elem_type != "ALIAS" {
                                    writeln!(file, "{}", generated).ok();
                                }
                                generated_branches.insert(generated);
                            }
                            false
                        });
                    if generated_branches.len() == 0 {
                        writeln!(file, "    '( '{} [Range] ')", k)?;
                    }
                    writeln!(file, "end define")?;
                }
                "SEQ" => {
                    let mut kk = String::from(k);
                    if let "comment" = k.as_str() {
                        kk = String::from("COMMENT");
                    }
                    let mut generated_branches: Vec<String> = Vec::new();
                    rule["members"].as_array().unwrap().iter().for_each(|e| {
                        // let elem_type = elem.as_object().unwrap()["type"].as_str().unwrap();
                        let elem_value = to_txl_2(&file2, &e);
                        if elem_value != "" {
                            generated_branches.push(elem_value);
                        }
                    });
                    writeln!(file, "\nredefine {}", kk)?;
                    write!(file, "    ").ok();
                    if generated_branches.len() > 0 {
                        write!(file, "{}", generated_branches.join(" ")).ok();
                    } else {
                        writeln!(file, "    '( '{} [Range] ')", k)?;
                    }
                    writeln!(file, "end define")?;
                }
                "FIELD" => {
                    let mut kk = String::from(k);
                    if let "comment" = k.as_str() {
                        kk = String::from("COMMENT");
                    }
                    writeln!(file, "\nredefine {}", kk)?;
                    writeln!(file, "{}", to_txl_2(&file2, &rule["content"])).ok();
                    writeln!(file, "end define")?;
                }
                "PREC_LEFT" | "PREC" | "PREC_RIGHT" => {
                    let content = to_txl_2(&file2, &rule["content"]);
                    writeln!(file, "\nredefine {}", k)?;
                    if content != "" {
                        writeln!(file, "{}", content).ok();
                    } else {
                        writeln!(file, "    '( '{} [Range] ')", k)?;
                    }
                    writeln!(file, "end define")?;
                }
                "STRING" | "PATTERN" => {
                    // writeln!(file, "'{}", &rule["value"].as_str().unwrap()).ok();
                    writeln!(file, "\nredefine {}", k)?;
                    writeln!(file, "    '( '{} [Range] ')", k)?;
                    writeln!(file, "end define")?;
                }
                "IMMEDIATE_TOKEN" | "TOKEN" => {
                    let content = to_txl_2(&file2, &rule["content"]);
                    writeln!(file, "\nredefine {}", k)?;
                    if content != "" {
                        writeln!(file, "{}", content).ok();
                    } else {
                        writeln!(file, "    '( '{} [Range] ')", k)?;
                    }
                    writeln!(file, "end define")?;
                }
                _ => {}
            }
        }
    }
    writeln!(file, "\nredefine Tree",)?;
    for k in rules.keys() {
        if !k.starts_with("_") {
            let mut kk = String::from(k);
            if let "comment" = k.as_str() {
                kk = String::from("COMMENT");
            }
            // if let "identifier" = k.as_str() {
            //     kk = String::from("id");
            // }
            writeln!(file, "    [{}] |", kk)?;
        }
    }
    writeln!(file, "   ...\nend define")?;
    Ok(())
}

fn main() {
    tree_sitter_to_txl_1().ok();
    tree_sitter_to_txl_2().ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn run(s: &str) -> String {
        let mut file = File::create("test.tst").unwrap();
        write!(file, "{}", s).ok();
        let output = Command::new("txl")
            .arg("-q")
            .arg("-s")
            .arg("2000")
            .arg("test.tst")
            .output()
            .expect("txl command failed to start");
        format!("{}", std::str::from_utf8(&output.stdout).unwrap())
    }

    fn runx(s: &str) -> String {
        let mut file = File::create("test.tst").unwrap();
        write!(file, "{}", s).ok();
        let output = Command::new("txl")
            .arg("-q")
            .arg("-s")
            .arg("2000")
            .arg("-x")
            .arg("test.tst")
            .output()
            .expect("txl command failed to start");
        format!("{}", std::str::from_utf8(&output.stdout).unwrap())
    }

    #[test]
    fn run_txl() {
        tree_sitter_to_txl_2().ok();
        insta::assert_snapshot!(run("(identifier [3, 15] - [3, 16])"), 
            @r###"
        (identifier [3, 15] - [3, 16])
        "###);
        insta::assert_snapshot!(runx("(identifier [3, 15] - [3, 16])"), 
            @r###"
        <program>
         <Tree><captured_pattern><identifier> ( identifier
            <Range> [
             <integer_number>3</integer_number> ,
             <integer_number>15</integer_number> ] - [
             <integer_number>3</integer_number> ,
             <integer_number>16</integer_number> ]
            </Range> )
           </identifier>
          </captured_pattern>
         </Tree>
        </program>
        "###);
        insta::assert_snapshot!(run("(line_comment [0, 0] - [0, 66])"), @r###"
        (line_comment [0, 0] - [0, 66])
        "###);
        insta::assert_snapshot!(runx("(line_comment [0, 0] - [0, 66])"), 
            @r###"
        <program>
         <Tree><COMMENT><line_comment> ( line_comment
            <Range> [
             <integer_number>0</integer_number> ,
             <integer_number>0</integer_number> ] - [
             <integer_number>0</integer_number> ,
             <integer_number>66</integer_number> ]
            </Range> )
           </line_comment>
          </COMMENT>
         </Tree>
        </program>
        "###);
        insta::assert_snapshot!(runx(r"(lifetime [3, 14] - [3, 16]
            (identifier [3, 15] - [3, 16]))"), @r###"
        <program>
         <Tree> (
          <id>lifetime</id>
          <Range> [
           <integer_number>3</integer_number> ,
           <integer_number>14</integer_number> ] - [
           <integer_number>3</integer_number> ,
           <integer_number>16</integer_number> ]
          </Range>
          <repeat_AttributeOrTree>
           <AttributeOrTree>
            <Tree><captured_pattern><identifier> ( identifier
               <Range> [
                <integer_number>3</integer_number> ,
                <integer_number>15</integer_number> ] - [
                <integer_number>3</integer_number> ,
                <integer_number>16</integer_number> ]
               </Range> )
              </identifier>
             </captured_pattern>
            </Tree>
           </AttributeOrTree>
          </repeat_AttributeOrTree> )
         </Tree>
        </program>
        "###);
    }
}
