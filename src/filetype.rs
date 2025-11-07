pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".js") || file_name.ends_with(".jsx") {
            return Self {
                name: String::from("JavaScript"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "await".to_string(), "break".to_string(), "case".to_string(),
                        "catch".to_string(), "class".to_string(), "const".to_string(),
                        "continue".to_string(), "default".to_string(), "delete".to_string(),
                        "do".to_string(), "else".to_string(), "export".to_string(),
                        "extends".to_string(), "finally".to_string(), "for".to_string(),
                        "function".to_string(), "if".to_string(), "import".to_string(),
                        "in".to_string(), "instanceof".to_string(), "let".to_string(),
                        "new".to_string(), "return".to_string(), "static".to_string(),
                        "super".to_string(), "switch".to_string(), "this".to_string(),
                        "throw".to_string(), "try".to_string(), "typeof".to_string(),
                        "var".to_string(), "void".to_string(), "while".to_string(),
                        "with".to_string(), "yield".to_string(), "async".to_string(),
                    ],
                    secondary_keywords: vec![
                        "true".to_string(), "false".to_string(), "null".to_string(),
                        "undefined".to_string(), "NaN".to_string(), "Infinity".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".ts") || file_name.ends_with(".tsx") {
            return Self {
                name: String::from("TypeScript"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "await".to_string(), "break".to_string(), "case".to_string(),
                        "catch".to_string(), "class".to_string(), "const".to_string(),
                        "continue".to_string(), "default".to_string(), "delete".to_string(),
                        "do".to_string(), "else".to_string(), "export".to_string(),
                        "extends".to_string(), "finally".to_string(), "for".to_string(),
                        "function".to_string(), "if".to_string(), "import".to_string(),
                        "in".to_string(), "instanceof".to_string(), "let".to_string(),
                        "new".to_string(), "return".to_string(), "static".to_string(),
                        "super".to_string(), "switch".to_string(), "this".to_string(),
                        "throw".to_string(), "try".to_string(), "typeof".to_string(),
                        "var".to_string(), "void".to_string(), "while".to_string(),
                        "with".to_string(), "yield".to_string(), "async".to_string(),
                        "interface".to_string(), "type".to_string(), "namespace".to_string(),
                        "enum".to_string(), "private".to_string(), "public".to_string(),
                        "protected".to_string(), "readonly".to_string(),
                    ],
                    secondary_keywords: vec![
                        "string".to_string(), "number".to_string(), "boolean".to_string(),
                        "any".to_string(), "void".to_string(), "never".to_string(),
                        "true".to_string(), "false".to_string(), "null".to_string(),
                        "undefined".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".py") {
            return Self {
                name: String::from("Python"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: false,
                    primary_keywords: vec![
                        "and".to_string(), "as".to_string(), "assert".to_string(),
                        "async".to_string(), "await".to_string(), "break".to_string(),
                        "class".to_string(), "continue".to_string(), "def".to_string(),
                        "del".to_string(), "elif".to_string(), "else".to_string(),
                        "except".to_string(), "finally".to_string(), "for".to_string(),
                        "from".to_string(), "global".to_string(), "if".to_string(),
                        "import".to_string(), "in".to_string(), "is".to_string(),
                        "lambda".to_string(), "nonlocal".to_string(), "not".to_string(),
                        "or".to_string(), "pass".to_string(), "raise".to_string(),
                        "return".to_string(), "try".to_string(), "while".to_string(),
                        "with".to_string(), "yield".to_string(),
                    ],
                    secondary_keywords: vec![
                        "True".to_string(), "False".to_string(), "None".to_string(),
                        "self".to_string(), "cls".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".c") || file_name.ends_with(".h") {
            return Self {
                name: String::from("C"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "auto".to_string(), "break".to_string(), "case".to_string(),
                        "const".to_string(), "continue".to_string(), "default".to_string(),
                        "do".to_string(), "else".to_string(), "enum".to_string(),
                        "extern".to_string(), "for".to_string(), "goto".to_string(),
                        "if".to_string(), "register".to_string(), "return".to_string(),
                        "sizeof".to_string(), "static".to_string(), "struct".to_string(),
                        "switch".to_string(), "typedef".to_string(), "union".to_string(),
                        "volatile".to_string(), "while".to_string(),
                    ],
                    secondary_keywords: vec![
                        "char".to_string(), "double".to_string(), "float".to_string(),
                        "int".to_string(), "long".to_string(), "short".to_string(),
                        "signed".to_string(), "unsigned".to_string(), "void".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".cpp") || file_name.ends_with(".cc") || 
           file_name.ends_with(".cxx") || file_name.ends_with(".hpp") {
            return Self {
                name: String::from("C++"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "alignas".to_string(), "alignof".to_string(), "and".to_string(),
                        "asm".to_string(), "auto".to_string(), "break".to_string(),
                        "case".to_string(), "catch".to_string(), "class".to_string(),
                        "const".to_string(), "constexpr".to_string(), "continue".to_string(),
                        "decltype".to_string(), "default".to_string(), "delete".to_string(),
                        "do".to_string(), "else".to_string(), "enum".to_string(),
                        "explicit".to_string(), "extern".to_string(), "for".to_string(),
                        "friend".to_string(), "goto".to_string(), "if".to_string(),
                        "inline".to_string(), "namespace".to_string(), "new".to_string(),
                        "noexcept".to_string(), "nullptr".to_string(), "operator".to_string(),
                        "private".to_string(), "protected".to_string(), "public".to_string(),
                        "return".to_string(), "sizeof".to_string(), "static".to_string(),
                        "struct".to_string(), "switch".to_string(), "template".to_string(),
                        "this".to_string(), "throw".to_string(), "try".to_string(),
                        "typedef".to_string(), "typeid".to_string(), "typename".to_string(),
                        "union".to_string(), "using".to_string(), "virtual".to_string(),
                        "volatile".to_string(), "while".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(), "char".to_string(), "double".to_string(),
                        "float".to_string(), "int".to_string(), "long".to_string(),
                        "short".to_string(), "signed".to_string(), "unsigned".to_string(),
                        "void".to_string(), "wchar_t".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".go") {
            return Self {
                name: String::from("Go"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "break".to_string(), "case".to_string(), "chan".to_string(),
                        "const".to_string(), "continue".to_string(), "default".to_string(),
                        "defer".to_string(), "else".to_string(), "fallthrough".to_string(),
                        "for".to_string(), "func".to_string(), "go".to_string(),
                        "goto".to_string(), "if".to_string(), "import".to_string(),
                        "interface".to_string(), "map".to_string(), "package".to_string(),
                        "range".to_string(), "return".to_string(), "select".to_string(),
                        "struct".to_string(), "switch".to_string(), "type".to_string(),
                        "var".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(), "byte".to_string(), "complex64".to_string(),
                        "complex128".to_string(), "error".to_string(), "float32".to_string(),
                        "float64".to_string(), "int".to_string(), "int8".to_string(),
                        "int16".to_string(), "int32".to_string(), "int64".to_string(),
                        "rune".to_string(), "string".to_string(), "uint".to_string(),
                        "uint8".to_string(), "uint16".to_string(), "uint32".to_string(),
                        "uint64".to_string(), "uintptr".to_string(),
                    ],
                },
            };
        }
        
        if file_name.ends_with(".java") {
            return Self {
                name: String::from("Java"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "abstract".to_string(), "assert".to_string(), "break".to_string(),
                        "case".to_string(), "catch".to_string(), "class".to_string(),
                        "const".to_string(), "continue".to_string(), "default".to_string(),
                        "do".to_string(), "else".to_string(), "enum".to_string(),
                        "extends".to_string(), "final".to_string(), "finally".to_string(),
                        "for".to_string(), "goto".to_string(), "if".to_string(),
                        "implements".to_string(), "import".to_string(), "instanceof".to_string(),
                        "interface".to_string(), "native".to_string(), "new".to_string(),
                        "package".to_string(), "private".to_string(), "protected".to_string(),
                        "public".to_string(), "return".to_string(), "static".to_string(),
                        "strictfp".to_string(), "super".to_string(), "switch".to_string(),
                        "synchronized".to_string(), "this".to_string(), "throw".to_string(),
                        "throws".to_string(), "transient".to_string(), "try".to_string(),
                        "volatile".to_string(), "while".to_string(),
                    ],
                    secondary_keywords: vec![
                        "boolean".to_string(), "byte".to_string(), "char".to_string(),
                        "double".to_string(), "float".to_string(), "int".to_string(),
                        "long".to_string(), "short".to_string(), "void".to_string(),
                        "true".to_string(), "false".to_string(), "null".to_string(),
                    ],
                },
            };
        }
        
        Self::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }
    pub fn strings(&self) -> bool {
        self.strings
    }
    pub fn characters(&self) -> bool {
        self.characters
    }
    pub fn comments(&self) -> bool {
        self.comments
    }
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }
    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
