/*
** SQLite's own resolved parse tree, as JSON, for the engine's SQLite analysis.
**
** Built as ONE translation unit with the patched amalgamation (csrc/patch-sqlite.py inserts five
** one-line hooks, each right after name resolution), so the SQLITE_PRIVATE internals are callable
** here. The engine gives this file the schema DDL and one statement; the hooks fire while SQLite
** prepares the statement against an in-memory replay of that schema, and the walker serializes
** what SQLite resolved: every expression with sqlite3ExprAffinity and sqlite3ExprCanBeNull, every
** column reference with its Table, Column, NOT NULL and outer-join mark (EP_CanBeNull), the FROM
** items with their join type and cursor, the DML target columns per value position, and each
** table's columns and unique keys. The rules (function result types, arithmetic, the row-count
** proof) live in Rust; nothing here decides a type. A host that registers scalar functions of its
** own on its connection names them with their argument counts, and the replay registers a stub
** for each, so a statement calling one prepares here as it does there.
*/
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct Parse; struct Select; struct SelectDest; struct Table; struct IdList; struct ExprList; struct Expr; struct Upsert; struct SrcList;
typedef void (*porg_select_fn)(struct Parse*, struct Select*, struct SelectDest*);
typedef void (*porg_insert_fn)(struct Parse*, struct Table*, struct IdList*, struct ExprList*, struct Select*, int, struct Upsert*);
typedef void (*porg_update_fn)(struct Parse*, struct Table*, struct ExprList*, struct Expr*, int*, struct SrcList*, int, struct Upsert*);
typedef void (*porg_delete_fn)(struct Parse*, struct Table*, struct Expr*);
typedef void (*porg_returning_fn)(struct Parse*, struct Table*, struct ExprList*);
static porg_select_fn porg_select_hook;
static porg_insert_fn porg_insert_hook;
static porg_update_fn porg_update_hook;
static porg_delete_fn porg_delete_hook;
static porg_returning_fn porg_returning_hook;

#include "sqlite3-patched.c"

/* ---- capture state: one statement at a time ------------------------------------------------ */

static sqlite3 *gDb;            /* the in-memory schema replay */
static char *gDdlKey;           /* the DDL it was built from */
static sqlite3_str *gStmt;      /* the top-level SELECT, or a DML's SELECT source */
static sqlite3_str *gDml;       /* the INSERT / UPDATE / DELETE */
static sqlite3_str *gReturning; /* the RETURNING list */
static int gCaptured;           /* the first top-level hook has fired */
static int gWantSelect;         /* a DML is waiting for its SELECT source */
static Table *gTables[128];
static int gnTables;

static void tableSeen(Table *pTab){
  int i;
  if( pTab==0 || IsView(pTab) || IsVirtual(pTab) ) return;
  for(i=0; i<gnTables; i++) if( gTables[i]==pTab ) return;
  if( gnTables<128 ) gTables[gnTables++] = pTab;
}

/* ---- JSON helpers -------------------------------------------------------------------------- */

static void js(sqlite3_str *s, const char *z){
  if( z==0 ){ sqlite3_str_appendall(s, "null"); return; }
  sqlite3_str_appendchar(s, 1, '"');
  for(; *z; z++){
    unsigned char c = (unsigned char)*z;
    switch( c ){
      case '"': sqlite3_str_appendall(s, "\\\""); break;
      case '\\': sqlite3_str_appendall(s, "\\\\"); break;
      case '\n': sqlite3_str_appendall(s, "\\n"); break;
      case '\r': sqlite3_str_appendall(s, "\\r"); break;
      case '\t': sqlite3_str_appendall(s, "\\t"); break;
      default:
        if( c<0x20 ) sqlite3_str_appendf(s, "\\u%04x", c);
        else sqlite3_str_appendchar(s, 1, (char)c);
    }
  }
  sqlite3_str_appendchar(s, 1, '"');
}
static void key(sqlite3_str *s, const char *k){ js(s, k); sqlite3_str_appendchar(s, 1, ':'); }
static void kv_str(sqlite3_str *s, const char *k, const char *v){ key(s, k); js(s, v); sqlite3_str_appendchar(s, 1, ','); }
static void kv_int(sqlite3_str *s, const char *k, int v){ key(s, k); sqlite3_str_appendf(s, "%d,", v); }
static void kv_bool(sqlite3_str *s, const char *k, int v){ key(s, k); sqlite3_str_appendall(s, v ? "true," : "false,"); }
/* Closes an object whose last member left a trailing comma. */
static void closeObj(sqlite3_str *s){
  int n = sqlite3_str_length(s);
  char *z = sqlite3_str_value(s);
  if( n>0 && z[n-1]==',' ) z[n-1] = '}'; else sqlite3_str_appendchar(s, 1, '}');
}
static void closeArr(sqlite3_str *s){
  int n = sqlite3_str_length(s);
  char *z = sqlite3_str_value(s);
  if( n>0 && z[n-1]==',' ) z[n-1] = ']'; else sqlite3_str_appendchar(s, 1, ']');
}

static const char *affName(char a){
  switch( a ){
    case SQLITE_AFF_BLOB: return "blob";
    case SQLITE_AFF_TEXT: return "text";
    case SQLITE_AFF_NUMERIC: return "numeric";
    case SQLITE_AFF_INTEGER: return "integer";
    case SQLITE_AFF_REAL: return "real";
    case SQLITE_AFF_FLEXNUM: return "numeric";
    default: return "none";
  }
}
static const char *opName(int op){
  switch( op ){
    case TK_COLUMN: case TK_AGG_COLUMN: case TK_TRIGGER: return "column";
    case TK_VARIABLE: return "variable"; case TK_INTEGER: return "integer"; case TK_FLOAT: return "float";
    case TK_STRING: return "string"; case TK_BLOB: return "blob"; case TK_NULL: return "null"; case TK_TRUEFALSE: return "truefalse";
    case TK_FUNCTION: case TK_AGG_FUNCTION: return "function";
    case TK_CAST: return "cast"; case TK_CONCAT: return "||"; case TK_PLUS: return "+"; case TK_MINUS: return "-"; case TK_STAR: return "*";
    case TK_SLASH: return "/"; case TK_REM: return "%"; case TK_EQ: return "="; case TK_NE: return "!="; case TK_LT: return "<"; case TK_LE: return "<=";
    case TK_GT: return ">"; case TK_GE: return ">="; case TK_IS: return "is"; case TK_ISNOT: return "is not"; case TK_IN: return "in"; case TK_BETWEEN: return "between";
    case TK_AND: return "and"; case TK_OR: return "or"; case TK_NOT: return "not"; case TK_BITAND: return "&"; case TK_BITOR: return "|";
    case TK_LSHIFT: return "<<"; case TK_RSHIFT: return ">>"; case TK_BITNOT: return "~"; case TK_UMINUS: return "neg"; case TK_UPLUS: return "pos";
    case TK_ISNULL: return "isnull"; case TK_NOTNULL: return "notnull"; case TK_SELECT: return "select"; case TK_EXISTS: return "exists";
    case TK_CASE: return "case"; case TK_COLLATE: return "collate"; case TK_VECTOR: return "vector"; case TK_SELECT_COLUMN: return "select_column";
    case TK_IF_NULL_ROW: return "if_null_row"; case TK_REGISTER: return "register"; case TK_ID: return "id"; case TK_ROW: return "row";
    default: return "other";
  }
}

static void selectJson(sqlite3_str *s, Select *p);
static void expr(sqlite3_str *s, Expr *p, Select *pSel);

static void exprList(sqlite3_str *s, ExprList *pList, Select *pSel){
  int i;
  sqlite3_str_appendchar(s, 1, '[');
  for(i=0; pList && i<pList->nExpr; i++){ expr(s, pList->a[i].pExpr, pSel); sqlite3_str_appendchar(s, 1, ','); }
  closeArr(s);
}

static int outerCursor(Select *pSel, int iCursor){
  int i;
  if( pSel==0 || pSel->pSrc==0 ) return 0;
  for(i=0; i<pSel->pSrc->nSrc; i++){
    SrcItem *pItem = &pSel->pSrc->a[i];
    if( pItem->iCursor==iCursor ) return (pItem->fg.jointype & (JT_LEFT|JT_LTORJ))!=0;
  }
  return 0;
}

/* The name a statement reads a rowid table's rowid by: "rowid", "_rowid_" or "oid", the first no
** declared column has, since a declared column of that name hides the rowid. 0 when all three do. */
static const char *rowidName(Table *pTab){
  static const char *azNames[] = { "rowid", "_rowid_", "oid" };
  int i, j;
  for(i=0; i<3; i++){
    for(j=0; j<pTab->nCol && sqlite3StrICmp(pTab->aCol[j].zCnName, azNames[i])!=0; j++){}
    if( j==pTab->nCol ) return azNames[i];
  }
  return 0;
}

/* The provenance members of a column reference (the object is already open). */
static void columnRef(sqlite3_str *s, Expr *p, Select *pSel){
  Table *pTab = ExprUseYTab(p) ? p->y.pTab : 0;
  int iCol = p->iColumn;
  kv_int(s, "cursor", p->iTable);
  kv_int(s, "index", iCol);
  if( pTab==0 ){ kv_bool(s, "resolved", 0); return; }
  tableSeen(pTab);
  kv_str(s, "table", pTab->zName);
  if( iCol<0 ) iCol = pTab->iPKey;
  if( iCol<0 ){
    kv_str(s, "column", rowidName(pTab) ? rowidName(pTab) : "rowid");
    kv_str(s, "affinity", "integer");
    kv_bool(s, "notNull", 1);
    kv_bool(s, "rowid", 1);
  }else{
    Column *pCol = &pTab->aCol[iCol];
    kv_str(s, "column", pCol->zCnName);
    kv_str(s, "declared", sqlite3ColumnType(pCol, ""));
    kv_str(s, "affinity", affName(pCol->affinity));
    /* The collation a comparison with the column uses, where the column decides it. */
    kv_str(s, "coll", sqlite3ColumnColl(pCol) ? sqlite3ColumnColl(pCol) : sqlite3StrBINARY);
    kv_bool(s, "notNull", pCol->notNull!=0);
    kv_bool(s, "rowid", p->iColumn<0 || iCol==pTab->iPKey);
    if( pCol->colFlags & COLFLAG_GENERATED ) kv_bool(s, "generated", 1);
  }
  kv_bool(s, "outer", ExprHasProperty(p, EP_CanBeNull) || (p->op==TK_COLUMN && outerCursor(pSel, p->iTable)));
  if( pTab->tabFlags & TF_Strict ) kv_bool(s, "strict", 1);
  if( IsView(pTab) ) kv_bool(s, "view", 1);
}

static void expr(sqlite3_str *s, Expr *p, Select *pSel){
  int op;
  if( p==0 ){ sqlite3_str_appendall(s, "null"); return; }
  op = p->op;
  if( op==TK_REGISTER ) op = p->op2;
  sqlite3_str_appendchar(s, 1, '{');
  kv_str(s, "op", opName(op));
  kv_str(s, "aff", affName(sqlite3ExprAffinity(p)));
  kv_bool(s, "canBeNull", sqlite3ExprCanBeNull(p));
  /* SQLite moves every ON term into the WHERE; one from an outer join's ON says so, and names the
  ** cursor of the join's right side, which the term restricts without removing a row. */
  if( ExprHasProperty(p, EP_OuterON) ){ kv_bool(s, "outerOn", 1); kv_int(s, "join", p->w.iJoin); }
  switch( op ){
    case TK_COLUMN: case TK_AGG_COLUMN: case TK_TRIGGER:
      columnRef(s, p, pSel);
      break;
    case TK_VARIABLE:
      if( ExprHasProperty(p, EP_IntValue) ){ key(s, "name"); sqlite3_str_appendf(s, "\"?%d\",", p->iColumn); }
      else kv_str(s, "name", p->u.zToken);
      kv_int(s, "index", p->iColumn);
      break;
    case TK_INTEGER:
      if( ExprHasProperty(p, EP_IntValue) ){ key(s, "value"); sqlite3_str_appendf(s, "\"%d\",", p->u.iValue); }
      else kv_str(s, "value", p->u.zToken);
      break;
    case TK_FLOAT: case TK_STRING: case TK_BLOB: case TK_TRUEFALSE: case TK_ID:
      kv_str(s, "value", ExprHasProperty(p, EP_IntValue) ? "" : p->u.zToken);
      break;
    case TK_FUNCTION: case TK_AGG_FUNCTION: {
      char *zName = sqlite3_mprintf("%s", ExprHasProperty(p, EP_IntValue) ? "" : p->u.zToken);
      char *z;
      for(z=zName; z && *z; z++) *z = (char)sqlite3Tolower(*z);
      kv_str(s, "name", zName);
      sqlite3_free(zName);
      kv_bool(s, "agg", p->op==TK_AGG_FUNCTION);
      kv_bool(s, "window", ExprHasProperty(p, EP_WinFunc));
      kv_bool(s, "distinct", ExprHasProperty(p, EP_Distinct));
      kv_bool(s, "infix", ExprHasProperty(p, EP_InfixFunc));
      key(s, "args");
      if( !ExprHasProperty(p, EP_TokenOnly|EP_Reduced) && ExprUseXList(p) ) exprList(s, p->x.pList, pSel);
      else sqlite3_str_appendall(s, "[]");
      sqlite3_str_appendchar(s, 1, ',');
      break;
    }
    case TK_CAST:
      kv_str(s, "to", ExprHasProperty(p, EP_IntValue) ? "" : p->u.zToken);
      kv_str(s, "affinity", affName(sqlite3AffinityType(ExprHasProperty(p, EP_IntValue) ? "" : p->u.zToken, 0)));
      /* fall through: the operand is pLeft */
      key(s, "left"); expr(s, ExprHasProperty(p, EP_TokenOnly) ? 0 : p->pLeft, pSel); sqlite3_str_appendchar(s, 1, ',');
      break;
    case TK_IN:
      key(s, "left"); expr(s, p->pLeft, pSel); sqlite3_str_appendchar(s, 1, ',');
      if( ExprUseXSelect(p) ){ key(s, "select"); selectJson(s, p->x.pSelect); }
      else { key(s, "list"); exprList(s, p->x.pList, pSel); }
      sqlite3_str_appendchar(s, 1, ',');
      break;
    case TK_BETWEEN: case TK_CASE: case TK_VECTOR:
      key(s, "left"); expr(s, p->pLeft, pSel); sqlite3_str_appendchar(s, 1, ',');
      key(s, "list"); if( ExprUseXList(p) ) exprList(s, p->x.pList, pSel); else sqlite3_str_appendall(s, "[]");
      sqlite3_str_appendchar(s, 1, ',');
      break;
    case TK_SELECT: case TK_EXISTS:
      key(s, "select"); if( ExprUseXSelect(p) ) selectJson(s, p->x.pSelect); else sqlite3_str_appendall(s, "null");
      sqlite3_str_appendchar(s, 1, ',');
      break;
    case TK_SELECT_COLUMN:
      kv_int(s, "index", p->iColumn);
      key(s, "left"); expr(s, p->pLeft, pSel); sqlite3_str_appendchar(s, 1, ',');
      break;
    default:
      if( !ExprHasProperty(p, EP_TokenOnly) ){
        if( p->pLeft ){ key(s, "left"); expr(s, p->pLeft, pSel); sqlite3_str_appendchar(s, 1, ','); }
        if( p->pRight ){ key(s, "right"); expr(s, p->pRight, pSel); sqlite3_str_appendchar(s, 1, ','); }
      }
      break;
  }
  closeObj(s);
}

static void selectJson(sqlite3_str *s, Select *p){
  int i;
  if( p==0 ){ sqlite3_str_appendall(s, "null"); return; }
  sqlite3_str_appendchar(s, 1, '{');
  key(s, "columns"); sqlite3_str_appendchar(s, 1, '[');
  for(i=0; p->pEList && i<p->pEList->nExpr; i++){
    struct ExprList_item *pItem = &p->pEList->a[i];
    sqlite3_str_appendchar(s, 1, '{');
    kv_str(s, "name", pItem->zEName);
    kv_bool(s, "named", pItem->fg.eEName==ENAME_NAME);
    key(s, "expr"); expr(s, pItem->pExpr, p); sqlite3_str_appendchar(s, 1, ',');
    closeObj(s); sqlite3_str_appendchar(s, 1, ',');
  }
  closeArr(s); sqlite3_str_appendchar(s, 1, ',');
  key(s, "from"); sqlite3_str_appendchar(s, 1, '[');
  for(i=0; p->pSrc && i<p->pSrc->nSrc; i++){
    SrcItem *pItem = &p->pSrc->a[i];
    sqlite3_str_appendchar(s, 1, '{');
    kv_int(s, "cursor", pItem->iCursor);
    kv_str(s, "table", pItem->zName);
    kv_str(s, "alias", pItem->zAlias);
    kv_bool(s, "outer", (pItem->fg.jointype & (JT_LEFT|JT_LTORJ))!=0);
    kv_bool(s, "cross", (pItem->fg.jointype & JT_CROSS)!=0);
    kv_bool(s, "right", (pItem->fg.jointype & JT_RIGHT)!=0);
    kv_bool(s, "tabFunc", pItem->fg.isTabFunc);
    kv_bool(s, "cte", pItem->fg.isCte);
    if( pItem->pSTab ){
      tableSeen(pItem->pSTab);
      if( IsView(pItem->pSTab) ) kv_bool(s, "view", 1);
      if( IsVirtual(pItem->pSTab) ) kv_bool(s, "virtual", 1);
    }
    key(s, "subquery");
    if( pItem->fg.isSubquery && pItem->u4.pSubq ) selectJson(s, pItem->u4.pSubq->pSelect); else sqlite3_str_appendall(s, "null");
    sqlite3_str_appendchar(s, 1, ',');
    key(s, "on"); if( !pItem->fg.isUsing && pItem->u3.pOn ) expr(s, pItem->u3.pOn, p); else sqlite3_str_appendall(s, "null");
    sqlite3_str_appendchar(s, 1, ',');
    closeObj(s); sqlite3_str_appendchar(s, 1, ',');
  }
  closeArr(s); sqlite3_str_appendchar(s, 1, ',');
  key(s, "where"); expr(s, p->pWhere, p); sqlite3_str_appendchar(s, 1, ',');
  key(s, "groupBy"); exprList(s, p->pGroupBy, p); sqlite3_str_appendchar(s, 1, ',');
  key(s, "having"); expr(s, p->pHaving, p); sqlite3_str_appendchar(s, 1, ',');
  key(s, "orderBy"); exprList(s, p->pOrderBy, p); sqlite3_str_appendchar(s, 1, ',');
  key(s, "limit"); expr(s, p->pLimit ? p->pLimit->pLeft : 0, p); sqlite3_str_appendchar(s, 1, ',');
  key(s, "offset"); expr(s, p->pLimit ? p->pLimit->pRight : 0, p); sqlite3_str_appendchar(s, 1, ',');
  kv_bool(s, "distinct", (p->selFlags & SF_Distinct)!=0);
  kv_bool(s, "hasAgg", (p->selFlags & SF_HasAgg)!=0);
  kv_bool(s, "aggregate", (p->selFlags & SF_Aggregate)!=0);
  kv_bool(s, "values", (p->selFlags & SF_Values)!=0);
  kv_bool(s, "window", p->pWin!=0);
  if( p->pPrior ){
    kv_str(s, "op", p->op==TK_ALL ? "union all" : p->op==TK_UNION ? "union" : p->op==TK_INTERSECT ? "intersect" : "except");
    key(s, "prior"); selectJson(s, p->pPrior); sqlite3_str_appendchar(s, 1, ',');
  }
  closeObj(s);
}

/* ---- the hooks ------------------------------------------------------------------------------- */

static int skipHook(Parse *pParse){ return pParse->nErr || pParse->db->init.busy; }

static void selectHook(Parse *pParse, Select *p, SelectDest *pDest){
  if( skipHook(pParse) ) return;
  if( !gCaptured && pDest->eDest==SRT_Output ){
    gCaptured = 1;
    selectJson(gStmt, p);
  }else if( gWantSelect ){
    gWantSelect = 0;
    selectJson(gStmt, p);
  }
}

static int insertColumnIndex(Table *pTab, IdList *pColumn, int i){
  int j, seen = 0;
  if( pColumn ){
    if( i>=pColumn->nId ) return -1;
    for(j=0; j<pTab->nCol; j++) if( sqlite3StrICmp(pTab->aCol[j].zCnName, pColumn->a[i].zName)==0 ) return j;
    return -1;
  }
  for(j=0; j<pTab->nCol; j++){
    if( pTab->aCol[j].colFlags & COLFLAG_NOINSERT ) continue;
    if( seen++==i ) return j;
  }
  return -1;
}
static const char *onErrorName(int onError){
  switch( onError ){
    case OE_Ignore: return "ignore"; case OE_Replace: return "replace"; case OE_Abort: return "abort";
    case OE_Rollback: return "rollback"; case OE_Fail: return "fail"; default: return "default";
  }
}
static void upsert(sqlite3_str *s, Upsert *pUpsert){
  int i;
  if( pUpsert==0 ){ sqlite3_str_appendall(s, "null"); return; }
  sqlite3_str_appendchar(s, 1, '{');
  kv_bool(s, "doUpdate", pUpsert->isDoUpdate);
  key(s, "set"); sqlite3_str_appendchar(s, 1, '[');
  for(i=0; pUpsert->pUpsertSet && i<pUpsert->pUpsertSet->nExpr; i++){
    sqlite3_str_appendchar(s, 1, '{');
    kv_str(s, "column", pUpsert->pUpsertSet->a[i].zEName);
    key(s, "expr"); expr(s, pUpsert->pUpsertSet->a[i].pExpr, 0); sqlite3_str_appendchar(s, 1, ',');
    closeObj(s); sqlite3_str_appendchar(s, 1, ',');
  }
  closeArr(s); sqlite3_str_appendchar(s, 1, ',');
  key(s, "where"); expr(s, pUpsert->pUpsertWhere, 0); sqlite3_str_appendchar(s, 1, ',');
  closeObj(s);
}
static void insertHook(Parse *pParse, Table *pTab, IdList *pColumn, ExprList *pList, Select *pSelect, int onError, Upsert *pUpsert){
  int i, n;
  if( skipHook(pParse) || gCaptured ) return;
  gCaptured = 1;
  tableSeen(pTab);
  sqlite3_str_appendchar(gDml, 1, '{');
  kv_str(gDml, "kind", "insert");
  kv_str(gDml, "table", pTab->zName);
  kv_str(gDml, "onError", onErrorName(onError));
  key(gDml, "upsert"); upsert(gDml, pUpsert); sqlite3_str_appendchar(gDml, 1, ',');
  n = pList ? pList->nExpr : pSelect && pSelect->pEList ? pSelect->pEList->nExpr : 0;
  key(gDml, "columns"); sqlite3_str_appendchar(gDml, 1, '[');
  for(i=0; i<n; i++){
    int j = insertColumnIndex(pTab, pColumn, i);
    js(gDml, j>=0 ? pTab->aCol[j].zCnName : 0); sqlite3_str_appendchar(gDml, 1, ',');
  }
  closeArr(gDml); sqlite3_str_appendchar(gDml, 1, ',');
  key(gDml, "values"); if( pList ) exprList(gDml, pList, 0); else sqlite3_str_appendall(gDml, "null");
  sqlite3_str_appendchar(gDml, 1, ',');
  if( pSelect ){
    int rows = 0;
    Select *q;
    if( pSelect->selFlags & SF_Values ){ for(q=pSelect; q; q=q->pPrior) rows++; }
    kv_int(gDml, "rows", (pSelect->selFlags & SF_Values) ? rows : -1);
    gWantSelect = 1;
  }else{
    kv_int(gDml, "rows", 1);
  }
  closeObj(gDml);
}
static void updateHook(Parse *pParse, Table *pTab, ExprList *pChanges, Expr *pWhere, int *aXRef, SrcList *pTabList, int onError, Upsert *pUpsert){
  int i, j;
  if( skipHook(pParse) || gCaptured ) return;
  gCaptured = 1;
  tableSeen(pTab);
  sqlite3_str_appendchar(gDml, 1, '{');
  kv_str(gDml, "kind", pUpsert ? "upsert" : "update");
  kv_str(gDml, "table", pTab->zName);
  kv_str(gDml, "onError", onErrorName(onError));
  key(gDml, "set"); sqlite3_str_appendchar(gDml, 1, '[');
  for(i=0; pChanges && i<pChanges->nExpr; i++){
    int col = -1;
    for(j=0; j<pTab->nCol; j++) if( aXRef[j]==i ) col = j;
    sqlite3_str_appendchar(gDml, 1, '{');
    kv_str(gDml, "column", col>=0 ? pTab->aCol[col].zCnName : 0);
    key(gDml, "expr"); expr(gDml, pChanges->a[i].pExpr, 0); sqlite3_str_appendchar(gDml, 1, ',');
    closeObj(gDml); sqlite3_str_appendchar(gDml, 1, ',');
  }
  closeArr(gDml); sqlite3_str_appendchar(gDml, 1, ',');
  key(gDml, "where"); expr(gDml, pWhere, 0); sqlite3_str_appendchar(gDml, 1, ',');
  kv_int(gDml, "fromItems", pTabList ? pTabList->nSrc : 1);
  closeObj(gDml);
}
static void deleteHook(Parse *pParse, Table *pTab, Expr *pWhere){
  if( skipHook(pParse) || gCaptured ) return;
  gCaptured = 1;
  tableSeen(pTab);
  sqlite3_str_appendchar(gDml, 1, '{');
  kv_str(gDml, "kind", "delete");
  kv_str(gDml, "table", pTab->zName);
  key(gDml, "where"); expr(gDml, pWhere, 0); sqlite3_str_appendchar(gDml, 1, ',');
  closeObj(gDml);
}
static void returningHook(Parse *pParse, Table *pTab, ExprList *pNew){
  int i;
  if( skipHook(pParse) || sqlite3_str_length(gReturning) ) return;
  tableSeen(pTab);
  sqlite3_str_appendchar(gReturning, 1, '[');
  for(i=0; pNew && i<pNew->nExpr; i++){
    sqlite3_str_appendchar(gReturning, 1, '{');
    kv_str(gReturning, "name", pNew->a[i].zEName);
    key(gReturning, "expr"); expr(gReturning, pNew->a[i].pExpr, 0); sqlite3_str_appendchar(gReturning, 1, ',');
    closeObj(gReturning); sqlite3_str_appendchar(gReturning, 1, ',');
  }
  closeArr(gReturning);
}

/* ---- tables: columns and unique keys ------------------------------------------------------- */

static void tables(sqlite3_str *s){
  int t, i, k;
  sqlite3_str_appendchar(s, 1, '{');
  for(t=0; t<gnTables; t++){
    Table *pTab = gTables[t];
    Index *pIdx;
    key(s, pTab->zName); sqlite3_str_appendchar(s, 1, '{');
    kv_bool(s, "strict", (pTab->tabFlags & TF_Strict)!=0);
    kv_bool(s, "withoutRowid", (pTab->tabFlags & TF_WithoutRowid)!=0);
    kv_str(s, "rowidAlias", pTab->iPKey>=0 ? pTab->aCol[pTab->iPKey].zCnName : 0);
    key(s, "columns"); sqlite3_str_appendchar(s, 1, '[');
    for(i=0; i<pTab->nCol; i++){
      Column *pCol = &pTab->aCol[i];
      sqlite3_str_appendchar(s, 1, '{');
      kv_str(s, "name", pCol->zCnName);
      kv_str(s, "declared", sqlite3ColumnType(pCol, ""));
      kv_str(s, "affinity", affName(pCol->affinity));
      kv_bool(s, "notNull", pCol->notNull!=0);
      kv_bool(s, "pk", (pCol->colFlags & COLFLAG_PRIMKEY)!=0);
      kv_bool(s, "generated", (pCol->colFlags & COLFLAG_GENERATED)!=0);
      kv_bool(s, "hidden", (pCol->colFlags & COLFLAG_HIDDEN)!=0);
      closeObj(s); sqlite3_str_appendchar(s, 1, ',');
    }
    closeArr(s); sqlite3_str_appendchar(s, 1, ',');
    /* A unique key: every unique, non-partial index over plain columns, the INTEGER PRIMARY KEY, and the rowid itself. */
    key(s, "uniqueKeys"); sqlite3_str_appendchar(s, 1, '[');
    if( pTab->iPKey>=0 ){ sqlite3_str_appendchar(s, 1, '['); js(s, pTab->aCol[pTab->iPKey].zCnName); sqlite3_str_appendall(s, "],"); }
    else if( !(pTab->tabFlags & TF_WithoutRowid) && rowidName(pTab) ){ sqlite3_str_appendchar(s, 1, '['); js(s, rowidName(pTab)); sqlite3_str_appendall(s, "],"); }
    for(pIdx=pTab->pIndex; pIdx; pIdx=pIdx->pNext){
      int ok = pIdx->onError!=OE_None && pIdx->pPartIdxWhere==0 && !pIdx->bHasExpr;
      for(k=0; ok && k<pIdx->nKeyCol; k++) if( pIdx->aiColumn[k]<0 ) ok = 0;
      /* A comparison of the column uses the column's collation: an index under another one bounds
      ** it only when the column's is BINARY, which tells every value apart (NOCASE does not). */
      for(k=0; ok && k<pIdx->nKeyCol; k++){
        const char *zColl = sqlite3ColumnColl(&pTab->aCol[pIdx->aiColumn[k]]);
        const char *zIdxColl = pIdx->azColl[k];
        if( zColl==0 ) zColl = sqlite3StrBINARY;
        if( zIdxColl==0 ) zIdxColl = sqlite3StrBINARY;
        if( sqlite3StrICmp(zColl, zIdxColl)!=0 && sqlite3StrICmp(zColl, sqlite3StrBINARY)!=0 ) ok = 0;
      }
      if( !ok ) continue;
      sqlite3_str_appendchar(s, 1, '[');
      for(k=0; k<pIdx->nKeyCol; k++){ js(s, pTab->aCol[pIdx->aiColumn[k]].zCnName); sqlite3_str_appendchar(s, 1, ','); }
      closeArr(s); sqlite3_str_appendchar(s, 1, ',');
    }
    closeArr(s); sqlite3_str_appendchar(s, 1, ',');
    closeObj(s); sqlite3_str_appendchar(s, 1, ',');
  }
  closeObj(s);
}

/* ---- the schema replay and the entry point ------------------------------------------------- */

static void installHooks(int on){
  porg_select_hook = on ? selectHook : 0;
  porg_insert_hook = on ? insertHook : 0;
  porg_update_hook = on ? updateHook : 0;
  porg_delete_hook = on ? deleteHook : 0;
  porg_returning_hook = on ? returningHook : 0;
}

/* A host function's stand-in: the replay only prepares, so it is never called. */
static void hostFunctionStub(sqlite3_context *ctx, int argc, sqlite3_value **argv){
  (void)argc; (void)argv;
  sqlite3_result_null(ctx);
}

/* Replays the DDL into a fresh in-memory database; errors are reported, not fatal. The host's
** functions are registered first, so a view, an index or a CHECK that calls one replays too. */
static int openSchema(const char *const *azDdl, int nDdl, const char *const *azFn, const int *aFnArg, int nFn, sqlite3_str *errors){
  sqlite3_str *k = sqlite3_str_new(0);
  char *zKey;
  int i, rc;
  for(i=0; i<nFn; i++){ sqlite3_str_appendf(k, "%s/%d", azFn[i], aFnArg[i]); sqlite3_str_appendchar(k, 1, '\2'); }
  for(i=0; i<nDdl; i++){ sqlite3_str_appendall(k, azDdl[i]); sqlite3_str_appendchar(k, 1, '\1'); }
  zKey = sqlite3_str_finish(k);
  if( zKey==0 ) zKey = sqlite3_mprintf("");
  if( gDb && gDdlKey && strcmp(gDdlKey, zKey)==0 ){ sqlite3_free(zKey); return SQLITE_OK; }
  if( gDb ){ sqlite3_close(gDb); gDb = 0; }
  sqlite3_free(gDdlKey); gDdlKey = zKey;
  rc = sqlite3_open(":memory:", &gDb);
  if( rc!=SQLITE_OK ) return rc;
  installHooks(0);
  for(i=0; i<nFn; i++){
    /* Deterministic and innocuous: whatever the host's own flags, the replay accepts every place
    ** the host's schema could already call it from. */
    if( sqlite3_create_function_v2(gDb, azFn[i], aFnArg[i], SQLITE_UTF8|SQLITE_DETERMINISTIC|SQLITE_INNOCUOUS, 0, hostFunctionStub, 0, 0, 0)!=SQLITE_OK ){
      sqlite3_str_appendchar(errors, 1, '{');
      kv_str(errors, "function", azFn[i]);
      kv_str(errors, "message", sqlite3_errmsg(gDb));
      closeObj(errors); sqlite3_str_appendchar(errors, 1, ',');
    }
  }
  for(i=0; i<nDdl; i++){
    char *zErr = 0;
    if( sqlite3_exec(gDb, azDdl[i], 0, 0, &zErr)!=SQLITE_OK ){
      sqlite3_str_appendchar(errors, 1, '{');
      kv_int(errors, "index", i);
      kv_str(errors, "message", zErr);
      closeObj(errors); sqlite3_str_appendchar(errors, 1, ',');
    }
    sqlite3_free(zErr);
  }
  return SQLITE_OK;
}

/*
** Analyzes one statement against the schema DDL, with the host's scalar functions (azFn[i] taking
** aFnArg[i] arguments, -1 for any number) registered on the replay. Returns a JSON document (free
** with porg_sqlite_free): { statement, dml, returning, tables, columns, params, readonly, error,
** schemaErrors }.
*/
char *porg_sqlite_analyze(const char *const *azDdl, int nDdl, const char *zSql, const char *const *azFn, const int *aFnArg, int nFn){
  sqlite3_str *out = sqlite3_str_new(0);
  sqlite3_str *schemaErrors = sqlite3_str_new(0);
  sqlite3_stmt *pStmt = 0;
  const char *zTail = 0;
  int rc, i, n;
  sqlite3_str_appendchar(schemaErrors, 1, '[');
  rc = openSchema(azDdl, nDdl, azFn, aFnArg, nFn, schemaErrors);
  closeArr(schemaErrors);
  sqlite3_str_appendchar(out, 1, '{');
  if( rc!=SQLITE_OK ){
    key(out, "error"); sqlite3_str_appendall(out, "{\"message\":\"cannot open the schema replay\",\"offset\":-1}");
    closeObj(out);
    sqlite3_free(sqlite3_str_finish(schemaErrors));
    return sqlite3_str_finish(out);
  }
  gStmt = sqlite3_str_new(0); gDml = sqlite3_str_new(0); gReturning = sqlite3_str_new(0);
  gCaptured = 0; gWantSelect = 0; gnTables = 0;
  installHooks(1);
  rc = sqlite3_prepare_v2(gDb, zSql, -1, &pStmt, &zTail);
  installHooks(0);
  if( rc!=SQLITE_OK ){
    key(out, "error"); sqlite3_str_appendchar(out, 1, '{');
    kv_str(out, "message", sqlite3_errmsg(gDb));
    kv_int(out, "offset", sqlite3_error_offset(gDb));
    closeObj(out); sqlite3_str_appendchar(out, 1, ',');
  }else{
    key(out, "statement"); if( sqlite3_str_length(gStmt) ) sqlite3_str_appendall(out, sqlite3_str_value(gStmt)); else sqlite3_str_appendall(out, "null");
    sqlite3_str_appendchar(out, 1, ',');
    key(out, "dml"); if( sqlite3_str_length(gDml) ) sqlite3_str_appendall(out, sqlite3_str_value(gDml)); else sqlite3_str_appendall(out, "null");
    sqlite3_str_appendchar(out, 1, ',');
    key(out, "returning"); if( sqlite3_str_length(gReturning) ) sqlite3_str_appendall(out, sqlite3_str_value(gReturning)); else sqlite3_str_appendall(out, "null");
    sqlite3_str_appendchar(out, 1, ',');
    n = sqlite3_column_count(pStmt);
    key(out, "columns"); sqlite3_str_appendchar(out, 1, '[');
    for(i=0; i<n; i++){
      sqlite3_str_appendchar(out, 1, '{');
      kv_str(out, "name", sqlite3_column_name(pStmt, i));
      kv_str(out, "declared", sqlite3_column_decltype(pStmt, i));
      kv_str(out, "table", sqlite3_column_table_name(pStmt, i));
      kv_str(out, "column", sqlite3_column_origin_name(pStmt, i));
      closeObj(out); sqlite3_str_appendchar(out, 1, ',');
    }
    closeArr(out); sqlite3_str_appendchar(out, 1, ',');
    n = sqlite3_bind_parameter_count(pStmt);
    key(out, "params"); sqlite3_str_appendchar(out, 1, '[');
    for(i=1; i<=n; i++){
      sqlite3_str_appendchar(out, 1, '{');
      kv_int(out, "index", i);
      kv_str(out, "name", sqlite3_bind_parameter_name(pStmt, i));
      closeObj(out); sqlite3_str_appendchar(out, 1, ',');
    }
    closeArr(out); sqlite3_str_appendchar(out, 1, ',');
    kv_bool(out, "readonly", sqlite3_stmt_readonly(pStmt));
    kv_bool(out, "tail", zTail && zTail[0] && sqlite3_complete(zTail)==0 ? 1 : (zTail && strspn(zTail, " \t\r\n;")!=strlen(zTail)));
    key(out, "tables"); tables(out); sqlite3_str_appendchar(out, 1, ',');
  }
  key(out, "schemaErrors"); sqlite3_str_appendall(out, sqlite3_str_value(schemaErrors) ? sqlite3_str_value(schemaErrors) : "[]");
  sqlite3_str_appendchar(out, 1, ',');
  closeObj(out);
  sqlite3_finalize(pStmt);
  sqlite3_free(sqlite3_str_finish(gStmt)); sqlite3_free(sqlite3_str_finish(gDml)); sqlite3_free(sqlite3_str_finish(gReturning));
  sqlite3_free(sqlite3_str_finish(schemaErrors));
  gStmt = gDml = gReturning = 0;
  return sqlite3_str_finish(out);
}

void porg_sqlite_free(char *p){ sqlite3_free(p); }

#ifdef PORG_SQLITE_MAIN
/* porg_sqlite_check <database file> <statements file>: the DDL of the file, one JSON line per statement. */
int main(int argc, char **argv){
  sqlite3 *db; sqlite3_stmt *st; FILE *f; char line[8192];
  const char *azDdl[1024]; int nDdl = 0;
  if( argc<3 ){ fprintf(stderr, "usage: porg_sqlite_check <db> <statements>\n"); return 2; }
  if( sqlite3_open_v2(argv[1], &db, SQLITE_OPEN_READONLY, 0)!=SQLITE_OK ){ fprintf(stderr, "open: %s\n", sqlite3_errmsg(db)); return 1; }
  sqlite3_prepare_v2(db, "select sql from sqlite_schema where sql is not null order by rowid", -1, &st, 0);
  while( sqlite3_step(st)==SQLITE_ROW && nDdl<1024 ) azDdl[nDdl++] = strdup((const char*)sqlite3_column_text(st, 0));
  sqlite3_finalize(st); sqlite3_close(db);
  f = fopen(argv[2], "r");
  if( f==0 ){ fprintf(stderr, "cannot read %s\n", argv[2]); return 1; }
  while( fgets(line, sizeof line, f) ){
    char *nl = strchr(line, '\n'); if( nl ) *nl = 0;
    if( line[0]==0 || line[0]=='#' ) continue;
    char *json = porg_sqlite_analyze(azDdl, nDdl, line, 0, 0, 0);
    printf("%s\n", json);
    porg_sqlite_free(json);
  }
  return 0;
}
#endif
