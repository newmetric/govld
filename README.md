# newmetric/govld

`govld` is a custom linker (or kind of..) for Go, allowing patches without forking/replacing in the `go.mod`.

`govld` works by first running `go mod vendor` to bring all dependencies to local, then patching listed files in the manifests provided.

## Usage

```bash
govld [-f] [-v=vendor_directory] -- [list_of_manifests.yaml]
```

- `-f/--force (default=false)` runs `go mod vendor` forcibly before patching.

## Manifest File

Manifest file is a YAML file, containing a list of patches to be applied.

```yaml
# sample patch manifest
file: github.com/fake-organization/pkg_b/internal/module.go
patch:
  # replacing a simple function
  - pattern: function_declaration

    # optionally, you can also add certain imports
    # below will be rendered as:
    # import (
    #   aaa "github.com/fake-organization/pkg_a"
    # )
    imports:
      - alias: aaa
        path: github.com/fake-organization/pkg_a
    code: |
      func say() string {
          return "World"
      }
  # replacing a method bound to a struct
  - pattern: method_declaration
    code: |
      func (f Foo) privateMethod() string {
        return "Newmetric was here"
      }

  # replacing struct itself
  - pattern: struct_declaration
    code: |
      type Foo struct {
        kkk int
        aaa a.Pointer
        added a.ArbitraryType
      }

  # replacing an empty struct
  - pattern: struct_declaration
    code: |
      type emptyStruct struct {
        notEmptyAnymore string
      }
```

