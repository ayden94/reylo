// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
//
// Changed in Reylo's copy (engine/mysql/vendor/sqlparser/README.md).

#[cfg(not(feature = "std"))]
use crate::alloc::string::ToString;
use crate::ast::{
    ColumnOption,
    ColumnPolicy, ColumnPolicyProperty, DollarQuotedString, Ident, IdentityParameters, IdentityProperty,
    IdentityPropertyFormatKind, IdentityPropertyKind, IdentityPropertyOrder, ObjectName, ObjectNamePart, TagsColumnOption,
};
use crate::dialect::{Dialect, Precedence};
use crate::keywords::Keyword;
use crate::parser::{Parser, ParserError};
use crate::tokenizer::Token;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::{format, vec};

use super::keywords::RESERVED_FOR_IDENTIFIER;

const RESERVED_KEYWORDS_FOR_SELECT_ITEM_OPERATOR: [Keyword; 1] = [Keyword::CONNECT_BY_ROOT];

// See: <https://docs.snowflake.com/en/sql-reference/reserved-keywords>
const RESERVED_KEYWORDS_FOR_TABLE_FACTOR: &[Keyword] = &[
    Keyword::ALL,
    Keyword::ALTER,
    Keyword::AND,
    Keyword::ANY,
    Keyword::AS,
    Keyword::BETWEEN,
    Keyword::BY,
    Keyword::CHECK,
    Keyword::COLUMN,
    Keyword::CONNECT,
    Keyword::CREATE,
    Keyword::CROSS,
    Keyword::CURRENT,
    Keyword::DELETE,
    Keyword::DISTINCT,
    Keyword::DROP,
    Keyword::ELSE,
    Keyword::EXISTS,
    Keyword::FOLLOWING,
    Keyword::FOR,
    Keyword::FROM,
    Keyword::FULL,
    Keyword::GRANT,
    Keyword::GROUP,
    Keyword::HAVING,
    Keyword::ILIKE,
    Keyword::IN,
    Keyword::INCREMENT,
    Keyword::INNER,
    Keyword::INSERT,
    Keyword::INTERSECT,
    Keyword::INTO,
    Keyword::IS,
    Keyword::JOIN,
    Keyword::LEFT,
    Keyword::LIKE,
    Keyword::MINUS,
    Keyword::NATURAL,
    Keyword::NOT,
    Keyword::NULL,
    Keyword::OF,
    Keyword::ON,
    Keyword::OR,
    Keyword::ORDER,
    Keyword::QUALIFY,
    Keyword::REGEXP,
    Keyword::REVOKE,
    Keyword::RIGHT,
    Keyword::RLIKE,
    Keyword::ROW,
    Keyword::ROWS,
    Keyword::SAMPLE,
    Keyword::SELECT,
    Keyword::SET,
    Keyword::SOME,
    Keyword::START,
    Keyword::TABLE,
    Keyword::TABLESAMPLE,
    Keyword::THEN,
    Keyword::TO,
    Keyword::TRIGGER,
    Keyword::UNION,
    Keyword::UNIQUE,
    Keyword::UPDATE,
    Keyword::USING,
    Keyword::VALUES,
    Keyword::WHEN,
    Keyword::WHENEVER,
    Keyword::WHERE,
    Keyword::WINDOW,
    Keyword::WITH,
];

/// A [`Dialect`] for [Snowflake](https://www.snowflake.com/)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnowflakeDialect;

impl Dialect for SnowflakeDialect {
    // see https://docs.snowflake.com/en/sql-reference/identifiers-syntax.html
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
    }

    /// See <https://docs.snowflake.com/en/sql-reference/identifiers-syntax>
    fn identifier_quote_style(&self, _identifier: &str) -> Option<char> {
        Some('"')
    }

    fn supports_projection_trailing_commas(&self) -> bool {
        true
    }

    fn supports_from_trailing_commas(&self) -> bool {
        true
    }

    // Snowflake supports double-dot notation when the schema name is not specified
    // In this case the default PUBLIC schema is used
    //
    // see https://docs.snowflake.com/en/sql-reference/name-resolution#resolution-when-schema-omitted-double-dot-notation
    fn supports_object_name_double_dot_notation(&self) -> bool {
        true
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_lowercase()
            || ch.is_ascii_uppercase()
            || ch.is_ascii_digit()
            || ch == '$'
            || ch == '_'
    }

    // See https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#escape_sequences
    fn supports_string_literal_backslash_escape(&self) -> bool {
        true
    }

    fn supports_within_after_array_aggregation(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/sql-reference/constructs/where#joins-in-the-where-clause>
    fn supports_outer_join_operator(&self) -> bool {
        true
    }

    fn supports_connect_by(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/sql-reference/sql/execute-immediate>
    fn supports_execute_immediate(&self) -> bool {
        true
    }

    fn supports_match_recognize(&self) -> bool {
        true
    }

    // Snowflake uses this syntax for "object constants" (the values of which
    // are not actually required to be constants).
    //
    // https://docs.snowflake.com/en/sql-reference/data-types-semistructured#label-object-constant
    fn supports_dictionary_syntax(&self) -> bool {
        true
    }

    // Snowflake doesn't document this but `FIRST_VALUE(arg, { IGNORE | RESPECT } NULLS)`
    // works (i.e. inside the argument list instead of after).
    fn supports_window_function_null_treatment_arg(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/sql/set#syntax)
    fn supports_parenthesized_set_variables(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/sql/comment)
    fn supports_comment_on(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/functions/extract)
    fn supports_extract_comma_syntax(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/functions/flatten)
    fn supports_subquery_as_function_arg(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/sql/create-view#optional-parameters)
    fn supports_create_view_comment_syntax(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/data-types-semistructured#array)
    fn supports_array_typedef_without_element_type(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/constructs/from)
    fn supports_parens_around_table_factor(&self) -> bool {
        true
    }

    /// See [doc](https://docs.snowflake.com/en/sql-reference/constructs/values)
    fn supports_values_as_table_factor(&self) -> bool {
        true
    }



    fn parse_column_option(
        &self,
        parser: &mut Parser,
    ) -> Result<Option<Result<Option<ColumnOption>, ParserError>>, ParserError> {
        parser.maybe_parse(|parser| {
            let with = parser.parse_keyword(Keyword::WITH);

            if parser.parse_keyword(Keyword::IDENTITY) {
                Ok(parse_identity_property(parser)
                    .map(|p| Some(ColumnOption::Identity(IdentityPropertyKind::Identity(p)))))
            } else if parser.parse_keyword(Keyword::AUTOINCREMENT) {
                Ok(parse_identity_property(parser).map(|p| {
                    Some(ColumnOption::Identity(IdentityPropertyKind::Autoincrement(
                        p,
                    )))
                }))
            } else if parser.parse_keywords(&[Keyword::MASKING, Keyword::POLICY]) {
                Ok(parse_column_policy_property(parser, with)
                    .map(|p| Some(ColumnOption::Policy(ColumnPolicy::MaskingPolicy(p)))))
            } else if parser.parse_keywords(&[Keyword::PROJECTION, Keyword::POLICY]) {
                Ok(parse_column_policy_property(parser, with)
                    .map(|p| Some(ColumnOption::Policy(ColumnPolicy::ProjectionPolicy(p)))))
            } else if parser.parse_keywords(&[Keyword::TAG]) {
                Ok(parse_column_tags(parser, with).map(|p| Some(ColumnOption::Tags(p))))
            } else {
                Err(ParserError::ParserError("not found match".to_string()))
            }
        })
    }

    fn get_next_precedence(&self, parser: &Parser) -> Option<Result<u8, ParserError>> {
        let token = parser.peek_token_ref();
        // Snowflake supports the `:` cast operator unlike other dialects
        match &token.token {
            Token::Colon => Some(Ok(self.prec_value(Precedence::DoubleColon))),
            _ => None,
        }
    }

    fn describe_requires_table_keyword(&self) -> bool {
        true
    }

    fn allow_extract_custom(&self) -> bool {
        true
    }

    fn allow_extract_single_quotes(&self) -> bool {
        true
    }

    /// Snowflake expects the `LIKE` option before the `IN` option,
    /// for example: <https://docs.snowflake.com/en/sql-reference/sql/show-views#syntax>
    fn supports_show_like_before_in(&self) -> bool {
        true
    }

    fn supports_left_associative_joins_without_parens(&self) -> bool {
        false
    }

    fn is_reserved_for_identifier(&self, kw: Keyword) -> bool {
        // Unreserve some keywords that Snowflake accepts as identifiers
        // See: https://docs.snowflake.com/en/sql-reference/reserved-keywords
        if matches!(kw, Keyword::INTERVAL) {
            false
        } else {
            RESERVED_FOR_IDENTIFIER.contains(&kw)
        }
    }

    fn supports_partiql(&self) -> bool {
        true
    }

    fn is_column_alias(&self, kw: &Keyword, parser: &mut Parser) -> bool {
        match kw {
            // The following keywords can be considered an alias as long as
            // they are not followed by other tokens that may change their meaning
            // e.g. `SELECT * EXCEPT (col1) FROM tbl`
            Keyword::EXCEPT
            // e.g. `INSERT INTO t SELECT 1 RETURNING *`
            | Keyword::RETURNING if !matches!(parser.peek_token_ref().token, Token::Comma | Token::EOF) =>
            {
                false
            }

            // e.g. `SELECT 1 LIMIT 5` - not an alias
            // e.g. `SELECT 1 OFFSET 5 ROWS` - not an alias
            Keyword::LIMIT | Keyword::OFFSET if peek_for_limit_options(parser) => false,

            // `FETCH` can be considered an alias as long as it's not followed by `FIRST`` or `NEXT`
            // which would give it a different meanings, for example:
            // `SELECT 1 FETCH FIRST 10 ROWS` - not an alias
            // `SELECT 1 FETCH 10` - not an alias
            Keyword::FETCH if parser.peek_one_of_keywords(&[Keyword::FIRST, Keyword::NEXT]).is_some()
                    || peek_for_limit_options(parser) =>
            {
                false
            }

            // Reserved keywords by the Snowflake dialect, which seem to be less strictive
            // than what is listed in `keywords::RESERVED_FOR_COLUMN_ALIAS`. The following
            // keywords were tested with the this statement: `SELECT 1 <KW>`.
            Keyword::FROM
            | Keyword::GROUP
            | Keyword::HAVING
            | Keyword::INTERSECT
            | Keyword::INTO
            | Keyword::MINUS
            | Keyword::ORDER
            | Keyword::SELECT
            | Keyword::UNION
            | Keyword::WHERE
            | Keyword::WITH => false,

            // Any other word is considered an alias
            _ => true,
        }
    }

    fn is_table_alias(&self, kw: &Keyword, parser: &mut Parser) -> bool {
        match kw {
            // The following keywords can be considered an alias as long as
            // they are not followed by other tokens that may change their meaning
            Keyword::RETURNING
            | Keyword::INNER
            | Keyword::USING
            | Keyword::PIVOT
            | Keyword::UNPIVOT
            | Keyword::EXCEPT
            | Keyword::MATCH_RECOGNIZE
                if !matches!(parser.peek_token_ref().token, Token::SemiColon | Token::EOF) =>
            {
                false
            }

            // `LIMIT` can be considered an alias as long as it's not followed by a value. For example:
            // `SELECT * FROM tbl LIMIT WHERE 1=1` - alias
            // `SELECT * FROM tbl LIMIT 3` - not an alias
            Keyword::LIMIT | Keyword::OFFSET if peek_for_limit_options(parser) => false,

            // `FETCH` can be considered an alias as long as it's not followed by `FIRST`` or `NEXT`
            // which would give it a different meanings, for example:
            // `SELECT * FROM tbl FETCH FIRST 10 ROWS` - not an alias
            // `SELECT * FROM tbl FETCH 10` - not an alias
            Keyword::FETCH
                if parser
                    .peek_one_of_keywords(&[Keyword::FIRST, Keyword::NEXT])
                    .is_some()
                    || peek_for_limit_options(parser) =>
            {
                false
            }

            // All sorts of join-related keywords can be considered aliases unless additional
            // keywords change their meaning.
            Keyword::RIGHT | Keyword::LEFT | Keyword::SEMI | Keyword::ANTI
                if parser
                    .peek_one_of_keywords(&[Keyword::JOIN, Keyword::OUTER])
                    .is_some() =>
            {
                false
            }

            Keyword::GLOBAL if parser.peek_keyword(Keyword::FULL) => false,

            // Reserved keywords by the Snowflake dialect, which seem to be less strictive
            // than what is listed in `keywords::RESERVED_FOR_TABLE_ALIAS`. The following
            // keywords were tested with the this statement: `SELECT <KW>.* FROM tbl <KW>`.
            Keyword::WITH
            | Keyword::ORDER
            | Keyword::SELECT
            | Keyword::WHERE
            | Keyword::GROUP
            | Keyword::HAVING
            | Keyword::LATERAL
            | Keyword::UNION
            | Keyword::INTERSECT
            | Keyword::MINUS
            | Keyword::ON
            | Keyword::JOIN
            | Keyword::INNER
            | Keyword::CROSS
            | Keyword::FULL
            | Keyword::LEFT
            | Keyword::RIGHT
            | Keyword::NATURAL
            | Keyword::USING
            | Keyword::ASOF
            | Keyword::MATCH_CONDITION
            | Keyword::SET
            | Keyword::QUALIFY
            | Keyword::FOR
            | Keyword::START
            | Keyword::CONNECT
            | Keyword::SAMPLE
            | Keyword::TABLESAMPLE
            | Keyword::FROM => false,

            // Any other word is considered an alias
            _ => true,
        }
    }

    fn is_table_factor(&self, kw: &Keyword, parser: &mut Parser) -> bool {
        match kw {
            Keyword::LIMIT if peek_for_limit_options(parser) => false,
            // Table function
            Keyword::TABLE if matches!(parser.peek_token_ref().token, Token::LParen) => true,
            _ => !RESERVED_KEYWORDS_FOR_TABLE_FACTOR.contains(kw),
        }
    }

    /// See: <https://docs.snowflake.com/en/sql-reference/constructs/at-before>
    fn supports_table_versioning(&self) -> bool {
        true
    }

    /// See: <https://docs.snowflake.com/en/sql-reference/constructs/group-by>
    fn supports_group_by_expr(&self) -> bool {
        true
    }

    /// See: <https://docs.snowflake.com/en/sql-reference/constructs/connect-by>
    fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword] {
        &RESERVED_KEYWORDS_FOR_SELECT_ITEM_OPERATOR
    }

    fn supports_space_separated_column_options(&self) -> bool {
        true
    }

    fn supports_comma_separated_drop_column_list(&self) -> bool {
        true
    }

    fn is_identifier_generating_function_name(
        &self,
        ident: &Ident,
        name_parts: &[ObjectNamePart],
    ) -> bool {
        ident.quote_style.is_none()
            && ident.value.to_lowercase() == "identifier"
            && !name_parts
                .iter()
                .any(|p| matches!(p, ObjectNamePart::Function(_)))
    }

    // For example: `SELECT IDENTIFIER('alias1').* FROM tbl AS alias1`
    fn supports_select_expr_star(&self) -> bool {
        true
    }

    fn supports_select_wildcard_exclude(&self) -> bool {
        true
    }

    fn supports_semantic_view_table_factor(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>
    fn supports_select_wildcard_replace(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>
    fn supports_select_wildcard_ilike(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>
    fn supports_select_wildcard_rename(&self) -> bool {
        true
    }

    /// See <https://docs.snowflake.com/en/user-guide/querying-semistructured#label-higher-order-functions>
    fn supports_lambda_functions(&self) -> bool {
        true
    }

    fn supports_comma_separated_trim(&self) -> bool {
        true
    }
}

// Peeks ahead to identify tokens that are expected after
// a LIMIT/FETCH keyword.
fn peek_for_limit_options(parser: &Parser) -> bool {
    match &parser.peek_token_ref().token {
        Token::Number(_, _) | Token::Placeholder(_) => true,
        Token::SingleQuotedString(val) if val.is_empty() => true,
        Token::DollarQuotedString(DollarQuotedString { value, .. }) if value.is_empty() => true,
        Token::Word(w) if w.keyword == Keyword::NULL => true,
        _ => false,
    }
}





















pub fn parse_stage_name_identifier(parser: &mut Parser) -> Result<Ident, ParserError> {
    let mut ident = String::new();
    while let Some(next_token) = parser.next_token_no_skip() {
        match &next_token.token {
            Token::Whitespace(_) | Token::SemiColon => break,
            Token::Period => {
                parser.prev_token();
                break;
            }
            Token::LParen | Token::RParen => {
                parser.prev_token();
                break;
            }
            Token::AtSign => ident.push('@'),
            Token::Tilde => ident.push('~'),
            Token::Mod => ident.push('%'),
            Token::Div => ident.push('/'),
            Token::Plus => ident.push('+'),
            Token::Minus => ident.push('-'),
            Token::Eq => ident.push('='),
            Token::Colon => ident.push(':'),
            Token::Number(n, _) => ident.push_str(n),
            Token::Word(w) => ident.push_str(&w.to_string()),
            _ => return parser.expected_ref("stage name identifier", parser.peek_token_ref()),
        }
    }
    Ok(Ident::new(ident))
}

/// Parses a Snowflake stage name, which may start with `@` for internal stages.
/// Examples: `@mystage`, `@namespace.stage`, `schema.table`
pub fn parse_snowflake_stage_name(parser: &mut Parser) -> Result<ObjectName, ParserError> {
    match parser.next_token().token {
        Token::AtSign => {
            parser.prev_token();
            let mut idents = vec![];
            loop {
                idents.push(parse_stage_name_identifier(parser)?);
                if !parser.consume_token(&Token::Period) {
                    break;
                }
            }
            Ok(ObjectName::from(idents))
        }
        _ => {
            parser.prev_token();
            Ok(parser.parse_object_name(false)?)
        }
    }
}











/// Parsing a property of identity or autoincrement column option
/// Syntax:
/// ```sql
/// [ (seed , increment) | START num INCREMENT num ] [ ORDER | NOORDER ]
/// ```
/// [Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table
fn parse_identity_property(parser: &mut Parser) -> Result<IdentityProperty, ParserError> {
    let parameters = if parser.consume_token(&Token::LParen) {
        let seed = parser.parse_number()?;
        parser.expect_token(&Token::Comma)?;
        let increment = parser.parse_number()?;
        parser.expect_token(&Token::RParen)?;

        Some(IdentityPropertyFormatKind::FunctionCall(
            IdentityParameters { seed, increment },
        ))
    } else if parser.parse_keyword(Keyword::START) {
        let seed = parser.parse_number()?;
        parser.expect_keyword_is(Keyword::INCREMENT)?;
        let increment = parser.parse_number()?;

        Some(IdentityPropertyFormatKind::StartAndIncrement(
            IdentityParameters { seed, increment },
        ))
    } else {
        None
    };
    let order = match parser.parse_one_of_keywords(&[Keyword::ORDER, Keyword::NOORDER]) {
        Some(Keyword::ORDER) => Some(IdentityPropertyOrder::Order),
        Some(Keyword::NOORDER) => Some(IdentityPropertyOrder::NoOrder),
        _ => None,
    };
    Ok(IdentityProperty { parameters, order })
}

/// Parsing a policy property of column option
/// Syntax:
/// ```sql
/// <policy_name> [ USING ( <col_name> , <cond_col1> , ... )
/// ```
/// [Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table
fn parse_column_policy_property(
    parser: &mut Parser,
    with: bool,
) -> Result<ColumnPolicyProperty, ParserError> {
    let policy_name = parser.parse_object_name(false)?;
    let using_columns = if parser.parse_keyword(Keyword::USING) {
        parser.expect_token(&Token::LParen)?;
        let columns = parser.parse_comma_separated(|p| p.parse_identifier())?;
        parser.expect_token(&Token::RParen)?;
        Some(columns)
    } else {
        None
    };

    Ok(ColumnPolicyProperty {
        with,
        policy_name,
        using_columns,
    })
}

/// Parsing tags list of column
/// Syntax:
/// ```sql
/// ( <tag_name> = '<tag_value>' [ , <tag_name> = '<tag_value>' , ... ] )
/// ```
/// [Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table
fn parse_column_tags(parser: &mut Parser, with: bool) -> Result<TagsColumnOption, ParserError> {
    parser.expect_token(&Token::LParen)?;
    let tags = parser.parse_comma_separated(Parser::parse_tag)?;
    parser.expect_token(&Token::RParen)?;

    Ok(TagsColumnOption { with, tags })
}












