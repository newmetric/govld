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

func say() string {
	return "foo"
}

type Foo struct {
	kkk int
	aaa a.Pointer
}

func (f Foo) privateMethod() string {
	return "asdf"
}

type emptyStruct struct {
}

type iface interface {
	A() string
	B() int
}
