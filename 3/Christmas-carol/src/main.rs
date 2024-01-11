fn main() {
    let mut i: usize = 0;

    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

        while i < 12 {
            song(i, &lyrics, &days);

            i += 1;
        }
}

fn song(i: usize, lyrics: &[&str], days: &[&str]) {
    let mut n: usize = i;

    println!("On the {} day of Christmas, my true love sent to me", days[n]);

    while n >= 1{
        println!("{},", lyrics[n]);

        n -= 1;
    }

    println!("And a partridge in a pear tree.\n");

}

