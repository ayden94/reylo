/* injected with -include: what wasi-libc lacks but PostgreSQL's port layer references */
#ifndef WASI_EXTRA_H
#define WASI_EXTRA_H
#include <setjmp.h>
/* PG_TRY uses sigsetjmp; wasi has no signals, so plain setjmp is equivalent */
#define sigjmp_buf jmp_buf
#define sigsetjmp(buf, savemask) setjmp(buf)
#define siglongjmp(buf, val) longjmp(buf, val)
#ifndef EHOSTDOWN
#define EHOSTDOWN 9001
#endif
#endif
