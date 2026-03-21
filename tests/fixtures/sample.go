package sample

import "fmt"

type Config struct{}

func CalculateTotal(items []int) int {
    sum := 0
    for _, item := range items {
        sum += item
    }
    return sum
}

func (c *Config) Run() {
    fmt.Println("run")
}
