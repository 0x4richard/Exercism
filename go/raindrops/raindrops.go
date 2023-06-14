package raindrops

import (
	"strconv"
	"strings"
)

func Convert(number int) string {
	factors := []int{3, 5, 7}
	factorsMap := map[int]string{
		3: "Pling",
		5: "Plang",
		7: "Plong",
	}
	builder := strings.Builder{}
	for _, n := range factors {
		if number%n == 0 {
			v := factorsMap[n]
			builder.WriteString(v)
		}
	}

	if builder.Len() <= 0 {
		builder.WriteString(strconv.Itoa(number))
	}

	return builder.String()
}
