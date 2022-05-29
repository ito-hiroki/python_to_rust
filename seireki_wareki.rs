fn main() {
    for year in 1926..=2026 {
        print!("西暦{}年 = ", year);
        if year < 1989 {
            let tmp = &(year - 1926 + 1).to_string();
            println!("昭和{}年", if tmp == "1" {"元"} else {tmp});
        } else if year < 2019 {
            let tmp = &(year - 1989 + 1).to_string();
            println!("平成{}年", if tmp == "1" {"元"} else {tmp});
        } else {
            let tmp = &(year - 2019 + 1).to_string();
            println!("令和{}年", if tmp == "1" {"元"} else {tmp});
        }
    }
}
