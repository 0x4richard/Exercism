package binarysearch

func SearchInts(list []int, key int) int {
	startIndex, endIndex := 0, len(list)-1

	return binarySearch(list, key, startIndex, endIndex)
}

func binarySearch(list []int, key int, startIndex, endIndex int) int {
	if endIndex < startIndex {
		return -1
	}

	middleIndex := (endIndex-startIndex)/2 + startIndex
	middleNum := list[middleIndex]

	switch true {
	case key == middleNum:
		return middleIndex
	case key > middleNum:
		return binarySearch(list, key, middleIndex+1, endIndex)
	case key < middleNum:
		return binarySearch(list, key, startIndex, middleIndex-1)
	default:
		return -1
	}
}
