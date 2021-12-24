// translated from input

#[allow(dead_code)]
pub fn program(inp: [i8; 14]) -> i64 {
    let mut z = inp[0] as i64 + 1;
    z = z * 26 + inp[1] as i64 + 7;
    z = z * 26 + inp[2] as i64 + 13;

    z = if z % 26 == inp[3] as i64 + 6 {
        z / 26
    } else {
        z - z % 26 + inp[3] as i64 + 10
    };

    z = z * 26 + inp[4] as i64;

    z = if z % 26 == inp[5] as i64 + 4 {
        z / 26
    } else {
        z - z % 26 + inp[5] as i64 + 13
    };

    z = z * 26 + inp[6] as i64 + 11;
    z = z * 26 + inp[7] as i64 + 6;
    z = z * 26 + inp[8] as i64 + 1;

    z = if z % 26 == inp[9] as i64 {
        z / 26
    } else {
        z - z % 26 + inp[9] as i64 + 7
    };

    z = if z % 26 == inp[10] as i64 {
        z / 26
    } else {
        z - z % 26 + inp[10] as i64 + 11
    };

    z = if z % 26 == inp[11] as i64 + 3 {
        z / 26
    } else {
        z - z % 26 + inp[11] as i64 + 14
    };

    z = if z % 26 == inp[12] as i64 + 9 {
        z / 26
    } else {
        z - z % 26 + inp[12] as i64 + 4
    };

    z = if z % 26 == inp[13] as i64 + 9 {
        z / 26
    } else {
        z - z % 26 + inp[13] as i64 + 10
    };

    // z will be zero iff all the z/26 conditions are executed

    z
}
