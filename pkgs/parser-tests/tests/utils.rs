use serde::Serialize;
use volumen_parser_core::VolumenParser;
use volumen_parser_py::ParserPy as ParserPyRustPython;
use volumen_parser_py_ruff::ParserPy as ParserPyRuff;
use volumen_parser_py_tree_sitter::ParserPy as ParserPyTreeSitter;
use volumen_parser_ts::ParserTs as ParserTsOxc;
use volumen_parser_ts_tree_sitter::ParserTs as ParserTsTreeSitter;
use volumen_types::{ParseResult, ParseResultSuccess, Prompt};

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

impl PromptSourceCuts {
    pub fn cut(source: &'static str, prompt: &Prompt) -> Self {
        let enclosure = &source[prompt.enclosure.start as usize..prompt.enclosure.end as usize];
        let outer = &source[prompt.span.outer.start as usize..prompt.span.outer.end as usize];
        let inner = &source[prompt.span.inner.start as usize..prompt.span.inner.end as usize];
        let vars = prompt
            .vars
            .iter()
            .map(|var| PromptSourceCutVar {
                outer: &source[var.span.outer.start as usize..var.span.outer.end as usize],
                inner: &source[var.span.inner.start as usize..var.span.inner.end as usize],
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
    pub fn test(name: Option<&str>, lang: &ParseTestLang, assertions: ParseAssertions) {
        let test_name = name.unwrap_or("default");

        let mut results = vec![];
        for (parser, parse) in lang.parsers() {
            let result = parse(lang.source, lang.filename());
            match result {
                ParseResult::ParseResultSuccess(success) => {
                    results.push(ParseTestResult {
                        result: success,
                        parser,
                    });
                }

                _ => panic!("Expected ParseResultSuccess"),
            }
        }

        insta::allow_duplicates! {
            for result in &results {
              insta::with_settings!({ description => format!("Test: {}, assertion: parser, parser name: {}", test_name, result.parser) }, {
                  (assertions.result)(&result.result);
              });
            }
        }

        insta::allow_duplicates! {
            for result in &results {
                let cuts: Vec<PromptSourceCuts> = result
                    .result
                    .prompts
                    .iter()
                    .map(|prompt| PromptSourceCuts::cut(lang.source, prompt))
                    .collect();

                insta::with_settings!({ description => format!("Test: {}, assertion: cuts, parser name: {}", test_name,result.parser) }, {
                    (assertions.cuts)(cuts);
                });

                if let Some(assertion) = &assertions.interpolate {
                    let interpolations : Vec<String> = result
                        .result
                        .prompts
                        .iter()
                        .map(|prompt| {
                            let interpolated_start = prompt.span.inner.start - prompt.span.outer.start;
                            let interpolated_end = prompt.span.inner.end - prompt.span.outer.start;
                            let mut interpolated = prompt.exp[interpolated_start as usize..interpolated_end as usize].to_owned();
                            prompt.vars.iter().enumerate().rev().for_each(|(var_index, var)| {
                                let var_start = (var.span.outer.start - prompt.span.inner.start) as usize;
                                let var_end = (var.span.outer.end - prompt.span.inner.start) as usize;
                                let range = var_start..var_end;
                                interpolated.replace_range(range, &format!("{{{}}}", var_index));
                            });
                            interpolated

                        })
                        .collect();

                    insta::with_settings!({ description => format!("Test: {}, assertion: interpolation, parser name: {}", test_name,result.parser) }, {
                        (assertion)(interpolations);
                    });
                }
            }
        }

        // let max_prompts_len = ParseTestResult::max_prompts_len(&results);

        // // insta::allow_duplicates! {
        // // }

        // for index in 0..max_prompts_len {
        //     let prompts: Vec<(&Prompt, &str)> = results.iter().map(|r| {
        //         (
        //             r.result.prompts.get(index).expect("Prompt is missing"),
        //             r.parser,
        //         )
        //     });
        //     // .collect();

        //     insta::allow_duplicates! {
        //         for (prompt, parser) in &prompts {
        //             let cuts = PromptSourceCuts::cut(lang.source, prompt);

        //             insta::with_settings!({ description => format!("Test: {}, assertion: cuts, parser name: {}, prompt index: {}", test_name,parser, index) }, {
        //                 (assertions.cuts)(&cuts);
        //             });
        //         }
        //     }

        //     if let Some(assertion) = &assertions.interpolate {
        //         insta::allow_duplicates! {
        //             for (prompt, parser) in &prompts {
        //                 let interpolated_start = prompt.span.inner.start - prompt.span.outer.start;
        //                 let interpolated_end = prompt.span.inner.end - prompt.span.outer.start;
        //                 let mut interpolated = prompt.exp[interpolated_start as usize..interpolated_end as usize].to_owned();
        //                 prompt.vars.iter().enumerate().rev().for_each(|(var_index, var)| {
        //                     let var_start = (var.span.outer.start - prompt.span.inner.start) as usize;
        //                     let var_end = (var.span.outer.end - prompt.span.inner.start) as usize;
        //                     let range = var_start..var_end;
        //                     interpolated.replace_range(range, &format!("{{{}}}", var_index));
        //                 });

        //                 insta::with_settings!({ description => format!("Test: {}, assertion: interpolation, parser name: {}, prompt index: {}", test_name, parser,index) }, {
        //                     (assertion)(interpolated);
        //                 });
        //             }
        //         }
        //     }
        // }
    }
}

pub enum ParseLang {
    Ts,
    Py,
}

impl ParseLang {
    pub fn parsers(&self) -> &Parsers {
        match self {
            ParseLang::Ts => TS_PARSERS,
            ParseLang::Py => PY_PARSERS,
        }
    }
}

pub struct ParseTestResult {
    pub result: ParseResultSuccess,
    pub parser: &'static str,
}

impl ParseTestResult {
    pub fn max_prompts_len(results: &Vec<ParseTestResult>) -> usize {
        results
            .iter()
            .map(|r| r.result.prompts.len())
            .max()
            .unwrap_or(0)
    }
}

pub struct ParseTestLang {
    pub source: &'static str,
    pub lang: ParseLang,
}

impl ParseTestLang {
    pub fn ts(source: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Ts,
        }
    }

    pub fn py(source: &'static str) -> ParseTestLang {
        ParseTestLang {
            source,
            lang: ParseLang::Py,
        }
    }

    pub fn filename(&self) -> &str {
        match self.lang {
            ParseLang::Ts => "prompts.js",
            ParseLang::Py => "prompts.py",
        }
    }

    pub fn parsers(&self) -> &Parsers {
        self.lang.parsers()
    }
}

pub struct ParseAssertions {
    pub result: ParseSnapshotAssertion,
    pub cuts: ParseCutsAssertion,
    pub interpolate: Option<ParseInterpolateAssertion>,
}

type ParseSnapshotAssertion = Box<dyn Fn(&ParseResultSuccess) -> ()>;

type ParseCutsAssertion = Box<dyn Fn(Vec<PromptSourceCuts>) -> ()>;

type ParseInterpolateAssertion = Box<dyn Fn(Vec<String>) -> ()>;
