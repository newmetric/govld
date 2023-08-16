module pkg_main

go 1.20

replace github.com/fake-organization/pkg_b => ./../pkg_b

require github.com/fake-organization/pkg_b v0.0.0-00010101000000-000000000000
