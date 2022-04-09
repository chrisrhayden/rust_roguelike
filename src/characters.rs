use sdl2::rect::Rect;

const CHARS: &[char; 154] = &[
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
    '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', '░', '▒',
    '▓', '│', '─', '┼', '┤', '┴', '├', '┬', '└', '┌', '┐', '┘', '▀', ' ', ' ',
    ' ', ' ', ' ', ' ', '↑', '↓', '→', '←', '▲', '▼', '►', '◄', '↕', '↔', '☐',
    '☑', '○', '◙', '║', '═', '╬', '╣', '╩', '╠', '╦', '╚', '╔', '╗', '╝', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', ' ', ' ', ' ', ' ', ' ', ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z',
];

pub struct Characters {
    pub width: u32,
    pub height: u32,
    char_recs: Vec<Rect>,
}

impl Characters {
    pub fn new(width: u32, height: u32, column: usize) -> Self {
        let mut char_recs = Vec::new();

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut current_column: usize = 1;

        let rect = Rect::new(x as i32, y as i32, width, height);
        char_recs.push(rect);

        x += width + 1;
        current_column += 1;

        for c in CHARS.iter().skip(1) {
            if *c != ' ' {
                let rect = Rect::new(x as i32, y as i32, width, height);
                char_recs.push(rect);
            }

            if current_column == column {
                x = 0;
                y += height;

                current_column = 1;
            } else {
                x += width;
                current_column += 1;
            }
        }

        Self {
            width,
            height,
            char_recs,
        }
    }

    pub fn get_rect(&self, c: char) -> Rect {
        match c {
            ' ' => self.char_recs[0],
            '!' => self.char_recs[1],
            '"' => self.char_recs[2],
            '#' => self.char_recs[3],
            '$' => self.char_recs[4],
            '%' => self.char_recs[5],
            '&' => self.char_recs[6],
            '\'' => self.char_recs[7],
            '(' => self.char_recs[8],
            ')' => self.char_recs[9],
            '*' => self.char_recs[10],
            '+' => self.char_recs[11],
            ',' => self.char_recs[12],
            '-' => self.char_recs[13],
            '.' => self.char_recs[14],
            '/' => self.char_recs[15],
            '0' => self.char_recs[16],
            '1' => self.char_recs[17],
            '2' => self.char_recs[18],
            '3' => self.char_recs[19],
            '4' => self.char_recs[20],
            '5' => self.char_recs[21],
            '6' => self.char_recs[22],
            '7' => self.char_recs[23],
            '8' => self.char_recs[24],
            '9' => self.char_recs[25],
            ':' => self.char_recs[26],
            ';' => self.char_recs[27],
            '<' => self.char_recs[28],
            '=' => self.char_recs[29],
            '>' => self.char_recs[30],
            '?' => self.char_recs[31],
            '@' => self.char_recs[32],
            '[' => self.char_recs[33],
            '\\' => self.char_recs[34],
            ']' => self.char_recs[35],
            '^' => self.char_recs[36],
            '_' => self.char_recs[37],
            '`' => self.char_recs[38],
            '{' => self.char_recs[39],
            '|' => self.char_recs[40],
            '}' => self.char_recs[41],
            '~' => self.char_recs[42],
            '░' => self.char_recs[43],
            '▒' => self.char_recs[44],
            '▓' => self.char_recs[45],
            '│' => self.char_recs[46],
            '─' => self.char_recs[47],
            '┼' => self.char_recs[48],
            '┤' => self.char_recs[49],
            '┴' => self.char_recs[50],
            '├' => self.char_recs[51],
            '┬' => self.char_recs[52],
            '└' => self.char_recs[53],
            '┌' => self.char_recs[54],
            '┐' => self.char_recs[55],
            '┘' => self.char_recs[56],
            '▀' => self.char_recs[57],
            '↑' => self.char_recs[58],
            '↓' => self.char_recs[59],
            '→' => self.char_recs[60],
            '←' => self.char_recs[61],
            '▲' => self.char_recs[62],
            '▼' => self.char_recs[63],
            '►' => self.char_recs[64],
            '◄' => self.char_recs[65],
            '↕' => self.char_recs[66],
            '↔' => self.char_recs[67],
            '☐' => self.char_recs[68],
            '☑' => self.char_recs[69],
            '○' => self.char_recs[70],
            '◙' => self.char_recs[71],
            '║' => self.char_recs[72],
            '═' => self.char_recs[73],
            '╬' => self.char_recs[74],
            '╣' => self.char_recs[75],
            '╩' => self.char_recs[76],
            '╠' => self.char_recs[77],
            '╦' => self.char_recs[78],
            '╚' => self.char_recs[79],
            '╔' => self.char_recs[80],
            '╗' => self.char_recs[81],
            '╝' => self.char_recs[82],
            'A' => self.char_recs[83],
            'B' => self.char_recs[84],
            'C' => self.char_recs[85],
            'D' => self.char_recs[86],
            'E' => self.char_recs[87],
            'F' => self.char_recs[88],
            'G' => self.char_recs[89],
            'H' => self.char_recs[90],
            'I' => self.char_recs[91],
            'J' => self.char_recs[92],
            'K' => self.char_recs[93],
            'L' => self.char_recs[94],
            'M' => self.char_recs[95],
            'N' => self.char_recs[96],
            'O' => self.char_recs[97],
            'P' => self.char_recs[98],
            'Q' => self.char_recs[99],
            'R' => self.char_recs[100],
            'S' => self.char_recs[101],
            'T' => self.char_recs[102],
            'U' => self.char_recs[103],
            'V' => self.char_recs[104],
            'W' => self.char_recs[105],
            'X' => self.char_recs[106],
            'Y' => self.char_recs[107],
            'Z' => self.char_recs[108],
            'a' => self.char_recs[109],
            'b' => self.char_recs[110],
            'c' => self.char_recs[111],
            'd' => self.char_recs[112],
            'e' => self.char_recs[113],
            'f' => self.char_recs[114],
            'g' => self.char_recs[115],
            'h' => self.char_recs[116],
            'i' => self.char_recs[117],
            'j' => self.char_recs[118],
            'k' => self.char_recs[119],
            'l' => self.char_recs[120],
            'm' => self.char_recs[121],
            'n' => self.char_recs[122],
            'o' => self.char_recs[123],
            'p' => self.char_recs[124],
            'q' => self.char_recs[125],
            'r' => self.char_recs[126],
            's' => self.char_recs[127],
            't' => self.char_recs[128],
            'u' => self.char_recs[129],
            'v' => self.char_recs[130],
            'w' => self.char_recs[131],
            'x' => self.char_recs[132],
            'y' => self.char_recs[133],
            'z' => self.char_recs[134],
            _ => panic!("got a bad char {}", c),
        }
    }
}
