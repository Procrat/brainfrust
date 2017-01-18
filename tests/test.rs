extern crate brainfrust;

use std::path::Path;


#[test]
fn hello_world() {
    test_case("hello_world.b", "", "Hello World!\n")
}

#[test]
fn taking_over_intro() {
    test_case("taking_over_intro.b", "",
    "Taking Over The World is a little, in brainfuck programmed textgame.\n\n\
    Like the name says it already, this game is about taking over the world. But it's not you \
    who wants to do that, it's the insane Dr. Retipuj and you must stop him. During the \
    conversation he asks you a few questions and if you answer them correct, mostly so that you \
    don't insult him, he won't kill you. The answer is always a number between 0 and 255. \
    Examples are: How old are you, how many drops of sweat do you have, how many pieces of sugar \
    do you want,... He gives you also directions in which area the answer should be. But you will \
    only follow them if you can easily be manipulated by the wrong person. If you survive until \
    the end you will contest a duel in Retipuj Roulette. In his version of Russian Roulette luck \
    hasn't such an important role. You can determine the position of the bullet but he can choose \
    who will begin. Fortunately a revolving breech is round so you can use his tactics against \
    him. But even if you win there's one stroke of fate you can't avoid. Altogether there are 13 \
    questions and also 13 roman-easter-eggs you can search if you want.\n\n\
    The download contains also BIABI (BIABI is a brainfuck interpreter). The game uses this \
    interpreter, but of course you can also use it for your own programs. It is written in C.\n\n\
    Taking Over The World and BIABI are both released under the terms of the GNU GPL.\n\n")
}

#[test]
fn quine410() {
    test_case("quine410.b", "", "->++>+++>+>+>++>>+>+>+++>>+>+>++>+++>+++>+>>>>>>>>>>>>>>>>>>>>>>>\
              >>>>>>>>>>+>+>++>>>+++>>>>>+++>+>>>>>>>>>>>>>>>>>>>>>>+++>>>>>>>++>+++>+++>+>>+++>>>\
              +++>+>+++>+>++>+++>>>+>+>+>+>++>+++>+>+>>+++>>>>>>>+>+>>>+>+>++>+++>+++>+>>+++>+++>+\
              >+++>+>++>+++>++>>+>+>++>+++>+>+>>+++>>>+++>+>>>++>+++>+++>+>>+++>>>+++>+>+++>+>>+++\
              >>+++>>+[[>>+[>]+>+[<]<-]>>[>]<+<+++[<]<<+]>>+[>]+++[++++++++++>++[-<+++++++++++++++\
              +>]<.<-<]")
}

#[test]
fn cat() {
    test_case("cat.b", "This\ngets\ncopied.\n", "This\ngets\ncopied.\n")
}

#[test]
fn prime() {
    test_case("prime.b", "45\n", "Primes up to: 2 3 5 7 11 13 17 19 23 29 31 37 41 43 \n")
}

fn test_case(file_name: &str, input: &str, expected_output: &str) {
    let test_file = Path::new(file!()).with_file_name(file_name);
    let mut output = vec![];
    brainfrust::run(test_file, input.as_bytes(), &mut output);
    let output = String::from_utf8(output).unwrap();
    assert_eq!(output, expected_output)
}
