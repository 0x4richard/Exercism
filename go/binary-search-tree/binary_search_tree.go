package binarysearchtree

type BinarySearchTree struct {
	left  *BinarySearchTree
	data  int
	right *BinarySearchTree
}

// NewBst creates and returns a new BinarySearchTree.
func NewBst(i int) *BinarySearchTree {
	return &BinarySearchTree{nil, i, nil}
}

// Insert inserts an int into the BinarySearchTree.
// Inserts happen based on the rules of a binary search tree
func (bst *BinarySearchTree) Insert(i int) {
	if i <= bst.data {
		if bst.left != nil {
			bst.left.Insert(i)
		} else {
			bst.left = &BinarySearchTree{nil, i, nil}
		}
		return
	}

	if bst.right != nil {
		bst.right.Insert(i)
	} else {
		bst.right = &BinarySearchTree{nil, i, nil}
	}
}

// SortedData returns the ordered contents of BinarySearchTree as an []int.
// The values are in increasing order starting with the lowest int value.
// A BinarySearchTree that has the numbers [1,3,7,5] added will return the
// []int [1,3,5,7].
func (bst *BinarySearchTree) SortedData() []int {
	sortedData := make([]int, 0)
	if bst.left != nil {
		leftData := bst.left.SortedData()
		sortedData = append(sortedData, leftData[:]...)
	}

	sortedData = append(sortedData, bst.data)

	if bst.right != nil {
		rightData := bst.right.SortedData()
		sortedData = append(sortedData, rightData[:]...)
	}

	return sortedData
}
