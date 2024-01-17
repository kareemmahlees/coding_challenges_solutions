package main

import (
	"bufio"
	"bytes"
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
	var reader io.ReadSeeker
	var fileName string

	if len(flag.Args()) == 0 {
		b, _ := io.ReadAll(os.Stdin)
		reader = bytes.NewReader(b)
	} else {
		filePath := flag.Arg(0)
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

		reader = file
		fileStat, _ := file.Stat()
		fileName = fileStat.Name()
	}

	output := ""

	if showBytes {
		output += fmt.Sprintf("%d ", calculateSize(reader))
	}

	if showNumOfLines {
		output += fmt.Sprintf("%d ", calculateNumOfLines(reader))
	}

	if showNumOfWords {
		output += fmt.Sprintf("%d ", calculateNumOfWords(reader))
	}

	if showNumOfChars {
		output += fmt.Sprintf("%d ", calculateNumOfChars(reader))
	}

	if !showBytes && !showNumOfLines && !showNumOfWords && !showNumOfChars {
		output += fmt.Sprintf("%d %d %d ", calculateSize(reader), calculateNumOfLines(reader), calculateNumOfWords(reader))
	}

	if fileName != "" {
		output += fmt.Sprintf("%v", fileName)
	}

	fmt.Println(output)
}

func throwErr(errMsg string, err error) {
	fmt.Println(errMsg, err)
	os.Exit(1)
}

func calculateSize(reader io.ReadSeeker) int64 {
	defer reader.Seek(0, io.SeekStart)

	buf := &bytes.Buffer{}
	nRead, _ := io.Copy(buf, reader)

	return nRead
}

func calculateNumOfLines(reader io.ReadSeeker) int {
	defer reader.Seek(0, io.SeekStart)

	numOfLines := 0
	scanner := bufio.NewScanner(reader)

	for scanner.Scan() {
		numOfLines++
	}
	return numOfLines
}

func calculateNumOfWords(reader io.ReadSeeker) int {
	defer reader.Seek(0, io.SeekStart)

	numOfWords := 0
	scanner := bufio.NewScanner(reader)
	scanner.Split(bufio.ScanWords)

	for scanner.Scan() {
		numOfWords++
	}
	return numOfWords
}

func calculateNumOfChars(reader io.ReadSeeker) int {
	defer reader.Seek(0, io.SeekStart)

	numOfChars := 0
	scanner := bufio.NewScanner(reader)
	scanner.Split(bufio.ScanRunes)

	for scanner.Scan() {
		numOfChars++
	}
	return numOfChars
}
