use std::collections::VecDeque;

use proc_macro as pm;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;

fn label(jump_target: usize) -> TokenStream {
    TokenStream::from_iter([
        TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
        TokenTree::Ident(Ident::new(&(format!("goto{jump_target}")), Span::call_site())),
    ])
}

#[proc_macro]
pub fn transpile_program(_input: pm::TokenStream) -> pm::TokenStream {
    let input = include_str!("/home/auno/stuff/projects/adventofcode/2024/input/2024/day17.txt");
    let (_, program) = input.split_once("\n\n").unwrap();
    let (_, program) = program.split_once(": ").unwrap();
    let program = program.trim().split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let jump_targets = (0..program.len())
        .step_by(2)
        .filter(|i| program[*i] == 3)
        .map(|i| program[i + 1])
        .collect::<Vec<_>>();

    let mut i = program.len();
    let mut tokens = VecDeque::<TokenStream>::new();

    loop {
        if jump_targets.contains(&i) {
            let label_token = label(i);
            let token_stream = TokenStream::from_iter(tokens);
            tokens = VecDeque::from([
                quote! {
                    #label_token: loop {
                        #token_stream
                        break;
                    }
                }
            ]);
        }

        if i < 1 {
            break;
        }

        let [instruction, operand] = program[(i - 2)..i] else { panic!("parse error: {i}") };

        let combo_operand_token = match operand {
            0..=3 => quote! { #operand },
            4 => quote! { a },
            5 => quote! { b },
            6 => quote! { c },
            _ => quote! { panic!("combo operand > 6: {}", #operand) },
        };

        let token = match instruction {
            0 => { quote! { a >>= #combo_operand_token; } },
            1 => { quote! { b ^= #operand; } },
            2 => { quote! { b = #combo_operand_token % 8; } },
            3 => {
                let label_token = label(operand);

                quote! {
                    if a > 0 {
                        continue #label_token;
                    }
                }
            },
            4 => { quote! { b ^= c; } },
            5 => { quote! { output.push(#combo_operand_token % 8); } },
            6 => { quote! { b = a >> #combo_operand_token; } },
            7 => { quote! { c = a >> #combo_operand_token; } },
            _ => panic!("parse error, instruction > 7: {instruction}"),
        };

        tokens.push_front(token);

        i -= 2;
    }

    let token_stream = TokenStream::from_iter(tokens);

    quote! {
        fn run_program_compiled(registers: [usize; 3]) -> Result<(Vec<usize>, Registers)> {
            let [mut a, mut b, mut c] = registers;
            let mut output: Vec<usize> = vec![];

            #token_stream

            Ok((output, [a, b, c]))
        }
    }.into()
}
