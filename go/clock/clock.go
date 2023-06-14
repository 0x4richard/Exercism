package clock

import (
	"fmt"
	"strconv"
	"time"
)

// Define the Clock type here.
type Clock struct {
	h int
	m int
}

func New(h, m int) Clock {
	t := newTime(h, m)
	return Clock{h: t.Hour(), m: t.Minute()}
}

func (c Clock) Add(m int) Clock {
	duration := time.Duration(m) * time.Minute
	t := c.time().Add(duration)
	return New(t.Hour(), t.Minute())
}

func (c Clock) Subtract(m int) Clock {
	duration := time.Duration(m) * time.Minute
	t := c.time().Add(-duration)
	return New(t.Hour(), t.Minute())
}

func (c Clock) String() string {
	h, m, _ := c.time().Clock()
	return fmt.Sprintf("%02d:%02d", h, m)
}

func (c Clock) time() time.Time {
	return newTime(c.h, c.m)
}

func newTime(h, m int) time.Time {
	t := time.Date(0, 0, 0, 0, 0, 0, 0, time.UTC)
	durationFromHour := time.Duration(h) * time.Hour
	durationFromMinute := time.Duration(m) * time.Minute
	t = t.Add(durationFromHour).Add(durationFromMinute)
	return t
}

func convertDurationToString(d int) string {
	if d < 10 {
		return fmt.Sprintf("0%d", d)
	}
	return strconv.Itoa(d)
}
