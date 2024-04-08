# newmetric/govld

'govld' is a code tool that adds/replaces code 'Go' according to a pre-written manifest files.
(Originally created only to modify files within the vendor folder, but now all files are targeted.)

## How to Install?

```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install govld
CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install govld --git https://github.com/newmetric/govld
```

## Usage

```bash
govld [-d=directory] -- [list_of_manifests.yaml]
```

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

  # replacing interface
  - pattern: interface_declaration
    code: |
      type iface interface {
        A() string
        B() int
        C() bool
      }
      
  # replacing global variable
  - pattern: variable_declaration
    code: |
      var NewlyAdded string

  # appending non-existent entry
  - pattern: interface_declaration
    code: |
      type iface_appended interface {
        A() string
        B() int
        C() bool
        Appended() uint64
      }
```

### Optional Manifest

You can make the manifest optional by declaring `optional: true`. Optional manifests won't fail even if the target patch file is not found.

```yaml
# sample patch manifest
file: github.com/fake-organization/pkg_xxx/internal/should_not_panic.go

# don't panic even if the file is NOT found
optional: true

# usual patch...
patch:
  # replacing a simple function
  - pattern: function_declaration
    code: |
      func say() string {
          return "World"
      }


```

### Patch Types

There are two types of patching available: `clone` and `overwrite`.

#### clone(default)

`clone` will generate the code with a list of patches, and append them to the end of the file.
Add the suffix to the original symbol name if it exists.
e.g. `func say() string` will be replaced as `func say__replaced_by_function_decl() string`

```yaml
# sample patch manifest
file: github.com/fake-organization/pkg_b/internal/module.go
patch:
  patch_type: clone
  ...
```

#### overwrite

`overwrite` is same as clone, but it will overwrite the original symbol if it exists.

```yaml
# sample patch manifest
file: github.com/fake-organization/pkg_b/internal/module.go
patch:
  patch_type: overwrite
  ...
```


### Postprocess

You can choose to run another batch of patches AFTER a manifest has been successfully processed. Simply append `postprocess` section in the manifest file.

Note that `postprocess` section is another `array` - allowing you to set different targets for postprocessing. the yaml schema is the same as normal patch schema.

```yaml

# run this AFTER all patches are done
postprocess:
- file: github.com/fake-organization/pkg_b/internal/postprocess_target.go
  patch:
    - pattern: method_declaration
      code: |
        func postprocessed() {}

# optional post process
- file: github.com/fake-organization/pkg_b/internal/postprocess_target_xx.go
  optional: true
  patch:
    - pattern: method_declaration
      code: |
        func postprocessed() {}
```



