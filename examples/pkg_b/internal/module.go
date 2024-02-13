package internal

import (
	"fmt"

	a "unsafe"

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

type FooButDifferent struct{}

func (f FooButDifferent) privateMethod() string {
	return ",mmmm"
}

type uselessInterface interface {
	a()
}

type uselessStruct struct {
	a int
}

var uselessVariable int

func uselessFunction() {
	fmt.Println("useless log")
}

func (u uselessStruct) uselessMethod() {
	fmt.Println("useless log")
}
