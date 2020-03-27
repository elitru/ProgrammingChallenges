package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"time"
)

func main() {
	num := Input()

	start := time.Now()
	result := GetAllSubSets(num)
	elapsed := time.Now().Sub(start).Microseconds()

	fmt.Println(result)
	fmt.Printf("Time elapsed: %d microseconds! (%d milliseconds)\n", elapsed, (elapsed / int64(1000)))
}

func Input() int {
	fmt.Println("|==========================================================================|")
	fmt.Println("|============================= Subset-Finder ==============================|")
	fmt.Println("|==========================================================================|")
	fmt.Println("Enter the number for which the subsets will be calculated:")

	correct := false
	var num int

	for !correct {
		in := bufio.NewScanner(os.Stdin)
		in.Scan()
		input, err := strconv.Atoi(in.Text())
		HandleError(&err, &correct)
		num = input
	}

	return num
}

func HandleError(err *error, correct *bool) {
	if (*err) != nil {
		fmt.Println("Error occured! Seems that your input was not correct!")
		(*correct) = false
		return
	}

	(*correct) = true
}

func GetAllSubSets(n int) [][]int {
	numbers := GetRange(1, n)

	var result [][][]int
	for _, num := range numbers {
		result = append(result, GetSubsetForNumber(num))
	}

	var resultVec [][]int
	for _, item := range result {
		for _, itemDeeper := range item {
			resultVec = append(resultVec, itemDeeper)
		}
	}

	return resultVec
}

func GetSubsetForNumber(n int) [][]int {
	var results [][]int
	numbers := []int{n}
	target := n + 1
	targetCount := -1

	for !RowComplete(n, numbers) {
		sort.Ints(numbers)
		results = append(results, numbers)

		if target == GetMin(numbers)+1 || target == 1 {
			numbers = append(numbers, n-1)
			target = n - 1
			targetCount++
		} else {
			numbers = Remove(numbers, targetCount)
			target--
			numbers = append(numbers, target)
		}
	}

	sort.Ints(numbers)
	results = append(results, numbers)

	fmt.Println(results)

	return results
}

func RowComplete(n int, vec []int) bool {
	return (len(vec) == n)
}

func GetMin(items []int) int {
	min := items[0]
	for i := 1; i < len(items); i++ {
		if items[i] < min {
			min = items[i]
		}
	}

	return min
}

func GetRange(start, end int) []int {
	values := make([]int, (end + 1 - start))
	counter := 0

	for i := start; i <= end; i++ {
		values[counter] = i
		counter++
	}

	return values
}

func Remove(slice []int, s int) []int {
	return append(slice[:s], slice[s+1:]...)
}
