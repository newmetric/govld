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
}

func (f Foo) privateMethod() string {
	return "asdf"
}
