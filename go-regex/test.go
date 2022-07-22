package main

import (
	"fmt"
	"regexp"
	"time"
)

const text = `John, please get that article on www.linkedin.com to me by 5:00PM on Jan 9th 2012. 4:00 would be ideal, actually. If you have any questions, You can reach me at(519)-236-2723 or get in touch with my associate at harold.smith@gmail.com.
All rights reserved. Printed in the United States of America. No part of this book may be used or reproduced in any manner whatsoever without written permission except in the case of brief quotations embodied in critical articles and reviews. For information address HarperCollins Publishers, 10 East 53rd Street, New York, NY 10022. His name is David. I met him and John last week. Gowtham Teja Kanneganti is a good student. I was born on Oct 4, 1995. My Indian mobile number is +91-7761975545. After coming to USA I got a new number +1-405-413-5255. I live on 1003 E Brooks St, Norman, Ok, 73071. I met  a child, who is playing with josh.
this is my IP: 102.23.5.1
My router is : 10.10.10.1
71.159.188.33
81.141.167.45
165.65.59.139
64.248.67.225

https://tech.gov.sg
http://www.google.com
www.faq.tech.gov.au

My email is harold@mail.com
brutewayne@protonmail.com

this is my IP: 102.23.5.1
My router is: 10.10.10.1
71.159.188.33
81.141.167.45
165.65.59.139
64.248.67.225

Base64 data
QVBJX1RPS0VO
UzNjcjN0UGFzc3dvcmQ=
U3VwM3JTM2NyZXRQQHNzd29yZA==`

const (
	IPv4Pattern = `(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)`
)

var (
	IPv4Regex = regexp.MustCompile(IPv4Pattern)
)

func findStringMatches(text string) {
	start := time.Now()
	for i := 1; i < 100000; i++ {
		IPv4Regex.MatchString(text)
	}
	elapsed := time.Since(start)
	fmt.Println("Finding string match process took : ", elapsed)
}

func replaceMatchedStrings(text string) {
	// replacedText := ""
	start := time.Now()
	for i := 1; i < 100000; i++ {
		IPv4Regex.ReplaceAllString(text, "ipv4 was here")
		// replacedText = IPv4Regex.ReplaceAllString(text, "ipv4 was here")
	}
	elapsed := time.Since(start)
	fmt.Println("Replacing string match process took : ", elapsed)
	// fmt.Print("\n" + replacedText)
}

func main() {
	findStringMatches(text)
	replaceMatchedStrings(text)
}
