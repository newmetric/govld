package pkg_b

import (
	"github.com/fake-organization/pkg_b/internal"
)

func ExportedFunction() string {
	return internal.Say()
}
