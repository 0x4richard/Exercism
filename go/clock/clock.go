package clock

import (
	"fmt"
	"math"
)

// Define the Clock type here.
type Clock struct {
	m int
}

const MINUTE_PER_HOUR = 60
const MINUTE_PER_DAY = MINUTE_PER_HOUR * 24

func New(h, m int) Clock {
	minutes := remEuclid(h*MINUTE_PER_HOUR+m, MINUTE_PER_DAY)

	return Clock{m: minutes}
}

func (c Clock) Add(m int) Clock {
	return New(0, c.m+m)
}

func (c Clock) Subtract(m int) Clock {
	return New(0, c.m-m)
}

func (c Clock) String() string {
	hours := c.m / MINUTE_PER_HOUR
	minutes := c.m % MINUTE_PER_HOUR
	return fmt.Sprintf("%02d:%02d", hours, minutes)
}

func remEuclid(a, b int) int {
	return a - b*int(math.Floor(float64(a)/float64(b)))
}
