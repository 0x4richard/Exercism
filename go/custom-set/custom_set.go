package stringset

import (
	"strings"
)

// Implement Set as a collection of unique string values.
//
// For Set.String, use '{' and '}', output elements as double-quoted strings
// safely escaped with Go syntax, and use a comma and a single space between
// elements. For example, a set with 2 elements, "a" and "b", should be formatted as {"a", "b"}.
// Format the empty set as {}.

// Define the Set type here.

type Set map[string]struct{}

func New() Set {
	return make(map[string]struct{})
}

func NewFromSlice(l []string) Set {
	set := New()
	for _, item := range l {
		set.Add(item)
	}

	return set
}

func (s Set) String() string {
	b := &strings.Builder{}
	b.WriteString("{")
	lastIndex := len(s) - 1
	index := 0
	for v := range s {
		b.WriteString("\"")
		b.WriteString(v)
		b.WriteString("\"")
		if lastIndex > index {
			b.WriteString(", ")
		}
		index++
	}
	b.WriteString("}")
	return b.String()
}

func (s Set) IsEmpty() bool {
	return len(s) <= 0
}

func (s Set) Has(elem string) bool {
	_, ok := s[elem]
	return ok
}

func (s Set) Add(elem string) {
	if s.Has(elem) {
		return
	}
	s[elem] = struct{}{}
}

func Subset(s1, s2 Set) bool {
	for v := range s1 {
		if !s2.Has(v) {
			return false
		}
	}

	return true
}

func Disjoint(s1, s2 Set) bool {
	for v := range s2 {
		if s1.Has(v) {
			return false
		}
	}

	return true
}

func Equal(s1, s2 Set) bool {
	if len(s1) != len(s2) {
		return false
	}

	for v := range s1 {
		if !s2.Has(v) {
			return false
		}
	}

	return true
}

func Intersection(s1, s2 Set) Set {
	newSet := New()
	for v := range s1 {
		if s2.Has(v) {
			newSet.Add(v)
		}
	}
	return newSet
}

func Difference(s1, s2 Set) Set {
	newSet := New()
	for v := range s1 {
		if !s2.Has(v) {
			newSet.Add(v)
		}
	}
	return newSet
}

func Union(s1, s2 Set) Set {
	newSet := New()

	for v := range s1 {
		newSet.Add(v)
	}

	for v := range s2 {
		newSet.Add(v)
	}
	return newSet
}
