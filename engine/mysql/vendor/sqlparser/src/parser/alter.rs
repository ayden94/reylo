// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Changed in Reylo's copy (engine/mysql/vendor/sqlparser/README.md).

//! SQL Parser for ALTER

#[cfg(not(feature = "std"))]
use alloc::{string::ToString, vec};

use super::{Parser, ParserError};
use crate::{
    ast::{
        helpers::key_value_options::{KeyValueOptions, KeyValueOptionsDelimiter}, AlterPolicy, AlterPolicyOperation, AlterUser,
        AlterUserAddMfaMethodOtp, AlterUserAddRoleDelegation, AlterUserModifyMfaMethod,
        AlterUserPassword, AlterUserRemoveRoleDelegation, AlterUserSetPolicy, MfaMethodKind, UserPolicyKind,
    },
    keywords::Keyword,
    tokenizer::Token,
};

impl Parser<'_> {


    /// Parse ALTER POLICY statement
    /// ```sql
    /// ALTER POLICY policy_name ON table_name [ RENAME TO new_name ]
    /// or
    /// ALTER POLICY policy_name ON table_name
    /// [ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
    /// [ USING ( using_expression ) ]
    /// [ WITH CHECK ( check_expression ) ]
    /// ```
    ///
    /// [PostgreSQL](https://www.postgresql.org/docs/current/sql-alterpolicy.html)
    pub fn parse_alter_policy(&mut self) -> Result<AlterPolicy, ParserError> {
        let name = self.parse_identifier()?;
        self.expect_keyword_is(Keyword::ON)?;
        let table_name = self.parse_object_name(false)?;

        if self.parse_keyword(Keyword::RENAME) {
            self.expect_keyword_is(Keyword::TO)?;
            let new_name = self.parse_identifier()?;
            Ok(AlterPolicy {
                name,
                table_name,
                operation: AlterPolicyOperation::Rename { new_name },
            })
        } else {
            let to = if self.parse_keyword(Keyword::TO) {
                Some(self.parse_comma_separated(|p| p.parse_owner())?)
            } else {
                None
            };

            let using = if self.parse_keyword(Keyword::USING) {
                self.expect_token(&Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Some(expr)
            } else {
                None
            };

            let with_check = if self.parse_keywords(&[Keyword::WITH, Keyword::CHECK]) {
                self.expect_token(&Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Some(expr)
            } else {
                None
            };
            Ok(AlterPolicy {
                name,
                table_name,
                operation: AlterPolicyOperation::Apply {
                    to,
                    using,
                    with_check,
                },
            })
        }
    }



    /// Parse an `ALTER USER` statement
    /// ```sql
    /// ALTER USER [ IF EXISTS ] [ <name> ] [ OPTIONS ]
    /// ```
    pub fn parse_alter_user(&mut self) -> Result<AlterUser, ParserError> {
        let if_exists = self.parse_keywords(&[Keyword::IF, Keyword::EXISTS]);
        let name = self.parse_identifier()?;
        let _ = self.parse_keyword(Keyword::WITH);
        let rename_to = if self.parse_keywords(&[Keyword::RENAME, Keyword::TO]) {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        let reset_password = self.parse_keywords(&[Keyword::RESET, Keyword::PASSWORD]);
        let abort_all_queries =
            self.parse_keywords(&[Keyword::ABORT, Keyword::ALL, Keyword::QUERIES]);
        let add_role_delegation = if self.parse_keywords(&[
            Keyword::ADD,
            Keyword::DELEGATED,
            Keyword::AUTHORIZATION,
            Keyword::OF,
            Keyword::ROLE,
        ]) {
            let role = self.parse_identifier()?;
            self.expect_keywords(&[Keyword::TO, Keyword::SECURITY, Keyword::INTEGRATION])?;
            let integration = self.parse_identifier()?;
            Some(AlterUserAddRoleDelegation { role, integration })
        } else {
            None
        };
        let remove_role_delegation = if self.parse_keywords(&[Keyword::REMOVE, Keyword::DELEGATED])
        {
            let role = if self.parse_keywords(&[Keyword::AUTHORIZATION, Keyword::OF, Keyword::ROLE])
            {
                Some(self.parse_identifier()?)
            } else if self.parse_keyword(Keyword::AUTHORIZATIONS) {
                None
            } else {
                return self.expected_ref(
                    "REMOVE DELEGATED AUTHORIZATION OF ROLE | REMOVE DELEGATED AUTHORIZATIONS",
                    self.peek_token_ref(),
                );
            };
            self.expect_keywords(&[Keyword::FROM, Keyword::SECURITY, Keyword::INTEGRATION])?;
            let integration = self.parse_identifier()?;
            Some(AlterUserRemoveRoleDelegation { role, integration })
        } else {
            None
        };
        let enroll_mfa = self.parse_keywords(&[Keyword::ENROLL, Keyword::MFA]);
        let set_default_mfa_method =
            if self.parse_keywords(&[Keyword::SET, Keyword::DEFAULT_MFA_METHOD]) {
                Some(self.parse_mfa_method()?)
            } else {
                None
            };
        let remove_mfa_method =
            if self.parse_keywords(&[Keyword::REMOVE, Keyword::MFA, Keyword::METHOD]) {
                Some(self.parse_mfa_method()?)
            } else {
                None
            };
        let modify_mfa_method =
            if self.parse_keywords(&[Keyword::MODIFY, Keyword::MFA, Keyword::METHOD]) {
                let method = self.parse_mfa_method()?;
                self.expect_keywords(&[Keyword::SET, Keyword::COMMENT])?;
                let comment = self.parse_literal_string()?;
                Some(AlterUserModifyMfaMethod { method, comment })
            } else {
                None
            };
        let add_mfa_method_otp =
            if self.parse_keywords(&[Keyword::ADD, Keyword::MFA, Keyword::METHOD, Keyword::OTP]) {
                let count = if self.parse_keyword(Keyword::COUNT) {
                    self.expect_token(&Token::Eq)?;
                    Some(self.parse_value()?)
                } else {
                    None
                };
                Some(AlterUserAddMfaMethodOtp { count })
            } else {
                None
            };
        let set_policy =
            if self.parse_keywords(&[Keyword::SET, Keyword::AUTHENTICATION, Keyword::POLICY]) {
                Some(AlterUserSetPolicy {
                    policy_kind: UserPolicyKind::Authentication,
                    policy: self.parse_identifier()?,
                })
            } else if self.parse_keywords(&[Keyword::SET, Keyword::PASSWORD, Keyword::POLICY]) {
                Some(AlterUserSetPolicy {
                    policy_kind: UserPolicyKind::Password,
                    policy: self.parse_identifier()?,
                })
            } else if self.parse_keywords(&[Keyword::SET, Keyword::SESSION, Keyword::POLICY]) {
                Some(AlterUserSetPolicy {
                    policy_kind: UserPolicyKind::Session,
                    policy: self.parse_identifier()?,
                })
            } else {
                None
            };

        let unset_policy =
            if self.parse_keywords(&[Keyword::UNSET, Keyword::AUTHENTICATION, Keyword::POLICY]) {
                Some(UserPolicyKind::Authentication)
            } else if self.parse_keywords(&[Keyword::UNSET, Keyword::PASSWORD, Keyword::POLICY]) {
                Some(UserPolicyKind::Password)
            } else if self.parse_keywords(&[Keyword::UNSET, Keyword::SESSION, Keyword::POLICY]) {
                Some(UserPolicyKind::Session)
            } else {
                None
            };

        let set_tag = if self.parse_keywords(&[Keyword::SET, Keyword::TAG]) {
            self.parse_key_value_options(false, &[])?
        } else {
            KeyValueOptions {
                delimiter: KeyValueOptionsDelimiter::Comma,
                options: vec![],
            }
        };

        let unset_tag = if self.parse_keywords(&[Keyword::UNSET, Keyword::TAG]) {
            self.parse_comma_separated(Parser::parse_identifier)?
                .iter()
                .map(|i| i.to_string())
                .collect()
        } else {
            vec![]
        };

        let set_props = if self.parse_keyword(Keyword::SET) {
            self.parse_key_value_options(false, &[])?
        } else {
            KeyValueOptions {
                delimiter: KeyValueOptionsDelimiter::Comma,
                options: vec![],
            }
        };

        let unset_props = if self.parse_keyword(Keyword::UNSET) {
            self.parse_comma_separated(Parser::parse_identifier)?
                .iter()
                .map(|i| i.to_string())
                .collect()
        } else {
            vec![]
        };

        let encrypted = self.parse_keyword(Keyword::ENCRYPTED);
        let password = if self.parse_keyword(Keyword::PASSWORD) {
            let password = if self.parse_keyword(Keyword::NULL) {
                None
            } else {
                Some(self.parse_literal_string()?)
            };
            Some(AlterUserPassword {
                encrypted,
                password,
            })
        } else {
            None
        };

        Ok(AlterUser {
            if_exists,
            name,
            rename_to,
            reset_password,
            abort_all_queries,
            add_role_delegation,
            remove_role_delegation,
            enroll_mfa,
            set_default_mfa_method,
            remove_mfa_method,
            modify_mfa_method,
            add_mfa_method_otp,
            set_policy,
            unset_policy,
            set_tag,
            unset_tag,
            set_props,
            unset_props,
            password,
        })
    }

    fn parse_mfa_method(&mut self) -> Result<MfaMethodKind, ParserError> {
        if self.parse_keyword(Keyword::PASSKEY) {
            Ok(MfaMethodKind::PassKey)
        } else if self.parse_keyword(Keyword::TOTP) {
            Ok(MfaMethodKind::Totp)
        } else if self.parse_keyword(Keyword::DUO) {
            Ok(MfaMethodKind::Duo)
        } else {
            self.expected_ref("PASSKEY, TOTP or DUO", self.peek_token_ref())
        }
    }






}
