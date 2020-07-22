package main

import (
	"C"

	"encoding/json"

	"github.com/hashicorp/hcl"
)

func main() {}

//export HCLtoJSON
func HCLtoJSON(inHCL string) *C.char {
	var v interface{}
	if err := hcl.Unmarshal([]byte(inHCL), &v); err != nil {
		return C.CString("error: " + err.Error())
	}
	outJSON, err := json.Marshal(v)
	if err != nil {
		return C.CString("error: " + err.Error())
	}
	return C.CString(string(outJSON))
}
