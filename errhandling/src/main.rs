// fn parsr(inpt: String) -> Option<i32> {
//     inpt.parse().ok()
// }

// handle None as zero
// fn sum_str_vec(input: Vec<String>) -> String {
//     let mut accum = 0i32;
//     for i in input.into_iter() {
//         // if let Some(val) = parsr(i) {
//         //     accum += val;
//         // }
//         accum += parsr(i).unwrap_or(0); // to return default value for option when getting None
//     }
//     accum.to_string()
// }

// Propagate None to be handled by the caller instead of us assuming None=0
// fn sum_str_vec(input: Vec<String>) -> Option<String> {
//     let mut accum = 0i32;
//     for i in input.into_iter() {
//         accum += parsr(i)?; // ? here propagates None value (i.e.) when ? used
//     }
//     Some(accum.to_string())
// }

// #[derive(Debug)]
// struct SumErr;

// // Above doesnt add any addn info, so we return result
// fn sum_str_vec(input: Vec<String>) -> Result<String, SumErr> {
//     let mut accum = 0i32;
//     for i in input.into_iter() {
//         accum += parsr(i).ok_or(SumErr)?;
//     }
//     Ok(accum.to_string())
// }

// fn main() {
//     let inpt = vec![String::from("3"), String::from("4")];
//     let total = sum_str_vec(inpt);
//     println!("{total:?}");

//     println!("---");

//     let inpt = vec![String::from("3"), String::from("abc")];
//     let total = sum_str_vec(inpt);
//     println!("{total:?}");
// }

use std::num::ParseIntError;

#[derive(Debug)]
struct SumErr;

fn parsr(inpt: String) -> Result<i32, ParseIntError> {
    inpt.parse() // No need for aquestion mark here, because we arent returning early
}

// fn sum_str_vec(input: Vec<String>) -> Result<String, ParseIntError> {
//     // return same err type
//     let mut accum = 0i32;
//     for i in input.into_iter() {
//         accum += parsr(i)? // ? here to return err early
//     }
//     Ok(accum.to_string())
// }

fn sum_str_vec(input: Vec<String>) -> Result<String, SumErr> {
    // Convert err type
    let mut accum = 0i32;
    for i in input.into_iter() {
        accum += parsr(i).map_err(|_| SumErr)? // ? here to return err early
    }
    Ok(accum.to_string())
}

fn main() {
    let inpt = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(inpt);
    println!("{total:?}");

    println!("---");

    let inpt = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(inpt);
    println!("{total:?}");
}
