
## rust code 

``` 
const BASE32_ALPHABET: &[u8] = b"9876543210mnbvcxzasdfghjklpoiuyt";
// ABCDEFGHIJKLMNOPQRSTUVWXYZ234567
const PADDING_CHAR: u8 = b'=';


struct Base32 {}

impl Base32 {
// 编码函数
fn encode(input: &[u8]) -> String {
let mut output = Vec::new();
let mut bit_buffer = 0u16;
let mut bits_left = 0u8;

        for &byte in input {
            bit_buffer = (bit_buffer << 8) | byte as u16;
            bits_left += 8;

            while bits_left >= 5 {
                bits_left -= 5;
                let index = ((bit_buffer >> bits_left) & 0x1F) as usize;
                output.push(BASE32_ALPHABET[index]);
            }
        }

        // 处理剩余的位
        if bits_left > 0 {
            let index = ((bit_buffer << (5 - bits_left)) & 0x1F) as usize;
            output.push(BASE32_ALPHABET[index]);
        }

        // 添加填充
        while output.len() % 8 != 0 {
            output.push(PADDING_CHAR);
        }

        String::from_utf8(output).unwrap()
    }

    // 解码函数
    fn decode(input: &str) -> Option<Vec<u8>> {
        let input = input.trim_end_matches('=').as_bytes();
        let mut output = Vec::new();
        let mut bit_buffer = 0u16;
        let mut bits_left = 0u8;

        for &c in input {
            let val = match c {
                b'm'..=b't' => c -b'a' + 11,
                b'0'=> 10 ,
                b'1' => 9 ,
                b'2' => 8 ,
                b'3' => 7,
                b'4' => 6 ,
                b'5' => 5 ,
                b'6' => 4 ,
                b'7' => 3 ,
                b'8' => 2 ,
                b'9' => 1 ,
                _ => return None,
            };

            bit_buffer = (bit_buffer << 5) | val as u16;
            bits_left += 5;

            while bits_left >= 8 {
                bits_left -= 8;
                output.push((bit_buffer >> bits_left) as u8);
            }
        }

        Some(output)
    }
}

fn main() {
// 使用示例
let original = "foo".as_bytes();
let encoded = Base32::encode(original);
println!("Encoded: {}", encoded);

    let raw_encoding_code = "b0zj5+++";
    if let Some(decoded) = Base32::decode(&raw_encoding_code) {
        println!("Decoded: {}", String::from_utf8_lossy(&decoded));
    }
}
```