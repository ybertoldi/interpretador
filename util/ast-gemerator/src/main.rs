use std::collections::HashSet;

static AST_VALS: &[&str] = &[
    "Binary: Expr left, Token operator, Expr right",
    "Grouping: Expr expression",
    "Literal: Object value",
    "Unary: Token operator, Expr right",
];

#[derive(Debug, Clone)]
struct Field {
    fd_name: String,
    fd_type: String,
}
#[derive(Debug, Clone)]
struct Class {
    name: String,
    fields: Vec<Field>,
}
fn main() {
    let (classes, _class_names) = parse_classes();

    generate_structs(&classes);
    generate_enum(&classes);
    generate_traits(&classes);
    generate_new(&classes);
}

fn parse_classes() -> (Vec<Class>, HashSet<String>) {
    let mut classes = Vec::new();
    let mut class_names = HashSet::new();
    for v in AST_VALS {
        let split: Vec<_> = v.split(":").map(|s| s.trim()).collect();
        let [cls_nm, fields] = split.as_slice() else {
            panic!("could not parse classname and fields!");
        };

        let parsed_fields = fields
            .split(",")
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .map(|splt| {
                let [fd_type, fd_name] = splt.as_slice() else {
                    panic!("could not parse field {split:?}")
                };
                Field {
                    fd_name: fd_name.to_string(),
                    fd_type: fd_type.to_string(),
                }
            })
            .collect();

        classes.push(Class {
            name: cls_nm.to_string(),
            fields: parsed_fields,
        });
        class_names.insert(cls_nm.to_string());
    }

    (classes, class_names)
}

fn generate_structs(classes: &Vec<Class>) {
    // struct definitions
    for c in classes {
        println!("pub struct {} {{", c.name);
        for f in &c.fields {
            if f.fd_type == "Expr" {
                println!("    {}: Box<{}>,", f.fd_name, f.fd_type);
            } else {
                println!("    {}: {},", f.fd_name, f.fd_type);
            }
        }
        println!("}}\n");
    }
}

fn generate_enum(classes: &Vec<Class>) {
    println!("pub enum Expr{{");
    for c in classes {
        println!("    {}({}),", c.name, c.name);
    }
    println!("}}\n");
}
fn generate_traits(classes: &[Class]) {
    println!("pub trait Visitor<R> {{");
    for c in classes {
        println!(
            "    fn visit_{}(&mut self, expr: &{}) -> R;",
            c.name.to_lowercase(),
            c.name
        );
    }
    println!("}}\n");

    println!("impl Expr {{");
    println!("    pub fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {{");
    println!("        match self{{");
    for c in classes {
        println!(
            "            Expr::{}(e) => visitor.visit_{}(e),",
            c.name,
            c.name.to_lowercase()
        );
    }
    println!("        }}");
    println!("    }}");
    println!("}}\n");
}

fn generate_new(classes: &Vec<Class>) {
    for c in classes {
        println!("impl {} {{", c.name);

        // function head
        print!("    pub fn new(");
        for f in &c.fields {
            print!("{}: {},", f.fd_name, f.fd_type);
        }
        println!(") -> Self {{");

        // function body
        println!("        Self{{");
        for f in &c.fields {
            if f.fd_type == "Expr" {
                println!("            {}: Box::new({}),", f.fd_name, f.fd_name);
            } else {
                println!("            {},", f.fd_name);
            }
        }
        println!("        }}");
        println!("    }}");
        println!("}}\n");
    }
}
