package main

import (
	"flag"
	"fmt"
	"io"
	"os"
)

var showBytes bool
var showNumOfLines bool
var showNumOfWords bool

func init() {
	flag.BoolVar(&showBytes, "c", false, "outputs the number of bytes")
	flag.BoolVar(&showNumOfLines, "l", false, "outputs the number of lines")
	flag.BoolVar(&showNumOfWords, "w", false, "outputs the number of words")
	flag.Parse()
}

func main() {
	if len(flag.Args()) == 0 {
		throwErr("A file path argument must be passed", nil)
	}
	file, err := os.Open(flag.Arg(0))

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
	fileName := stats.Name()

	output := ""

	if showBytes {
		fileSizeBytes := stats.Size()
		output += fmt.Sprintf("%d ", fileSizeBytes)
	}

	if showNumOfLines {
		fileNumOfLines := calculateNumOfLines(file)
		output += fmt.Sprintf("%d ", fileNumOfLines)
	}

	if showNumOfWords {
		fileNumOfWords := calculateNumOfWords(file)
		output += fmt.Sprintf("%d ", fileNumOfWords)
	}

	output += fmt.Sprintf("%v", fileName)

	fmt.Println(output)
}

func throwErr(errMsg string, err error) {
	fmt.Println(errMsg, err)
	os.Exit(1)
}

func calculateNumOfLines(file *os.File) int {
	numOfLines := 0
	content, _ := io.ReadAll(file)
	for _, value := range content {
		if value == '\n' {
			numOfLines += 1
		}
	}
	return numOfLines
}

func calculateNumOfWords(file *os.File) int {
	numOfWords := 0
	content, _ := io.ReadAll(file)

	for range string(content) {
		numOfWords += 1
	}
	return numOfWords
}
