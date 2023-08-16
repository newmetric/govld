package internal

// Patched by govld. DO NOT EDIT
import (
	aaa "github.com/fake-organization/pkg_a"



)



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

type Foo__replaced_by_struct_decl struct {
	kkk int
	aaa a.Pointer
}

func (f Foo) privateMethod__replaced_by_method_decl() string {
	return "asdf"
}

type emptyStruct__replaced_by_struct_decl struct {
}

// Patched by govld. DO NOT EDIT
func say() string {
    return "World"
}

func (f Foo) privateMethod() string {
  return "Newmetric was here"
}

type Foo struct {
  kkk int
  aaa a.Pointer
  added a.ArbitraryType
}

type emptyStruct struct {
  notEmptyAnymore string
}
