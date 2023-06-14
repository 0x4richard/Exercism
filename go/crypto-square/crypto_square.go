package cryptosquare

import (
	"math"
	"regexp"
	"strings"
)

func Encode(pt string) string {
	str := normalize(pt)
	strLen := len(str)
	c, r := calculateColumnAndRow(strLen)

	return rebuildString(str, c, r, strLen)
}

func normalize(str string) string {
	r := regexp.MustCompile("[a-zA-Z0-9]")

	strArr := r.FindAllString(str, -1)
	normalStr := strings.Join(strArr, "")

	return strings.ToLower(normalStr)
}

func calculateColumnAndRow(strLen int) (col int, row int) {
	n := int(math.Sqrt(float64(strLen)))

	col, row = n, n

	for col*row < strLen {
		if col == row {
			col++
		} else {
			row = col
		}
	}

	return
}

func rebuildString(str string, col, row, strLen int) string {
	b := strings.Builder{}
	for i := 0; i < col; i++ {
		for j := 0; j < row; j++ {
			index := j*col + i
			if index >= strLen {
				b.WriteString(" ")
				break
			}
			b.WriteByte(str[index])
		}
		if i < col-1 {
			b.WriteString(" ")
		}
	}

	return b.String()
}
