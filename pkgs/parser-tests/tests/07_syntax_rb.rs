use indoc::indoc;
use insta::{assert_json_snapshot, assert_ron_snapshot};

mod utils;
use utils::*;

#[test]
fn invalid() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            x = "unclosed
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r#"
                ParseResultError(
                  state: "error",
                  error: "<error>",
                )
                "#);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @"[]");
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @"[]");
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @"[]");
            }),
        },
    );
}

#[test]
fn heredoc() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            system = <<~TEXT
              You are a helpful assistant.
              You will answer the user's questions to the best of your ability.
              If you don't know the answer, just say that you don't know, don't try to make it up.
            TEXT
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (19, 213),
                        inner: (27, 213),
                      ),
                      enclosure: (0, 26),
                      exp: "<<~TEXT\n  You are a helpful assistant.\n  You will answer the user\'s questions to the best of your ability.\n  If you don\'t know the answer, just say that you don\'t know, don\'t try to make it up.\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nsystem = <<~TEXT",
                    "outer": "<<~TEXT\n  You are a helpful assistant.\n  You will answer the user's questions to the best of your ability.\n  If you don't know the answer, just say that you don't know, don't try to make it up.\n",
                    "inner": "  You are a helpful assistant.\n  You will answer the user's questions to the best of your ability.\n  If you don't know the answer, just say that you don't know, don't try to make it up.\n",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "  You are a helpful assistant.\n  You will answer the user's questions to the best of your ability.\n  If you don't know the answer, just say that you don't know, don't try to make it up.\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn heredoc_interpolated() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            user = <<~TEXT
              Hello, #{name}!
              How is the weather today in #{city}?
            TEXT
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (17, 82),
                        inner: (25, 82),
                      ),
                      enclosure: (0, 24),
                      exp: "<<~TEXT\n  Hello, #{name}!\n  How is the weather today in #{city}?\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nuser = <<~TEXT",
                    "outer": "<<~TEXT\n  Hello, #{name}!\n  How is the weather today in #{city}?\n",
                    "inner": "  Hello, #{name}!\n  How is the weather today in #{city}?\n",
                    "vars": []
                  }
                ]
                "##);
            }),

            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "  Hello, #{name}!\n  How is the weather today in #{city}?\n"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn single_quote() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            single_quote = 'hello world'
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (25, 38),
                        inner: (26, 37),
                      ),
                      enclosure: (0, 38),
                      exp: "\'hello world\'",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nsingle_quote = 'hello world'",
                    "outer": "'hello world'",
                    "inner": "hello world",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "hello world"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn percent_q_paren() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            percent_q_paren = %q(hello world)
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (28, 43),
                        inner: (31, 42),
                      ),
                      enclosure: (0, 43),
                      exp: "%q(hello world)",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\npercent_q_paren = %q(hello world)",
                    "outer": "%q(hello world)",
                    "inner": "hello world",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "hello world"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn percent_q_brace() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            percent_q_brace = %q{no interpolation #{x}}
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (28, 53),
                        inner: (31, 52),
                      ),
                      enclosure: (0, 53),
                      exp: "%q{no interpolation #{x}}",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\npercent_q_brace = %q{no interpolation #{x}}",
                    "outer": "%q{no interpolation #{x}}",
                    "inner": "no interpolation #{x}",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "no interpolation #{x}"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn percent_q_upper() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            percent_q_upper = %Q(Hello #{name})
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (28, 45),
                        inner: (31, 44),
                      ),
                      enclosure: (0, 45),
                      exp: "%Q(Hello #{name})",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: (37, 44),
                            inner: (39, 43),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),

            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\npercent_q_upper = %Q(Hello #{name})",
                    "outer": "%Q(Hello #{name})",
                    "inner": "Hello #{name}",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello {0}"
                ]
                "#);
            }),

            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                 [
                   [
                     "# @prompt"
                   ]
                 ]
                 "##);
            }),
        },
    );
}

#[test]
fn percent_q_pipe() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            percent_q_pipe = %Q|Pipes #{name}|
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (27, 44),
                        inner: (30, 43),
                      ),
                      enclosure: (0, 44),
                      exp: "%Q|Pipes #{name}|",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: (36, 43),
                            inner: (38, 42),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\npercent_q_pipe = %Q|Pipes #{name}|",
                    "outer": "%Q|Pipes #{name}|",
                    "inner": "Pipes #{name}",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Pipes {0}"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn percent_q_angle() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            percent_q_angle = %Q<Angles #{name}>
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (28, 46),
                        inner: (31, 45),
                      ),
                      enclosure: (0, 46),
                      exp: "%Q<Angles #{name}>",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: (38, 45),
                            inner: (40, 44),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\npercent_q_angle = %Q<Angles #{name}>",
                    "outer": "%Q<Angles #{name}>",
                    "inner": "Angles #{name}",
                    "vars": [
                      {
                        "outer": "#{name}",
                        "inner": "name"
                      }
                    ]
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Angles {0}"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn heredoc_plain() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            heredoc = <<EOF
            Hello #{name}
            EOF
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (20, 40),
                        inner: (26, 40),
                      ),
                      enclosure: (0, 25),
                      exp: "<<EOF\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nheredoc = <<EOF",
                    "outer": "<<EOF\nHello #{name}\n",
                    "inner": "Hello #{name}\n",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello #{name}\n"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn heredoc_squiggly() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            squiggly_heredoc = <<~EOF
            Hello #{name}
            EOF
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (29, 50),
                        inner: (36, 50),
                      ),
                      enclosure: (0, 35),
                      exp: "<<~EOF\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nsquiggly_heredoc = <<~EOF",
                    "outer": "<<~EOF\nHello #{name}\n",
                    "inner": "Hello #{name}\n",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello #{name}\n"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn heredoc_single() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            single_heredoc = <<'EOF'
            Hello #{name}
            EOF
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (27, 49),
                        inner: (35, 49),
                      ),
                      enclosure: (0, 34),
                      exp: "<<\'EOF\'\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\nsingle_heredoc = <<'EOF'",
                    "outer": "<<'EOF'\nHello #{name}\n",
                    "inner": "Hello #{name}\n",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello #{name}\n"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}

#[test]
fn heredoc_double() {
    ParseTest::test(
        &ParseTestLang::rb(indoc! {r#"
            # @prompt
            double_heredoc = <<"EOF"
            Hello #{name}
            EOF
        "#}),
        ParseAssertions {
            result: Box::new(|result| {
                assert_ron_snapshot!(result, @r##"
                ParseResultSuccess(
                  state: "success",
                  prompts: [
                    Prompt(
                      file: "prompts.rb",
                      span: SpanShape(
                        outer: (27, 49),
                        inner: (35, 49),
                      ),
                      enclosure: (0, 34),
                      exp: "<<\"EOF\"\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: (0, 9),
                          exp: "# @prompt",
                        ),
                      ],
                    ),
                  ],
                )
                "##);
            }),
            cuts: Box::new(|prompt_source_cuts| {
                assert_json_snapshot!(prompt_source_cuts, @r##"
                [
                  {
                    "enclosure": "# @prompt\ndouble_heredoc = <<\"EOF\"",
                    "outer": "<<\"EOF\"\nHello #{name}\n",
                    "inner": "Hello #{name}\n",
                    "vars": []
                  }
                ]
                "##);
            }),
            interpolate: Box::new(|interpolations| {
                assert_json_snapshot!(interpolations, @r#"
                [
                  "Hello #{name}\n"
                ]
                "#);
            }),
            annotations: Box::new(|annotations| {
                assert_json_snapshot!(annotations, @r##"
                [
                  [
                    "# @prompt"
                  ]
                ]
                "##);
            }),
        },
    );
}
