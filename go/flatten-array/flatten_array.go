package flatten

func Flatten(nested interface{}) []interface{} {
	result := make([]interface{}, 0)

	switch item := nested.(type) {
	case []interface{}:
		for _, v := range item {
			result = append(result, Flatten(v)...)
		}
	case interface{}:
		result = append(result, item)
	}

	return result
}
