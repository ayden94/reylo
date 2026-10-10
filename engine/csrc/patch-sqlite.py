#!/usr/bin/env python3
# Copies the SQLite amalgamation with five one-line hooks, each right after name resolution:
# in sqlite3Select, sqlite3Insert, sqlite3Update, sqlite3DeleteFrom and the RETURNING path.
# Every anchor must occur exactly once, so a SQLite upgrade that moves one fails loudly here.
import sys
src_path, out_path = sys.argv[1], sys.argv[2]
src = open(src_path).read()
hooks = [
    ("  sqlite3SelectPrep(pParse, p, 0);\n",
     "  sqlite3SelectPrep(pParse, p, 0);\n  if( reylo_select_hook ) reylo_select_hook(pParse, p, pDest);\n"),
    ('    sqlite3ErrorMsg(pParse, "%d values for %d columns", nColumn, pColumn->nId);\n    goto insert_cleanup;\n  }\n',
     '    sqlite3ErrorMsg(pParse, "%d values for %d columns", nColumn, pColumn->nId);\n    goto insert_cleanup;\n  }\n  if( reylo_insert_hook ) reylo_insert_hook(pParse, pTab, pColumn, pList, pSelect, onError, pUpsert);\n'),
    ("  if( nChangeFrom==0 && sqlite3ResolveExprNames(&sNC, pWhere) ){\n    goto update_cleanup;\n  }\n",
     "  if( nChangeFrom==0 && sqlite3ResolveExprNames(&sNC, pWhere) ){\n    goto update_cleanup;\n  }\n  if( reylo_update_hook ) reylo_update_hook(pParse, pTab, pChanges, pWhere, aXRef, pTabList, onError, pUpsert);\n"),
    ("  if( sqlite3ResolveExprNames(&sNC, pWhere) ){\n    goto delete_from_cleanup;\n  }\n",
     "  if( sqlite3ResolveExprNames(&sNC, pWhere) ){\n    goto delete_from_cleanup;\n  }\n  if( reylo_delete_hook ) reylo_delete_hook(pParse, pTab, pWhere);\n"),
    ("    if( sqlite3ResolveExprListNames(&sNC, pNew)==SQLITE_OK\n     && ALWAYS(!db->mallocFailed)\n    ){\n",
     "    if( sqlite3ResolveExprListNames(&sNC, pNew)==SQLITE_OK\n     && ALWAYS(!db->mallocFailed)\n    ){\n      if( reylo_returning_hook ) reylo_returning_hook(pParse, pTab, pNew);\n"),
]
for old, new in hooks:
    n = src.count(old)
    if n != 1:
        sys.exit(f"patch-sqlite: anchor occurs {n} times, expected 1: {old[:70]!r}")
    src = src.replace(old, new)
open(out_path, "w").write(src)
print(f"patch-sqlite: 5 hooks -> {out_path}")
