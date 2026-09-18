extern "C" {
    pub fn __darwin_check_fd_set_overflow(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn deparseRawStmt(str_: StringInfo, raw_stmt: *mut RawStmt);
}
extern "C" {
    pub fn deparseRawStmtOpts(str_: StringInfo, raw_stmt: *mut RawStmt, opts: PostgresDeparseOpts);
}
extern "C" {
    pub fn pg_query_normalize(input: *const ::std::os::raw::c_char) -> PgQueryNormalizeResult;
}
extern "C" {
    pub fn pg_query_normalize_utility(
        input: *const ::std::os::raw::c_char,
    ) -> PgQueryNormalizeResult;
}
extern "C" {
    pub fn pg_query_scan(input: *const ::std::os::raw::c_char) -> PgQueryScanResult;
}
extern "C" {
    pub fn pg_query_parse(input: *const ::std::os::raw::c_char) -> PgQueryParseResult;
}
extern "C" {
    pub fn pg_query_parse_opts(
        input: *const ::std::os::raw::c_char,
        parser_options: ::std::os::raw::c_int,
    ) -> PgQueryParseResult;
}
extern "C" {
    pub fn pg_query_parse_protobuf(
        input: *const ::std::os::raw::c_char,
    ) -> PgQueryProtobufParseResult;
}
extern "C" {
    pub fn pg_query_parse_protobuf_opts(
        input: *const ::std::os::raw::c_char,
        parser_options: ::std::os::raw::c_int,
    ) -> PgQueryProtobufParseResult;
}
extern "C" {
    pub fn pg_query_parse_plpgsql(
        input: *const ::std::os::raw::c_char,
    ) -> PgQueryPlpgsqlParseResult;
}
extern "C" {
    pub fn pg_query_fingerprint(input: *const ::std::os::raw::c_char) -> PgQueryFingerprintResult;
}
extern "C" {
    pub fn pg_query_fingerprint_opts(
        input: *const ::std::os::raw::c_char,
        parser_options: ::std::os::raw::c_int,
    ) -> PgQueryFingerprintResult;
}
extern "C" {
    pub fn pg_query_split_with_scanner(input: *const ::std::os::raw::c_char) -> PgQuerySplitResult;
}
extern "C" {
    pub fn pg_query_split_with_parser(input: *const ::std::os::raw::c_char) -> PgQuerySplitResult;
}
extern "C" {
    pub fn pg_query_deparse_protobuf(parse_tree: PgQueryProtobuf) -> PgQueryDeparseResult;
}
extern "C" {
    pub fn pg_query_deparse_protobuf_opts(
        parse_tree: PgQueryProtobuf,
        opts: PostgresDeparseOpts,
    ) -> PgQueryDeparseResult;
}
extern "C" {
    pub fn pg_query_deparse_comments_for_query(
        query: *const ::std::os::raw::c_char,
    ) -> PgQueryDeparseCommentsResult;
}
extern "C" {
    pub fn pg_query_is_utility_stmt(query: *const ::std::os::raw::c_char)
        -> PgQueryIsUtilityResult;
}
extern "C" {
    pub fn pg_query_summary(
        input: *const ::std::os::raw::c_char,
        parser_options: ::std::os::raw::c_int,
        truncate_limit: ::std::os::raw::c_int,
    ) -> PgQuerySummaryParseResult;
}
extern "C" {
    pub fn pg_query_free_normalize_result(result: PgQueryNormalizeResult);
}
extern "C" {
    pub fn pg_query_free_scan_result(result: PgQueryScanResult);
}
extern "C" {
    pub fn pg_query_free_parse_result(result: PgQueryParseResult);
}
extern "C" {
    pub fn pg_query_free_split_result(result: PgQuerySplitResult);
}
extern "C" {
    pub fn pg_query_free_deparse_result(result: PgQueryDeparseResult);
}
extern "C" {
    pub fn pg_query_free_deparse_comments_result(result: PgQueryDeparseCommentsResult);
}
extern "C" {
    pub fn pg_query_free_protobuf_parse_result(result: PgQueryProtobufParseResult);
}
extern "C" {
    pub fn pg_query_free_plpgsql_parse_result(result: PgQueryPlpgsqlParseResult);
}
extern "C" {
    pub fn pg_query_free_fingerprint_result(result: PgQueryFingerprintResult);
}
extern "C" {
    pub fn pg_query_free_is_utility_result(result: PgQueryIsUtilityResult);
}
extern "C" {
    pub fn pg_query_free_summary_parse_result(result: PgQuerySummaryParseResult);
}
extern "C" {
    pub fn pg_query_exit();
}
extern "C" {
    pub fn pg_query_init();
}
