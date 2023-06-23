pub fn verse(n: u32) -> String {
    let mut verse: String = String::new();
    let temp_verse: String;

    verse += match n {
        2 => {
                temp_verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, (n-1));
                &temp_verse
            },
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        _ => {
                temp_verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, (n-1));
                &temp_verse
            }
    };
    verse
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::new();

    for i in (end..=start).rev().step_by(1) {
        song.push_str(&verse(i));
        if i > end {
            song.push('\n');
        }
    }
    song
}
