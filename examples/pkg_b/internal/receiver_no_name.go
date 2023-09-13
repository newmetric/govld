package internal

type Receiver struct{}

func (Receiver) NoReceiverName(a int) uint64 { return 1 }

func (r Receiver) WithReceiverName(a int) uint64 { return 1 }

func (r *Receiver) WithPointerReceiverName(a int) uint64 { return 1 }
