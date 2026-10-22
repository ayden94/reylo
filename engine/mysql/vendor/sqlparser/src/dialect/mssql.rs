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

use crate::ast::GranteesType;
use crate::dialect::Dialect;
use crate::keywords::Keyword;
use crate::parser::{Parser, ParserError};
use crate::tokenizer::Token;
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

/// A [`Dialect`] for [Microsoft SQL Server](https://www.microsoft.com/en-us/sql-server/)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MsSqlDialect {}

impl Dialect for MsSqlDialect {
    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '"' || ch == '['
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        // See https://docs.microsoft.com/en-us/sql/relational-databases/databases/database-identifiers?view=sql-server-2017#rules-for-regular-identifiers
        ch.is_alphabetic() || ch == '_' || ch == '#' || ch == '@'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_alphabetic()
            || ch.is_ascii_digit()
            || ch == '@'
            || ch == '$'
            || ch == '#'
            || ch == '_'
    }

    fn identifier_quote_style(&self, _identifier: &str) -> Option<char> {
        Some('[')
    }

    /// SQL Server has `CONVERT(type, value)` instead of `CONVERT(value, type)`
    /// <https://learn.microsoft.com/en-us/sql/t-sql/functions/cast-and-convert-transact-sql?view=sql-server-ver16>
    fn convert_type_before_value(&self) -> bool {
        true
    }

    fn supports_outer_join_operator(&self) -> bool {
        true
    }

    /// SQL Server supports `$` as a prefix for money literals
    /// <https://learn.microsoft.com/en-us/sql/t-sql/data-types/constants-transact-sql?view=sql-server-ver17#money-constants>
    fn supports_dollar_as_money_prefix(&self) -> bool {
        true
    }

    fn supports_connect_by(&self) -> bool {
        true
    }

    fn supports_eq_alias_assignment(&self) -> bool {
        true
    }

    fn supports_try_convert(&self) -> bool {
        true
    }

    /// In MSSQL, there is no boolean type, and `true` and `false` are valid column names
    fn supports_boolean_literals(&self) -> bool {
        false
    }

    fn supports_named_fn_args_with_colon_operator(&self) -> bool {
        true
    }

    fn supports_named_fn_args_with_expr_name(&self) -> bool {
        true
    }

    fn supports_named_fn_args_with_rarrow_operator(&self) -> bool {
        false
    }

    fn supports_start_transaction_modifier(&self) -> bool {
        true
    }

    fn supports_end_transaction_modifier(&self) -> bool {
        true
    }

    /// See: <https://learn.microsoft.com/en-us/sql/t-sql/statements/set-statements-transact-sql>
    fn supports_set_stmt_without_operator(&self) -> bool {
        true
    }

    /// See: <https://learn.microsoft.com/en-us/sql/relational-databases/tables/querying-data-in-a-system-versioned-temporal-table>
    fn supports_table_versioning(&self) -> bool {
        true
    }

    /// See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/slash-star-comment-transact-sql?view=sql-server-ver16>
    fn supports_nested_comments(&self) -> bool {
        true
    }

    /// See <https://learn.microsoft.com/en-us/sql/t-sql/queries/from-transact-sql>
    fn supports_object_name_double_dot_notation(&self) -> bool {
        true
    }

    /// See <https://learn.microsoft.com/en-us/sql/relational-databases/security/authentication-access/server-level-roles>
    fn get_reserved_grantees_types(&self) -> &[GranteesType] {
        &[GranteesType::Public]
    }

    fn is_select_item_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser) -> bool {
        match kw {
            // List of keywords that cannot be used as select item (column) aliases in MSSQL
            // regardless of whether the alias is explicit or implicit.
            //
            // These are T-SQL statement-starting keywords; allowing them as implicit aliases
            // causes the parser to consume the keyword as an alias for the previous expression,
            // then fail on the token that follows (e.g. `TABLE`, `@var`, `sp_name`, …).
            Keyword::IF
            | Keyword::ELSE
            | Keyword::DECLARE
            | Keyword::EXEC
            | Keyword::EXECUTE
            | Keyword::INSERT
            | Keyword::UPDATE
            | Keyword::DELETE
            | Keyword::DROP
            | Keyword::CREATE
            | Keyword::ALTER
            | Keyword::TRUNCATE
            | Keyword::PRINT
            | Keyword::WHILE
            | Keyword::RETURN
            | Keyword::THROW
            | Keyword::RAISERROR
            | Keyword::MERGE => false,
            _ => explicit || self.is_column_alias(kw, parser),
        }
    }

    fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser) -> bool {
        match kw {
            // List of keywords that cannot be used as table aliases in MSSQL
            // regardless of whether the alias is explicit or implicit.
            //
            // These are T-SQL statement-starting keywords. Without blocking them here,
            // a bare `SELECT * FROM t` followed by a newline and one of these keywords
            // would cause the parser to consume the keyword as a table alias for `t`,
            // then fail on the token that follows (e.g. `@var`, `sp_name`, `TABLE`, …).
            //
            // `SET` is already covered by the global `RESERVED_FOR_TABLE_ALIAS` list;
            // the keywords below are MSSQL-specific additions.
            Keyword::IF
            | Keyword::ELSE
            | Keyword::DECLARE
            | Keyword::EXEC
            | Keyword::EXECUTE
            | Keyword::INSERT
            | Keyword::UPDATE
            | Keyword::DELETE
            | Keyword::DROP
            | Keyword::CREATE
            | Keyword::ALTER
            | Keyword::TRUNCATE
            | Keyword::PRINT
            | Keyword::WHILE
            | Keyword::RETURN
            | Keyword::THROW
            | Keyword::RAISERROR
            | Keyword::MERGE => false,
            _ => explicit || self.is_table_alias(kw, parser),
        }
    }



    fn get_next_precedence(&self, parser: &Parser) -> Option<Result<u8, ParserError>> {
        let token = parser.peek_token_ref();
        match &token.token {
            // lowest prec to prevent it from turning into a binary op
            Token::Colon => Some(Ok(self.prec_unknown())),
            _ => None,
        }
    }
}

impl MsSqlDialect {





}
