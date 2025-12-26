use serde::Serialize;
use volumen_parser_core::VolumenParser;
use volumen_parser_cs::ParserCs;
use volumen_parser_go::ParserGo;
use volumen_parser_java::ParserJava;
use volumen_parser_php::ParserPhp;
use volumen_parser_py::ParserPy as ParserPyRustPython;
use volumen_parser_py_ruff::ParserPy as ParserPyRuff;
use volumen_parser_py_tree_sitter::ParserPy as ParserPyTreeSitter;
use volumen_parser_rb::ParserRb;
use volumen_parser_ts::ParserTs as ParserTsOxc;
use volumen_parser_ts_tree_sitter::ParserTs as ParserTsTreeSitter;
use volumen_types::{ParseResult, Prompt};

type Parsers = [(&'static str, Parse)];

type Parse = fn(&str, &str) -> ParseResult;

static TS_PARSERS: &Parsers = &[
    ("ParserTsOxc", ParserTsOxc::parse),
    ("ParserTsTreeSitter", ParserTsTreeSitter::parse),
];

static PY_PARSERS: &Parsers = &[
    ("ParserPyRustPython", ParserPyRustPython::parse),
    ("ParserPyRuff", ParserPyRuff::parse),
    ("ParserPyTreeSitter", ParserPyTreeSitter::parse),
];

static RB_PARSERS: &Parsers = &[("ParserRb", ParserRb::parse)];

static PHP_PARSERS: &Parsers = &[("ParserPhp", ParserPhp::parse)];

static JAVA_PARSERS: &Parsers = &[("ParserJava", ParserJava::parse)];

static GO_PARSERS: &Parsers = &[("ParserGo", ParserGo::parse)];

static CSHARP_PARSERS: &Parsers = &[("ParserCs", ParserCs::parse)];

#[derive(Serialize)]
pub struct PromptSourceCuts {
    pub enclosure: &'static str,
    pub outer: &'static str,
    pub inner: &'static str,
    pub vars: Vec<PromptSourceCutVar>,
}

#[derive(Serialize)]
pub struct PromptSourceCutVar {
    pub outer: &'static str,
    pub inner: &'static str,
}

#[derive(Serialize)]
pub struct PromptAnnotationCuts {
    pub outer: &'static str,
    pub inner: &'static str,
}

impl PromptSourceCuts {
    pub fn cut(source: &'static str, prompt: &Prompt) -> Self {
        let enclosure = &source[prompt.enclosure.0 as usize..prompt.enclosure.1 as usize];
        let outer = &source[prompt.span.outer.0 as usize..prompt.span.outer.1 as usize];
        let inner = &source[prompt.span.inner.0 as usize..prompt.span.inner.1 as usize];
        let vars = prompt
            .vars
            .iter()
            .map(|var| PromptSourceCutVar {
                outer: &source[var.span.outer.0 as usize..var.span.outer.1 as usize],
                inner: &source[var.span.inner.0 as usize..var.span.inner.1 as usize],
            })
            .collect();

        PromptSourceCuts {
            enclosure,
            outer,
            inner,
            vars,
        }
    }
}

pub struct ParseTest {}

impl ParseTest {
    pub fn test(lang: &ParseTestLang, assertions: ParseAssertions) {
        let mut insta_settings = insta::Settings::new();
        insta_settings.add_redaction(".error", "<error>");

        insta_settings.bind(|| {
            let mut results = vec![];
            for (parser, parse) in lang.parsers() {
                let result = parse(lang.source, lang.filename);
                results.push(ParseTestResult { result, parser });
            }

            insta::allow_duplicates! {
                for result in &results {
                    let error_description = match &result.result {
                        ParseResult::ParseResultError(err) => format!("\nError: {}", err.error),
                        _ => "".to_owned(),
                    };

                    insta::with_settings!({ description => format!("Assertion: parser, parser name: {}{}", result.parser, error_description) }, {
                        (assertions.result)(&result.result);
                    });

                    match &result.result {
                        ParseResult::ParseResultSuccess(result_success) => {
                            let cuts: Vec<PromptSourceCuts> = result_success
                                .prompts
                                .iter()
                                .map(|prompt| PromptSourceCuts::cut(lang.source, prompt))
                                .collect();

                            insta::with_settings!({ description => format!("Assertion: cuts, parser name: {}", result.parser) }, {
                                (assertions.cuts)(cuts);
                            });

                            let interpolations : Vec<String> = result_success
                                .prompts
                                .iter()
                                .map(|prompt| {
                                    let interpolated_start = prompt.span.inner.0 - prompt.span.outer.0;
                                    let interpolated_end = prompt.span.inner.1 - prompt.span.outer.0;
                                    let mut interpolated = prompt.exp[interpolated_start as usize..interpolated_end as usize].to_owned();
                                    prompt.vars.iter().enumerate().rev().for_each(|(var_index, var)| {
                                        let var_start = (var.span.outer.0 - prompt.span.inner.0) as usize;
                                        let var_end = (var.span.outer.1 - prompt.span.inner.0) as usize;
                                        let range = var_start..var_end;
                                        interpolated.replace_range(range, &format!("{{{}}}", var_index));
                                    });
                                    interpolated

                                })
                                .collect();

                            insta::with_settings!({ description => format!("Assertion: interpolation, parser name: {}", result.parser) }, {
                                (assertions.interpolate)(interpolations);
                            });

                            let annotations : Vec<Vec<Vec<PromptAnnotationCuts>>> = result_success
                                .prompts
                                .iter()
                                .map(|prompt| {
                                    prompt
                                        .annotations
                                        .iter()
                                        .map(|annotation| {
                                            annotation.spans.iter().map(|span_shape| {
                                                PromptAnnotationCuts {
                                                    outer: &lang.source[span_shape.outer.0 as usize..span_shape.outer.1 as usize],
                                                    inner: &lang.source[span_shape.inner.0 as usize..span_shape.inner.1 as usize],
                                                }
                                            }).collect()
                                        })
                                        .collect()
                                })
                                .collect();


                            insta::with_settings!({ description => format!("Assertion: annotations, parser name: {}", result.parser) }, {
                                (assertions.annotations)(annotations);
                            });
                        }

                        ParseResult::ParseResultError(_) => {
                            (assertions.cuts)(vec![]);
                            (assertions.interpolate)(vec![]);
                            (assertions.annotations)(vec![]);
                        }
                    }

                }
            }
        });
    }
}

pub enum ParseLang {
    Ts,
    Py,
    Rb,
    Php,
    Java,
    Go,
    Cs,
}

impl ParseLang {
    pub fn parsers(&self) -> &Parsers {
        match self {
            ParseLang::Ts => TS_PARSERS,
            ParseLang::Py => PY_PARSERS,
            ParseLang::Rb => RB_PARSERS,
            ParseLang::Php => PHP_PARSERS,
            ParseLang::Java => JAVA_PARSERS,
            ParseLang::Go => GO_PARSERS,
            ParseLang::Cs => CSHARP_PARSERS,
        }
    }
}

pub struct ParseTestResult {
    pub result: ParseResult,
    pub parser: &'static str,
}

pub struct ParseTestLang {
    pub source: &'static str,
    pub lang: ParseLang,
    pub filename: &'static str,
}

impl ParseTestLang {
    pub fn ts(source: &'static str) -> ParseTestLang {
        Self::ts_named(source, "prompts.js")
    }

    pub fn ts_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Ts,
            filename,
        }
    }

    pub fn py(source: &'static str) -> ParseTestLang {
        Self::py_named(source, "prompts.py")
    }

    pub fn py_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Py,
            filename,
        }
    }

    pub fn rb(source: &'static str) -> ParseTestLang {
        Self::rb_named(source, "prompts.rb")
    }

    pub fn rb_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Rb,
            filename,
        }
    }

    pub fn php(source: &'static str) -> ParseTestLang {
        Self::php_named(source, "prompts.php")
    }

    pub fn php_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Php,
            filename,
        }
    }

    pub fn java(source: &'static str) -> ParseTestLang {
        Self::java_named(source, "Prompts.java")
    }

    pub fn java_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Java,
            filename,
        }
    }

    pub fn go(source: &'static str) -> ParseTestLang {
        Self::go_named(source, "prompts.go")
    }

    pub fn go_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Go,
            filename,
        }
    }

    pub fn cs(source: &'static str) -> ParseTestLang {
        Self::cs_named(source, "Prompts.cs")
    }

    pub fn cs_named(source: &'static str, filename: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Cs,
            filename,
        }
    }

    pub fn parsers(&self) -> &Parsers {
        self.lang.parsers()
    }
}

pub struct ParseAssertions {
    pub result: ParseSnapshotAssertion,
    pub cuts: ParseCutsAssertion,
    pub interpolate: ParseInterpolateAssertion,
    pub annotations: ParseAnnotationsAssertion,
}

type ParseSnapshotAssertion = Box<dyn Fn(&ParseResult) -> ()>;

type ParseCutsAssertion = Box<dyn Fn(Vec<PromptSourceCuts>) -> ()>;

type ParseInterpolateAssertion = Box<dyn Fn(Vec<String>) -> ()>;

type ParseAnnotationsAssertion = Box<dyn Fn(Vec<Vec<Vec<PromptAnnotationCuts>>>) -> ()>;
