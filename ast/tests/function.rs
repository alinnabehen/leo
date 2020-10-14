use leo_ast::ast::{LanguageParser, Rule};

use pest::*;

#[test]
fn call_wo_args() {
    parses_to! {
        parser: LanguageParser,
        input:  "x()",
        rule:   Rule::expression_postfix,
        tokens: [
            expression_postfix(0, 3, [
                keyword_or_identifier(0, 1, [self_keyword_or_identifier(0, 1, [identifier(0, 1, [])])]),
                access(1, 3, [access_call(1, 3, [])])
            ])
        ]
    }
}

#[test]
fn call_with_arg() {
    parses_to! {
        parser: LanguageParser,
        input:  "x(true)",
        rule:   Rule::expression_postfix,
        tokens: [
            expression_postfix(0, 7, [
                keyword_or_identifier(0, 1, [self_keyword_or_identifier(0, 1, [identifier(0, 1, [])])]),
                access(1, 7, [access_call(1, 7, [
                    expression(2, 6, [expression_term(2, 6, [value(2, 6, [value_boolean(2, 6, [])])])])
                ])])
            ])
        ]
    }
}

#[test]
fn call_with_2_args() {
    parses_to! {
        parser: LanguageParser,
        input:  "x(true, false)",
        rule:   Rule::expression_postfix,
        tokens: [
            expression_postfix(0, 14, [
                keyword_or_identifier(0, 1, [self_keyword_or_identifier(0, 1, [identifier(0, 1, [])])]),
                access(1, 14, [access_call(1, 14, [
                    expression(2, 6, [expression_term(2, 6, [value(2, 6, [value_boolean(2, 6, [])])])]),
                    expression(8, 13, [expression_term(8, 13, [value(8, 13, [value_boolean(8, 13, [])])])])
                ])])
            ])
        ]
    }
}

#[test]
fn empty_def() {
    parses_to! {
        parser: LanguageParser,
        input:  "function x() {}",
        rule:   Rule::function,
        tokens: [
            function(0, 15, [identifier(9, 10, [])])
        ]
    }
}

#[test]
fn id_def() {
    parses_to! {
        parser: LanguageParser,
        input:  "function id(x: u8) -> u8 { return x }",
        rule:   Rule::function,
        tokens: [
            function(0, 37, [
                identifier(9, 11, []),
                input(12, 17, [
                    function_input(12, 17, [
                        identifier(12, 13, []),
                        type_(15, 17, [type_data(15, 17, [type_integer(15, 17, [type_integer_unsigned(15, 17, [type_u8(15, 17, [])])])])])
                    ])
                ]),
                type_(22, 24, [type_data(22, 24, [type_integer(22, 24, [type_integer_unsigned(22, 24, [type_u8(22, 24, [])])])])]),
                statement(27, 36, [statement_return(27, 36, [
                    expression(34, 36, [expression_term(34, 35, [identifier(34, 35, [])])])
                ])])
            ])
        ]
    }
}
