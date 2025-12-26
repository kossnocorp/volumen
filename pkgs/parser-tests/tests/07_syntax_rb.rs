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
                        outer: Span(
                          start: 19,
                          end: 213,
                        ),
                        inner: Span(
                          start: 27,
                          end: 213,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 26,
                      ),
                      exp: "<<~TEXT\n  You are a helpful assistant.\n  You will answer the user\'s questions to the best of your ability.\n  If you don\'t know the answer, just say that you don\'t know, don\'t try to make it up.\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 17,
                          end: 82,
                        ),
                        inner: Span(
                          start: 25,
                          end: 82,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 24,
                      ),
                      exp: "<<~TEXT\n  Hello, #{name}!\n  How is the weather today in #{city}?\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 25,
                          end: 38,
                        ),
                        inner: Span(
                          start: 26,
                          end: 37,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 38,
                      ),
                      exp: "\'hello world\'",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 28,
                          end: 43,
                        ),
                        inner: Span(
                          start: 31,
                          end: 42,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 43,
                      ),
                      exp: "%q(hello world)",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 28,
                          end: 53,
                        ),
                        inner: Span(
                          start: 31,
                          end: 52,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 53,
                      ),
                      exp: "%q{no interpolation #{x}}",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 28,
                          end: 45,
                        ),
                        inner: Span(
                          start: 31,
                          end: 44,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 45,
                      ),
                      exp: "%Q(Hello #{name})",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 37,
                              end: 44,
                            ),
                            inner: Span(
                              start: 39,
                              end: 43,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 27,
                          end: 44,
                        ),
                        inner: Span(
                          start: 30,
                          end: 43,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 44,
                      ),
                      exp: "%Q|Pipes #{name}|",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 36,
                              end: 43,
                            ),
                            inner: Span(
                              start: 38,
                              end: 42,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 28,
                          end: 46,
                        ),
                        inner: Span(
                          start: 31,
                          end: 45,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 46,
                      ),
                      exp: "%Q<Angles #{name}>",
                      vars: [
                        PromptVar(
                          exp: "#{name}",
                          span: SpanShape(
                            outer: Span(
                              start: 38,
                              end: 45,
                            ),
                            inner: Span(
                              start: 40,
                              end: 44,
                            ),
                          ),
                        ),
                      ],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 20,
                          end: 40,
                        ),
                        inner: Span(
                          start: 26,
                          end: 40,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 25,
                      ),
                      exp: "<<EOF\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 29,
                          end: 50,
                        ),
                        inner: Span(
                          start: 36,
                          end: 50,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 35,
                      ),
                      exp: "<<~EOF\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 27,
                          end: 49,
                        ),
                        inner: Span(
                          start: 35,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 34,
                      ),
                      exp: "<<\'EOF\'\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
                        outer: Span(
                          start: 27,
                          end: 49,
                        ),
                        inner: Span(
                          start: 35,
                          end: 49,
                        ),
                      ),
                      enclosure: Span(
                        start: 0,
                        end: 34,
                      ),
                      exp: "<<\"EOF\"\nHello #{name}\n",
                      vars: [],
                      annotations: [
                        PromptAnnotation(
                          span: Span(
                            start: 0,
                            end: 9,
                          ),
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
