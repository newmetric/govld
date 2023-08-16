package internal

import (
	a "unsafe"
)

import (
	b "unsafe"
)

var k a.Pointer
var m b.Pointer

func Say() string {
	return say()
}

func say__replaced_by_function_decl() string {
	return "foo"
}

type Foo struct {
}

func (f Foo) privateMethod() string {
	return "asdf"
}

// Patched by govld. DO NOT EDIT
import (
	aaa "github.com/fake-organization/pkg_a"
)
// Patched by govld. DO NOT EDIT
func say() string {
    return "World"
}