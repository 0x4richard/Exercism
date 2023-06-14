package etl

import "strings"

func Transform(in map[int][]string) map[string]int {
	newData := make(map[string]int)
	for score, strList := range in {
		for _, str := range strList {
			newStr := strings.ToLower(str)
			newData[newStr] = score
		}
	}

	return newData
}
