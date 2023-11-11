package moetokengo

//go:generate bash -c "cd moetokenrs && cargo build --release"

/*
#cgo linux LDFLAGS: ${SRCDIR}/moetokenrs/target/release/libmoetoken.a -ldl 
#cgo darwin LDFLAGS: ${SRCDIR}/moetokenrs/target/release/libmoetoken.a -framework Security -framework CoreFoundation

#include <stdlib.h>
extern char *ohayou_oniichan(const char*);
extern unsigned int get_qtd_tokens(const char*, const char*);
*/ 
import "C"
import (
	"unsafe"
)

func OhayouOniichan(name string) string {
	n := C.CString(name)
	result := C.ohayou_oniichan(n)
	goString := C.GoString(result)

	C.free(unsafe.Pointer(n))
	C.free(unsafe.Pointer(result))

	return goString
}

func CountTokens(model_name, txt string) int {
	t := C.CString(txt)
	m := C.CString(model_name)

	count := C.get_qtd_tokens(m, t)

	C.free(unsafe.Pointer(t))
	C.free(unsafe.Pointer(m))
	return int(count)
}
