extern crate regex;
extern crate lazy_static;

use regex::Regex;
use lazy_static::lazy_static;
use std::time::{Instant};


pub const DATA: &str = "John, please get that article on www.linkedin.com to me by 5:00PM on Jan 9th 2012. 4:00 would be ideal, actually. If you have any questions, You can reach me at(519)-236-2723 or get in touch with my associate at harold.smith@gmail.com.
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
U3VwM3JTM2NyZXRQQHNzd29yZA==";

pub const IPV4_PATTERN :&str = r#"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)"#;
// pub const IPV4_REGEX: Regex = Regex::new(IPV4_PATTERN).unwrap();

lazy_static! {
    static ref IPV4_REGEX: Regex = Regex::new(IPV4_PATTERN).unwrap();
}

fn find_string_matches(text_var :&str) {
    let start = Instant::now();
    for _i in 0..1000000 {
        IPV4_REGEX.captures(text_var);
    }
    let duration = start.elapsed();
    println!("Finding string match process took : {:?}", duration);
}

fn replace_matched_strings(text_var :&str){
    // let replaced_text = IPV4_REGEX.replace_all(text_var, "IPV4");
    let start = Instant::now();
    for _i in 0..1000000 {
        IPV4_REGEX.replace(text_var, "IPv4");
    }
    let duration = start.elapsed();
    println!("Replacing string match process took : {:?}", duration);
    // println!("{replaced_text}");
}

fn main() {
    find_string_matches(DATA);
    replace_matched_strings(DATA);
}
