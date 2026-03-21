package sample

import (
	"fmt"
	"strings"
)

func Run(value string) string {
    fmt.Println(value)
    return strings.TrimSpace(value)
}
