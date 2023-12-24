pub(crate) fn c_to_f(celcius: f32) -> f32 {
    let f: f32 = (9.0 / 5.0 * celcius) + 32.0;
    println!("{celcius} -> {f}");
    return f;
}

pub(crate) fn f_to_c(fahrenheit: f32) -> f32 {
    let c = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit} -> {c}");
    return c;
}

pub(crate) fn fibbonacci(num: u8) -> u32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        return fibbonacci(num - 1) + fibbonacci(num - 2);
    }
}

fn gen_lyrics(day: i8) -> String {
    let mut song: [&str; 12] = [
        "On the twelfth day of Christmas,my true love gave to me",
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
    ];
    song.reverse();
    let ordinal = match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    let mut start = format!(
        "On the {}{} day of Christmas my true love gave to me",
        day, ordinal
    );
    for i in 1..day as usize{
        start = format!("{}\n{}", start, song[i-1]);
    }
    if day != 1 {
        start = format!("{} {} ", start, "and");
    }
    start = format!("{}\n{}", start, "a partridge in a pear tree!");

    return start;
}
pub(crate) fn twelve_days_of_xmas() {
    for i in 1..13 {
        println!("{}\n", gen_lyrics(i))
    }
}
