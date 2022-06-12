use tinyserde::JsonParser;
use tinyserde::JsonValue;
use tinyserde::ParserError;

fn main() {
    let json_input = "
[
  	{
		\"foo\": null
	},
	{
		\"bar\": 123
	},
	{
		\"baz\": \"abcde\"
	},
	345,
	\"efgh\",
	null,
	false
]
".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let value: Result<JsonValue, ParserError> = parser.parse();
    match value {
        Ok(value) => println!("The parsed value is: {:?}", value),
        Err(_) => panic!("Could not parse JSON: \n {}", parser.input),
    };
}
