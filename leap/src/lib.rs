pub fn is_leap_year(year: i32) -> bool {
    if year%4 == 0 && year%400 == 0{
       return true;
    } else if year%4 == 0 && year%100 == 0{
        return false;
    } else if year%4 == 0{
        return true;
    }
    false
}
