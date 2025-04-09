// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

mod interpreter;

use std::sync::atomic::{AtomicUsize, Ordering};

// Define a static atomic counter for thread-safe incrementing
// used to make temps
static TEMP_COUNT: AtomicUsize = AtomicUsize::new(0);

static mut func_table: Vec<String> = vec![];


static mut WhileCount: i32 = 0; //Global Variable checking for while loops
static mut ifCount: i32 = 0;
static mut elseCount: i32 = 0;

fn main() {
/*
    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {
        code
    } 

    };

    let tokens = match lex(&code) {
    Err(error_message) => {
        println!("**Error**");
        println!("----------------------");
        println!("{}", error_message);
        println!("----------------------");
        return;
    }

    Ok(data) => data,
    
    };

    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

    Ok(()) => {
        println!("Program Parsed Successfully.");
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Error: {message}");
            println!("----------------------");
        }
    }

    }
}


    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }
      */

  // get commandline arguments.
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
      println!("Please provide an input file.");
      return;
  }

  if args.len() > 2 {
      println!("Too many commandline arguments.");
      return;
  }

  // read the entire file.
  let filename = &args[1];
  let result = fs::read_to_string(filename);
  let code = match result {
  Err(error) => {
      println!("**Error. File \"{}\": {}", filename, error);
      return;
  }

  Ok(code) => {
    code
  } 

  };

  let tokens = match lex(&code) {
  Err(error_message) => {
      println!("**Error**");
      println!("----------------------");
      println!("{}", error_message);
      println!("----------------------");
      return;
  }

  Ok(tokens) => tokens,
  
  };

  let mut index: usize = 0;
  match parse_program(&tokens, &mut index) {

  Ok(generated_code) => {
      println!("Program Parsed Successfully.");

      //might be wrong
      //let tokens = lex(code)?; 
      //let generated_code: String = parse(tokens)?;


      //println!("{generated_code}");
      interpreter::execute_ir(&generated_code);
  }

  Err(message) => {
      println!("**Error**");
      println!("----------------------");
      if tokens.len() == 0 {
          println!("No code has been provided.");
      } else {
          println!("Error: {message}");
          println!("----------------------");
      }
  }

  }
}


//=========================================================================================================================================//

// PHASE 1: LEXER

// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
enum Token {
  Plus,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Assign,
  Num(i32),
  Ident(String),
  If,
  While,
  Read, 
  Func,
  Return,
  Int,
  End,
  Print,
  Else,
  Break,
  Continue,
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  LeftBracket,
  RightBracket,
  Comma,
  SemiColon,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equality,
  NotEqual
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }

// This is a lexer

fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;
    let mut lookahead = '0';
    if i + 1 < bytes.len(){
      lookahead = bytes[i + 1] as char;
    }

    match c {
    
    //For numbers

    '0'..='9' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit >= '0' && digit <= '9' {
          i += 1;
        } 
        else if (digit >= 'a' && digit <= 'z') | (digit >= 'A' && digit <= 'Z') | (digit == '_')
        {
          let end = i;
          let string_token = &code[start..end];
          let word = string_token;
          return Err(format!("Lexer Error: Invalid identifier '{}'", word));
        }
        else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }

    //For Comments
    '#' => { //MAYBE BREAK IS WRONG
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit == '\n' {
          break;
        } 
        if i > bytes.len()
        {
          break;
        }
        i += 1;
      }

      i += 1;
    }

    //For Symbols (EXCEPT ASSIGN)

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }

    '-' => {
      tokens.push(Token::Subtract);
      i += 1;
    }

    '*' => {
      tokens.push(Token::Multiply);
      i += 1;
    }

    '/' => {
      tokens.push(Token::Divide);
      i += 1;
    }

    '%' => {
      tokens.push(Token::Modulus);
      i += 1;
    }

    ',' => {
      tokens.push(Token::Comma);
      i += 1;
    }

    ';' => {
      tokens.push(Token::SemiColon);
      i += 1;
    }

    '[' => {
      tokens.push(Token::LeftBracket);
      i += 1;
    }

    ']' => {
      tokens.push(Token::RightBracket);
      i += 1;
    }

    '(' => {
      tokens.push(Token::LeftParen);
      i += 1;
    }

    ')' => {
      tokens.push(Token::RightParen);
      i += 1;
    }

    '{' => {
      tokens.push(Token::LeftCurly);
      i += 1;
    }

    '}' => {
      tokens.push(Token::RightCurly);
      i += 1;
    }

    '}' => {
      tokens.push(Token::RightBracket);
      i += 1;
    }

    //Equalities
    '>' => {
      let lookahead = bytes[i + 1] as char;
      if lookahead == '='
      {
          tokens.push(Token::GreaterEqual);
          i += 2;
      }
      else
      {
        tokens.push(Token::Greater);
        i += 1;
      }
    }

    '<' => {
      //let lookahead = bytes[i + 1] as char;
      if lookahead == '='
      {
          tokens.push(Token::LessEqual);
          i += 2;
      }
      else
      {
        tokens.push(Token::Less);
        i += 1;
      }
    }

    '=' => {
      //let lookahead = bytes[i + 1] as char;
      if lookahead == '='
      {
          tokens.push(Token::Equality);
          i += 2;
      }
      else
      {
        tokens.push(Token::Assign);
        i += 1;
      }
    }

    '!' => {
      //let lookahead = bytes[i + 1] as char;
      if lookahead == '='
      {
          tokens.push(Token::NotEqual);
          i += 2;
      }
      else
      {
        return Err(format!("Lexer Error: Unrecognized symbol '{}'", c));
      }
    }

    //All Letters / Words

    'a'..='z' | 'A'..='Z' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let letter = bytes[i] as char;
        if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || (letter == '_') {
          i += 1;
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let word = string_token;

      if word == "func"
      {
        tokens.push(Token::Func);
      }

      else if word == "return"
      {
        tokens.push(Token::Return);
      }

      else if word == "int"
      {
        tokens.push(Token::Int);
      }

      else if word == "print"
      {
        tokens.push(Token::Print);
      }

      else if word == "read"
      {
        tokens.push(Token::Read);
      }

      else if word == "while"
      {
        tokens.push(Token::While);
      }

      else if word == "if"
      {
        tokens.push(Token::If);
      }

      else if word == "else"
      {
        tokens.push(Token::Else);
        unsafe {elseCount += 1;}
      }

      else if word == "break"
      {
        tokens.push(Token::Break);
      }

      else if word == "continue"
      {
        tokens.push(Token::Continue);
      }

      else
      {
        let token = Token::Ident(word.to_string());
        tokens.push(token);
      }
    }

    ' ' | '\n' => {
      //let lookahead = bytes[i + 1] as char;
      if lookahead == '_' {
        return Err(format!("Lexer Error: Unrecognized symbol '{}'", c));
      }
      else{
        i += 1;
      }

    }

    '"' | ('\0'..='\t') | '\u{b}'..='\u{1f}' | '$' | '&'..='\\' | '.' | '^'..='`'| '|' | '`'|'~'..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => {
      i += 1;
    }
  }
}

  tokens.push(Token::End);
  return Ok(tokens);

}

//========================================================================================================================================//

// PHASE 2: PARSER

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
  let mut ir_code: String = String::from(""); 
  let mut symbol_table: Vec<(String, String)> = vec![];

  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  while !at_end(tokens, *index) {
    match parse_function(tokens, index, &mut symbol_table) {
    Ok(function_ir_code) => {
      ir_code += &function_ir_code;
    }
    Err(e) => { return Err(e); }
    }
  }

  let main = format!("main");
  if !find_func(unsafe{&func_table}, &main) {
    return Err(String::from("Semantic Error: main doesn't exists"));
  }

  unsafe{
    func_table.clear();
  }

  return Ok(ir_code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
match tokens[index] {
Token::End => { true }
_ => { false }
}
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

fn find_symbol(symbol_table: &Vec<(String, String)>, symbol: &String, genre: &str) -> bool {
  for (symbol_in_table, type_in_table) in symbol_table {
    if symbol_in_table.eq(symbol) && type_in_table.eq(&genre) {
      return true
    }
  }
  return false;
}


  fn find_func(table: &Vec<String>, symbol: &String) -> bool {
      for func_in_table in table {
        if func_in_table.eq(symbol) {
          return true
        }
      }
    return false;
  }


fn parse_function(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  
  match tokens[*index] {
  Token::Func => { 
    *index += 1;

    symbol_table.clear();

  }
  _ => { return Err(String::from("Parser Error: functions must begin with func")); }
  }

  let mut function_code: String;
  let mut size: i32;

  match &tokens[*index] {
  Token::Ident(identifier_name) => { 
    *index += 1; 
    function_code = format!("%func {identifier_name}");

    //print!("{identifier_name}");

    if find_func(unsafe{&func_table}, &identifier_name) {
      return Err(String::from("Semantic Error: {identifier_name} func already exists"));
    }

    unsafe{
      func_table.push(identifier_name.clone());
    }
    
  }
  _  => { return Err(String::from("Parser Error: functions must have a function identifier"));}
  }

  match tokens[*index] {
  Token::LeftParen => { 
    *index += 1; 
    function_code += &format!("(");
  }
  _ => { return Err(String::from("Parser Error: expected '('"));}
  }

  //CHECKING PARAMETERS

    while !matches!(tokens[*index], Token::RightParen) {
      match tokens[*index] {
        Token::Int => {
          *index += 1;
          function_code += &format!("%int");
        }
        _ => {return Err(String::from("Parser Error: Declaration statements must being with 'int' keyword"));}
        }
      
        match &tokens[*index] {
        Token::LeftBracket => {
          *index += 1;
          function_code += &format!("[");
          match tokens[*index] {
            Token::Num(val) => {
              *index += 1;
              size = val;
            }
            _ => {return Err(String::from("Parser Error: Declaration statements for array number missing"));}
          }
          match tokens[*index] {
            Token::RightBracket => {
              *index += 1;
              function_code += &format!("] ");
            }
             _ => {return Err(String::from("Parser Error: Declaration statements for array rightBracket missing"));}
          }
          match &tokens[*index] {
              Token::Ident(ident) => {
                *index += 1;
                function_code += &format!("{ident}, {}", size);

                if find_symbol(&symbol_table, &ident, "array") {
                  return Err(String::from("Semantic Error: found duplicate variable {ident}"));
                }

                symbol_table.push((ident.clone(), "array".to_string()));
              }
               _ => {return Err(String::from("Parser Error: Declaration statements for array identifier missing"));}
            }
          }
      
          Token::Ident(ident) => {
            *index += 1;
            function_code += &format!(" {ident}");

            if find_symbol(&symbol_table, &ident, "int") {
              return Err(String::from("Semantic Error: found duplicate variable {ident}"));
            }

            symbol_table.push((ident.clone(), "int".to_string()));
          }
          _ => {return Err(String::from("Parser Error: Declarations must have an identifier or array size declaration"));}
        }

      match tokens[*index] {
        Token::Comma => {
          *index += 1;
          function_code += &format!(", "); 
        }
        Token::RightParen => {} //DO NOTHING
        _ => { return Err(String::from("Parser Error: invalid function call"));}
      }
    }

  match tokens[*index] {
  Token::RightParen => {
    *index += 1;
    function_code += &format!(")\n");
  }
  _ => { return Err(String::from("Parser Error: expected ')'"));}
  }

  match tokens[*index] {
  Token::LeftCurly => { *index += 1; }
  _ => { return Err(String::from("Parser Error: expected '{'"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {

      match parse_statement(tokens, index, symbol_table) {
      Ok(statements_code) => {
        function_code += &statements_code;
      }
      Err(e) => {return Err(e);}
      }
  }

  match tokens[*index] {
  Token::RightCurly => { *index += 1; }
  _ => { return Err(String::from("Parser Error: expected '}'"));}
  }

  function_code += &format!("%endfunc\n\n");

  return Ok(function_code);
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'
fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {

  match tokens[*index] {

  Token::Int => {
    let code = parse_declaration_statement(tokens, index, symbol_table)?;
    return Ok(code);
    //modified declaration function
  },
  Token::Ident(_) => {
    let code = parse_assignment_statement(tokens, index, symbol_table)?;
    return Ok(code);
    //modify function + parse expression
  },
  Token::Return => {
    let code = parse_return_statement(tokens, index, symbol_table)?;
    return Ok(code);
    //modify function + parse expression
  },
  Token::Print => {
    let code = parse_print_statement(tokens, index, symbol_table)?;
    return Ok(code);
    //modify function + parse expression
  },
  Token::Read => {
    let code = parse_read_statement(tokens, index, symbol_table)?;
    return Ok(code);
    //modify function + parse expression
  },
  Token::While => {
    let code = parse_While_loop(tokens, index, symbol_table);
    return Ok(code?);
    //modify function
  },
  Token::If => {

    let code = parse_if_else_statement(tokens, index, symbol_table);

    return Ok(code?);

    //modify function
  },
  Token::Continue => {
    unsafe {
    if WhileCount > 0
    {
      *index += 1;
      let code = format!("");
      match tokens[*index] {
        Token::SemiColon => {*index += 1;}
        _ => {return Err(String::from("Parser Error: Missing SemiColon"));}
      }
      return Ok(code);
    }
    else
    {
      Err(String::from("Parser Error: Continue called out of while loop"))
    }
    }
  }, //checks if continue called in while loop
  Token::Break => {
    unsafe {
    if WhileCount > 0
    {
      *index += 1;
      match tokens[*index] {
        Token::SemiColon => {*index += 1;}
        _ => {return Err(String::from("Parser Error: Missing SemiColon"));}
      }
      let code = format!("%jmp :endloop{}\n", unsafe{WhileCount});
      //let code = format!("");
      return Ok(code);
    }
    else
    {
      Err(String::from("Parser Error: Break called out of while loop"))
    }
    }
  }, //checks if break called in while loop
  _ => Err(String::from("Parser Error: invalid statement"))
  }
}

//added array declaration

fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  let mut size: i32;
  let mut ident: String;

  match tokens[*index] {
  Token::Int => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Declaration statements must being with 'int' keyword"));}
  }

  let mut code = format!("%int");

  match &tokens[*index] {
  Token::LeftBracket => {
    *index += 1;
    code += &format!("[");
    match tokens[*index] {
      Token::Num(val) => {
        *index += 1;
        size = val;

        if size <= 0
        {
          return Err(String::from("Semantic Error: can't have an array of size 0 or lower"));
        }

      }
      _ => {return Err(String::from("Parser Error: Declaration statements for array number missing"));}
      }
    match tokens[*index] {
      Token::RightBracket => {
        *index += 1;
        code += &format!("] ");
      }
       _ => {return Err(String::from("Parser Error: Declaration statements for array rightBracket missing"));}
      }
      match &tokens[*index] {
        Token::Ident(ident) => {
          *index += 1;
          code += &format!("{ident}, {size}\n");

          if find_symbol(&symbol_table, &ident, "array") {
            return Err(String::from("Semantic Error: found duplicate variable {ident}"));
          }

          symbol_table.push((ident.clone(), "array".to_string()));
        }
         _ => {return Err(String::from("Parser Error: Declaration statements for array identifier missing"));}
        }
  }

  Token::Ident(ident) => {
    *index += 1;
    code += &format!(" {ident}\n");

    if find_symbol(&symbol_table, &ident, "int") {
      return Err(String::from("Semantic Error: found duplicate variable {ident}"));
    }

    symbol_table.push((ident.clone(), "int".to_string()));
  }
  _ => {return Err(String::from("Parser Error: Declarations must have an identifier or array size declaration"));}
  }

  match tokens[*index] {
  Token::SemiColon => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Statements must end with a semicolon"));}
  }

  return Ok(code);
}

fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  let mut ident: String;
  let mut code: String;

  match &tokens[*index] {

  Token::Ident(dest) => {
    *index += 1;
    ident = dest.clone();

    match tokens[*index]{
      Token::Assign => {
        *index += 1;

        if !find_symbol(&symbol_table, &ident, "int") && !find_func(unsafe {&func_table}, &ident) {
          return Err(String::from("Semantic Error: {ident} doesn't exist / undeclared / mismatched types (not int)"));
        }
        
        let expr = parse_expression(tokens, index, symbol_table)?;

        code = format!("{}%mov {}, {}\n", expr.code, ident, expr.name);
        //codenode = CodeNode::Code(code);
      }

      Token::LeftBracket => {
      *index += 1;

      /*match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
      }*/

      let expr_index = parse_expression(tokens, index, symbol_table)?;

      match tokens[*index] {
      Token::RightBracket => {
        *index += 1;
        
        match tokens[*index]{
          Token::Assign => {
            *index += 1;

            if !find_symbol(&symbol_table, &ident, "array") && !find_func(unsafe {&func_table}, &ident) {
              return Err(String::from("Semantic Error: {ident} doesn't exist / undeclared / mismatched type (not array)"));
            }

            /*match parse_expression(tokens, index)
            {
            Ok(()) => {},
            Err(e) => {return Err(e);}
            }*/
            
            let expr = parse_expression(tokens, index, symbol_table)?;
            code = format!("{}{}%mov [{} + {}], {}\n", expr_index.code, expr.code, ident, expr_index.name, expr.name);
            //codenode = CodeNode::Code(code);
          }
          _ => {return Err(String::from("Parser Error: missing '='"));}
      }
      }
      _ => { return Err(String::from("Parser Error: missing right bracket ']'")); }
    }
    }

    _ => { return Err(String::from("Parser Error: missing '='")); }
  }
  }
  _ => {return Err(String::from("Parser Error: Assignment statements must being with an identifier"));}
  }

  match tokens[*index] {
  Token::SemiColon => {
    *index += 1;
    return Ok(code);
  }
  _ => {return Err(String::from("Parser Error: Missing SemiColon"));}
  }
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  match tokens[*index] {
  Token::Return => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Return statements must being with a return keyword"));}
  }

  let express = parse_expression(tokens,index, symbol_table)?;

  let code = format!("{}%ret {}\n", express.code, express.name);

  /*match parse_expression(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }*/

  match tokens[*index] {
  Token::SemiColon => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Statement is missing the '=' operator"));}
  }

  return Ok(code);
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  match tokens[*index] {
  Token::Print=> {*index += 1;}
  _ => {return Err(String::from("Parser Error: Return statements must being with a return keyword"));}
  }

  let express_p = parse_expression(tokens,index, symbol_table)?;
  let code = format!("{}%out {}\n", express_p.code, express_p.name);

  /*match parse_expression(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }*/

  match tokens[*index] {
  Token::SemiColon => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Statement is missing the '=' operator"));}
  }



  return Ok(code);
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {

  match tokens[*index] {
  Token::Read => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Return statements must being with a return keyword"));}
  }

  let express = parse_expression(tokens,index, symbol_table)?;
  let code = format!("%input {}\n", express.name);
  
  /*match parse_expression(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }*/

  match tokens[*index] {
  Token::SemiColon => {*index += 1;}
  _ => {return Err(String::from("Parser Error: Statement is missing the '=' operator"));}
  }

  return Ok(code);
}

struct Expression {
  code: String,
  name: String,
}

fn create_temp() -> String {
  // Increment the counter and return a temporary variable name
  let count = TEMP_COUNT.fetch_add(1, Ordering::SeqCst);
  format!("t{}", count)
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<Expression, String> {
  let mut opcode: String;

  /*match parse_multiply_expression(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }*/

  let mut expr = parse_multiply_expression(tokens, index, symbol_table)?;

  loop {
     match tokens[*index] {

     Token::Plus => {
         *index += 1;
        opcode = format!("%add");
         /*match parse_multiply_expression(tokens, index) {
         Ok(()) => {},
         Err(e) => {return Err(e);}
         }*/
     }

     Token::Subtract => {
         *index += 1;
         opcode = format!("%sub");
         /*match parse_multiply_expression(tokens, index) {
         Ok(()) => {},
         Err(e) => {return Err(e);}
         }*/
     }

     _ => { 
         break;
     }

     };

     let m_expr = parse_multiply_expression(tokens, index, symbol_table)?;
     let mut t = create_temp();
     let mut instr: String;
     instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
     expr.code += &m_expr.code;
     expr.code += &instr;
     expr.name = t;
  }

  return Ok(expr);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<Expression, String> {
  let mut opcode: String;

  let mut expression = parse_term(tokens, index, symbol_table)?;

  /*match parse_term(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }*/

  loop {
     match tokens[*index] {
     Token::Multiply => {
        *index += 1;
        opcode = format!("%mult");
        /*match parse_term(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
        }*/
        let mut term = parse_term(tokens, index, symbol_table)?;
        expression.code += &term.code;
        let mut t2 = create_temp();
        let mut instr2: String;
        instr2 = format!("%int {}\n{opcode} {}, {}, {}\n", t2, t2, expression.name, term.name);
        expression.code += &instr2;
        expression.name = t2;
     }

     Token::Divide => {
        *index += 1;
        opcode = format!("%div");
        /*match parse_term(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
        }*/
        let mut term = parse_term(tokens, index, symbol_table)?;
        expression.code += &term.code;
        let mut t2 = create_temp();
        let mut instr2: String;
        instr2 = format!("%int {}\n{opcode} {}, {}, {}\n", t2, t2, expression.name, term.name);
        expression.code += &instr2;
        expression.name = t2;
     }

     Token::Modulus => {
        *index += 1;
        opcode = format!("%mod");
        /*match parse_term(tokens, index) {
        Ok(()) => {},
        Err(e) => {return Err(e);}
        }*/
        let mut term = parse_term(tokens, index, symbol_table)?;
        expression.code += &term.code;
        let mut t2 = create_temp();
        let mut instr2: String;
        instr2 = format!("%int {}\n{opcode} {}, {}, {}\n", t2, t2, expression.name, term.name);
        expression.code += &instr2;
        expression.name = t2;
     }

     _ => {
         break;
     }
     };

  }

  return Ok(expression);
}

// a term is either a Number or an Identifier.
//INCLUDES Arr[expression]
fn parse_term(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<Expression, String> {

let mut ident: String;
let mut instr: String = Default::default();

  match &tokens[*index] {
  Token::Ident(dest) => {
      *index += 1;
      ident = dest.clone();

      //print!("{ident}");

      /*
      unsafe{
      if !find_symbol(&symbol_table, &ident, "int") && !find_func(&func_table, &ident) !find_symbol(&symbol_table, &ident, "array") {
        return Err(String::from("Semantic Error: function / identifier not found"));
      }
      }
      */

      let mut term = Expression {
        code : String::from(""),
        name : dest.clone(),
      };

      match tokens[*index] {
        Token::LeftBracket => {
          *index += 1;

          /*match parse_expression(tokens, index) {
            Ok(()) => {},
            Err(e) => {return Err(e);}
          }*/

          let inside_term = parse_expression(tokens, index, symbol_table)?;

          match tokens[*index] {
            Token::RightBracket => {
              *index += 1;

              if !find_symbol(&symbol_table, &ident, "array") {
                return Err(String::from("Semantic Error: array not found"));
              }

              instr += &format!("[{ident} + {}]\n", inside_term.name);
              let mut t = create_temp();
              term.code = format!("%int {}\n%mov {}, {}", t, t, instr);
              term.name = t;
              //term.code = &instr;

              return Ok(term);
            }
            _ => { return Err(String::from("Parser Error: missing right bracket ']'"));}
          }
        }

        //FUNCTION CALL
        Token::LeftParen => {
          *index += 1;
          instr += &format!("{}(", dest);
          while !matches!(tokens[*index], Token::RightParen) {

            let mut inside = parse_expression(tokens, index, symbol_table)?;
            
            term.code += &format!("{}\n", inside.code);
        
            instr += &format!("{}", inside.name);

            /*
            match parse_expression(tokens, index) {
              Ok(()) => {}
              Err(e) => {}
            }
            */

            match tokens[*index] {
              Token::Comma => {
                *index += 1; 
                instr += &format!(", ");
              }
              Token::RightParen => {} //DO NOTHING
              _ => { return Err(String::from("Parser Error: invalid function call"));}
            }
          }
          match tokens[*index] {
            Token::RightParen => {
              *index += 1;
              instr += &format!(")");

              unsafe{
                if !find_func(&func_table, &dest) {
                  return Err(String::from("Semantic Error: function not found"));
                }
              }

              if find_func(unsafe {&func_table}, &dest)
              {
                let mut t = create_temp();
                term.code += &format!("%int {}\n%call {}, {}\n", t, t, instr);
                term.name = t;
              }

              else{
              term.name = dest.clone();
              term.code += &format!("");
              }
            }
            _ => { return Err(String::from("Parser Error: missing right bracket ']'"));}
          }
        }
        _ => {}
      }

        if !find_symbol(&symbol_table, &ident, "int") && !find_func(unsafe {&func_table}, &dest) {
          return Err(String::from("Semantic Error: function / identifier int not found"));
        }

      //let instr = format!("{ident}");
      //term.name += &instr;
      //term.code = $instr;

      return Ok(term);
  }

  Token::Num(val) => {
      *index += 1;

      let term = Expression {
        code : String::from(""),
        name : format!("{val}"),
      };

      return Ok(term);
  }


  //(a) MIGHT BE WRONG
  Token::LeftParen => {
      *index += 1;

      /*match parse_expression(tokens, index) {
      Ok(()) => {},
      Err(e) => {return Err(e);}
      }*/

      let term = parse_expression(tokens, index, symbol_table)?;

      match tokens[*index] {
      Token::RightParen => {*index += 1;}
      _ => { return Err(String::from("Parser Error: missing right parenthesis ')'")); }
      }
      return Ok(term);
  }
  
  _ => {
      return Err(String::from("Parser Error: missing expression term."));
  }

  }
}

// Boolean Expression (ex: a < b)

fn parse_bool_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<Expression, String> {
  let mut first_term: String;
  let mut second_term: String;
  let mut inequality: String;

  let mut term = Expression {
    code : String::from(""),
    name : String::from(""),
  };

  match parse_expression(tokens, index, symbol_table) {
    Ok(code) => {
      first_term = code.name;
    },
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Greater => {
      *index += 1;
      inequality = format!("gt");
    }
    Token::Less => {
      *index += 1;
      inequality = format!("lt");
    }
    Token::LessEqual => {
      *index += 1;
      inequality = format!("le");
    }
    Token::GreaterEqual => {
      *index += 1;
      inequality = format!("ge");
    }
    Token::NotEqual => {
      *index += 1;
      inequality = format!("neq");
    }
    Token::Equality => {
      *index += 1;
      inequality = format!("eq");
    }
    _ => {return Err(String::from("Parser Error: inequality sign missing"));}
  }

  match parse_expression(tokens, index, symbol_table) {
    Ok(code) => {
      second_term = code.name;
    },
    Err(e) => {return Err(e);}
  }

  /*
  outputs:
  %int _temp1
  %lt _temp1, i, 10
  */

  let t = create_temp();
  term.code += &format!("%int {}\n%{} {}, {}, {}", t, inequality, t, first_term, second_term);
  term.name = t;

  return Ok(term);
}

// While Loop (Account for break / continue)

fn parse_While_loop(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  let mut code: String;

  match tokens[*index] {
    Token::While => {
      *index += 1;
      unsafe {
        WhileCount += 1;
      
        code = format!(":loopbegin{}\n", WhileCount);
      }
    }
    _ => {return Err(String::from("Parser Error: missing 'while'"));}
  }

  match parse_bool_expression(tokens, index, symbol_table) {
    Ok(term) => {
      code += &format!("{}\n", term.code);
      code += &format!("%branch_ifn {}, :endloop{}\n", term.name, unsafe{WhileCount});
    },
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::LeftCurly => { *index += 1; }
    _ => { return Err(String::from("Parser Error: expected '{'"));}
    }
  
    while !matches!(tokens[*index], Token::RightCurly) {

        //MAY NEED TO OUTPUT STATEMENTS
        match parse_statement(tokens, index, symbol_table) { 
        Ok(statement) => {
          code += &format!("{}", statement);
        }
        Err(e) => {return Err(e);}
        }
    }
  
  
    match tokens[*index] {
    Token::RightCurly => { 
      *index += 1;
      code += &format!("%jmp :loopbegin{}\n:endloop{}\n", unsafe{WhileCount}, unsafe{WhileCount});
     }
    _ => { return Err(String::from("Parser Error: expected '}'"));}
    }
    unsafe {
      WhileCount -= 1;
    

    if WhileCount < 0
    {
      WhileCount = 0;
    }
    }
    return Ok(code);
}

// If/Else statement (Doesn't account for nested loops)

fn parse_if_else_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<(String, String)>) -> Result<String, String> {
  let mut code: String;

  match tokens[*index] {
    Token::If => {
      *index += 1;
      unsafe{ifCount += 1;}
    }
    _ => {return Err(String::from("Parser Error: missing 'if'"));}
  }

  match parse_bool_expression(tokens, index, symbol_table) {
    Ok(term) => {
      code = format!("{}\n", term.code);
      code += &format!("%branch_if {}, :iftrue{}\n%jmp :else{}\n", term.name, unsafe{ifCount}, unsafe{ifCount});
    },
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::LeftCurly => { 
      *index += 1;
      code += &format!(":iftrue{}\n", unsafe{ifCount});
     }
    _ => { return Err(String::from("Parser Error: expected '{'"));}
    }
  
    while !matches!(tokens[*index], Token::RightCurly) {

        match parse_statement(tokens, index, symbol_table) {
        Ok(statement) => {
          code += &format!("{}", statement);
          code += &format!("%jmp :endif{}\n", unsafe{ifCount});
        }
        Err(e) => {return Err(e);}
        }
    }
  
  
    match tokens[*index] {
    Token::RightCurly => { *index += 1; }
    _ => { return Err(String::from("Parser Error: expected '}'"));}
    }

    match tokens[*index] {
    Token::Else => {
        match tokens[*index] {
          Token::Else => {
            *index += 1;
            code += &format!(":else{}\n", unsafe{ifCount});
            //code += &format!(":else{}\n", unsafe{ifCount});
          }
          _ => {return Err(String::from("Parser Error: missing 'else'"));}
        }
      
        match tokens[*index] {
          Token::LeftCurly => { 
            *index += 1; 
          }
          _ => { return Err(String::from("Parser Error: expected '{'"));}
        }
        
          while !matches!(tokens[*index], Token::RightCurly) {
        
              match parse_statement(tokens, index, symbol_table) {
              Ok(statement) => {
                code += &format!("{}", statement);
              }
              Err(e) => {return Err(e);}
              }
          }
        
        
          match tokens[*index] {
          Token::RightCurly => { *index += 1; }
          _ => { return Err(String::from("Parser Error: expected '}'"));}
          }
    }
    _ => {code += &format!(":else{}\n", unsafe { ifCount });}
  }

  code += &format!(":endif{}\n", unsafe { ifCount });

  /*
  unsafe{
    ifCount -= 1;
  
    if ifCount < 0
    {
      ifCount = 0;
    }
  }
  */

  return Ok(code);
}



// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
/*
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 6);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));
        assert!(matches!(toks[5], Token::End));

        let toks = lex("3 + 215 +").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::End));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));

}*/
