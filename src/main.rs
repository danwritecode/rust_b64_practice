use std::{collections::HashMap};

fn main() {
    let foo = "This is a test of my encoding and decoding".to_string();
    let foo_b64 = b64_encode_string(foo); 

    dbg!(&foo_b64);
    dbg!(b64_decode_string(foo_b64));
}


fn b64_encode_string(input_string: String) -> String {
    let b64_encode_table: HashMap<u32, String> = HashMap::from([
        (0, 'A'.to_string()),
        (1, 'B'.to_string()),
        (2, 'C'.to_string()),
        (3, 'D'.to_string()),
        (4, 'E'.to_string()),
        (5, 'F'.to_string()),
        (6, 'G'.to_string()),
        (7, 'H'.to_string()),
        (8, 'I'.to_string()),
        (9, 'J'.to_string()),
        (10, 'K'.to_string()),
        (11, 'L'.to_string()),
        (12, 'M'.to_string()),
        (13, 'N'.to_string()),
        (14, 'O'.to_string()),
        (15, 'P'.to_string()),
        (16, 'Q'.to_string()),
        (17, 'R'.to_string()),
        (18, 'S'.to_string()),
        (19, 'T'.to_string()),
        (20, 'U'.to_string()),
        (21, 'V'.to_string()),
        (22, 'W'.to_string()),
        (23, 'X'.to_string()),
        (24, 'Y'.to_string()),
        (25, 'Z'.to_string()),
        (26, 'a'.to_string()),
        (27, 'b'.to_string()),
        (28, 'c'.to_string()),
        (29, 'd'.to_string()),
        (30, 'e'.to_string()),
        (31, 'f'.to_string()),
        (32, 'g'.to_string()),
        (33, 'h'.to_string()),
        (34, 'i'.to_string()),
        (35, 'j'.to_string()),
        (36, 'k'.to_string()),
        (37, 'l'.to_string()),
        (38, 'm'.to_string()),
        (39, 'n'.to_string()),
        (40, 'o'.to_string()),
        (41, 'p'.to_string()),
        (42, 'q'.to_string()),
        (43, 'r'.to_string()),
        (44, 's'.to_string()),
        (45, 't'.to_string()),
        (46, 'u'.to_string()),
        (47, 'v'.to_string()),
        (48, 'w'.to_string()),
        (49, 'x'.to_string()),
        (50, 'y'.to_string()),
        (51, 'z'.to_string()),
        (52, '0'.to_string()),
        (53, '1'.to_string()),
        (54, '2'.to_string()),
        (55, '3'.to_string()),
        (56, '4'.to_string()),
        (57, '5'.to_string()),
        (58, '6'.to_string()),
        (59, '7'.to_string()),
        (60, '8'.to_string()),
        (61, '9'.to_string()),
        (62, '+'.to_string()),
        (63, '/'.to_string()),
    ]);
    
    // this takes 'And' as and outputs:
    // [
    //     '010000',
    //     '010110',
    //     '111001',
    //     '100100',
    // ]
    let six_bit_chunks: Vec<String> = input_string
        .as_bytes()
        .iter()
        .flat_map(|b| format!("{:08b}", b).chars().collect::<Vec<char>>())
        .collect::<Vec<char>>()
        .chunks(6)
        .map(|c| c.iter().collect::<String>())
        .collect();

    // For 'And' this outputs: QW5k
    let base_64_string = six_bit_chunks
        .chunks(4)
        .flat_map(|chunk| {
            chunk.iter()
            .map(|c| {
                let dec_val = u32::from_str_radix(c, 2).unwrap();
                return b64_encode_table.get(&dec_val).unwrap().clone();
            })
            .collect::<Vec<String>>()
        })
        .collect::<String>();

    return base_64_string;
}


fn b64_decode_string(b64_string: String) -> String {
    let b64_decode_table:HashMap<char, u32> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3),
        ('E', 4),
        ('F', 5),
        ('G', 6),
        ('H', 7),
        ('I', 8),
        ('J', 9),
        ('K', 10),
        ('L', 11),
        ('M', 12),
        ('N', 13),
        ('O', 14),
        ('P', 15),
        ('Q', 16),
        ('R', 17),
        ('S', 18),
        ('T', 19),
        ('U', 20),
        ('V', 21),
        ('W', 22),
        ('X', 23),
        ('Y', 24),
        ('Z', 25),
        ('a', 26),
        ('b', 27),
        ('c', 28),
        ('d', 29),
        ('e', 30),
        ('f', 31),
        ('g', 32),
        ('h', 33),
        ('i', 34),
        ('j', 35),
        ('k', 36),
        ('l', 37),
        ('m', 38),
        ('n', 39),
        ('o', 40),
        ('p', 41),
        ('q', 42),
        ('r', 43),
        ('s', 44),
        ('t', 45),
        ('u', 46),
        ('v', 47),
        ('w', 48),
        ('x', 49),
        ('y', 50),
        ('z', 51),
        ('0', 52),
        ('1', 53),
        ('2', 54),
        ('3', 55),
        ('4', 56),
        ('5', 57),
        ('6', 58),
        ('7', 59),
        ('8', 60),
        ('9', 61),
        ('+', 62),
        ('/', 63),
    ]);

    let decoded = b64_string
        .chars()
        .map(|c| format!("{:06b}", b64_decode_table.get(&c).unwrap().clone()))
        .collect::<Vec<String>>()
        .iter()
        .flat_map(|bin| bin.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chnk| chnk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|bin| {
            let dec_val = u32::from_str_radix(bin, 2).unwrap();
            return char::from_u32(dec_val).unwrap();
        })
        .collect::<String>();

    return decoded;
}


