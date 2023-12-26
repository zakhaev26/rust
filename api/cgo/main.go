package main

import "C"

//export addTwoNums
func addTwoNums(arg C.int) C.int {
	return arg * 2
}

func main() {
	
}
