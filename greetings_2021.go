package main

import (
	"fmt"
	"regexp"
	"sort"
	"strings"
)

func main() {
	names := []string{"world hello", "new happy year", "年 吉 大 牛"}
	var han = regexp.MustCompile("^[\u4e00-\u9fa5]$")
	for i := 0; i < len(names); i++ {
		temp := strings.Split(names[i], " ")
		if han.MatchString(temp[0]) {
			sort.Sort(sort.Reverse(sort.StringSlice(temp)))
		} else {
			sort.Strings(temp)
		}
		for j, k := range temp {
			temp[j] = strings.Title(k)
		}
		fmt.Println(temp)
	}
}
