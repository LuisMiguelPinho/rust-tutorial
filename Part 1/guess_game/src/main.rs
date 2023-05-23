/*
 * Copyright 2022 Instituto Superior de Engenharia do Porto
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::io;
use rand::Rng;

fn guess_game(num_temptives:i8) {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..100);
    println!("Cheating: the secret number is: {secret_number}");

    let mut remaining = num_temptives;
    while remaining > 0 {

        println!("Please input your guess (tries: {remaining}/{num_temptives}).");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        if guess == secret_number{
            break;
        }
        println!("Wrong! Try again.");
        remaining-=1;
    }
    if remaining == 0 {
        println!("You blew it! The secret was {secret_number}.");
    }else{
        println!("You guessed it! It was {secret_number}.");
    }
}

fn main(){
    guess_game(3);
}

