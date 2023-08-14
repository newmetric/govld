package internal

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
