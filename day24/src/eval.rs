// translated from input

#[allow(dead_code)]
pub fn program(inp: [i8; 14]) -> i64 {
    let mut z = i64::from(inp[0]) + 1;
    z = z * 26 + i64::from(inp[1]) + 7;
    z = z * 26 + i64::from(inp[2]) + 13;

    z = if z % 26 == i64::from(inp[3]) + 6 {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[3]) + 10
    };

    z = z * 26 + i64::from(inp[4]);

    z = if z % 26 == i64::from(inp[5]) + 4 {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[5]) + 13
    };

    z = z * 26 + i64::from(inp[6]) + 11;
    z = z * 26 + i64::from(inp[7]) + 6;
    z = z * 26 + i64::from(inp[8]) + 1;

    z = if z % 26 == i64::from(inp[9]) {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[9]) + 7
    };

    z = if z % 26 == i64::from(inp[10]) {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[10]) + 11
    };

    z = if z % 26 == i64::from(inp[11]) + 3 {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[11]) + 14
    };

    z = if z % 26 == i64::from(inp[12]) + 9 {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[12]) + 4
    };

    z = if z % 26 == i64::from(inp[13]) + 9 {
        z / 26
    } else {
        z - z % 26 + i64::from(inp[13]) + 10
    };

    // z will be zero iff all the z/26 conditions are executed

    z
}
