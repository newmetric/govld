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
# manifest for function_declaration
file: github.com/fake-organization/pkg_b/internal/module.go
patch:
  - pattern: function_declaration
    patch: |
      func say() string {
          return "World"
      }

# manifest for method_declaration
TBD
```

