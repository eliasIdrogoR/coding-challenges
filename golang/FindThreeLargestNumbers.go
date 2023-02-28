package main

import (
	"math"
	"sort"
)

func FindThreeLargestNumbers(array []int) []int {

	threeLargets := []int{math.MinInt32, math.MinInt32, math.MinInt32}

	for _, index := range array {

		if index > threeLargets[2] {
			threeLargets[0] = threeLargets[1]
			threeLargets[1] = threeLargets[2]
			threeLargets[2] = index
		} else if index > threeLargets[1] {
			threeLargets[0] = threeLargets[1]
			threeLargets[1] = index
		} else if index > threeLargets[0] {
			threeLargets[0] = index
		}
	}

	sort.Ints(threeLargets)
	return threeLargets
}
