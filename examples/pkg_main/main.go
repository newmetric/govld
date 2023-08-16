package main

import (
	"fmt"
	"github.com/fake-organization/pkg_b"
)

func main() {
	fmt.Println("Hello", pkg_b.ExportedFunction())
}
