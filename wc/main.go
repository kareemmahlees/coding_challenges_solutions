package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
)

var showBytes bool
var showNumOfLines bool
var showNumOfWords bool
var showNumOfChars bool

func init() {
	flag.BoolVar(&showBytes, "c", false, "outputs the number of bytes")
	flag.BoolVar(&showNumOfLines, "l", false, "outputs the number of lines")
	flag.BoolVar(&showNumOfWords, "w", false, "outputs the number of words")
	flag.BoolVar(&showNumOfChars, "m", false, "outputs the number of characters")
	flag.Parse()
}

func main() {
	var filePath string
	if len(flag.Args()) == 0 {
		fmt.Scan(&filePath)
		// throwErr("A file path argument must be passed", nil)
	} else {
		filePath = flag.Arg(0)
	}
	file, err := os.Open(filePath)

	// make sure to close the file at the end
	defer func() {
		if err = file.Close(); err != nil {
			throwErr("Error while closing file: ", err)
		}
	}()

	if err != nil {
		throwErr("Error opening the specified file: ", err)
	}

	stats, _ := file.Stat()

	output := ""

	switch {
	case showBytes:
		output += fmt.Sprintf("%d ", stats.Size())
		fallthrough

	case showNumOfLines:
		output += fmt.Sprintf("%d ", calculateNumOfLines(file))
		fallthrough

	case showNumOfWords:
		output += fmt.Sprintf("%d ", calculateNumOfWords(file))
		fallthrough

	case showNumOfChars:
		output += fmt.Sprintf("%d ", calculateNumOfChars(file))
	default:
		output += fmt.Sprintf("%d %d %d %d ", stats.Size(), calculateNumOfLines(file), calculateNumOfWords(file), calculateNumOfChars(file))
	}

	output += fmt.Sprintf("%v", stats.Name())

	fmt.Println(output)
}

func throwErr(errMsg string, err error) {
	fmt.Println(errMsg, err)
	os.Exit(1)
}

func calculateNumOfLines(file *os.File) int {
	defer file.Seek(0, io.SeekStart)

	numOfLines := 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		numOfLines++
	}
	return numOfLines
}

func calculateNumOfWords(file *os.File) int {
	defer file.Seek(0, io.SeekStart)

	numOfWords := 0
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanWords)

	for scanner.Scan() {
		numOfWords++
	}
	return numOfWords
}

func calculateNumOfChars(file *os.File) int {
	numOfChars := 0

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanRunes)

	for scanner.Scan() {
		numOfChars++
	}
	return numOfChars
}
