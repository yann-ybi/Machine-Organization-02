use csc411_image::{GrayImage, Read};
use csc411_image::Gray;
use std::env;
use std::process;
use array2::*;

// The check_row function constructs a new 2 dimensional array from the image structure read from the input
// it checks whether a value between 1 and 9 included is present more than once in a same row
// if a value is repeated withing a row it terminates the process with exits with code 1.

fn check_rows(sudoku: &mut Array2<Gray>){
    let mut k = 0;
    // For every element in the 2D array, iterate throught its values in row major order
    for index in sudoku.iter_row_major(){
        let current = sudoku.arr[index.2].value;
        k += 1;
        // for every value iterate through the remaining values present in the row 
        for other_index in sudoku.iter_row_major().skip(k){
            
            if index.1 != other_index.1{ 
                break;
            }
            let next = sudoku.arr[other_index.2].value;
            if next == current {
                process::exit(1);
            }
        }
    }
}

// The check_col function constructs a new 2 dimensional array from the image structure read from the input
// it checks whether a value between 1 and 9 included is present more than once in a same column
// if a value is repeated withing a row it terminates the process with exits with code 1.

fn check_cols(sudoku: &mut Array2<Gray>){
    let mut k = 0;
    // For every element in the 2D array, iterate throught its values in column major order
    for index in sudoku.iter_col_major(){
        let current = sudoku.arr[index.2].value;
        if current > 9 || current < 1 {
            process::exit(1);
        }
        k += 1;
        // for every value iterate through the remaining values present in the column 
        for other_index in sudoku.iter_col_major().skip(k){
            if index.0 != other_index.0{ 
                break;
            }
            let next = sudoku.arr[other_index.2].value;
            if next == current {
                process::exit(1);
            }
        }
    }
}

// The check_blocks function constructs a new 2 dimensional array from the image structure read from the input
// it checks whether a value between 1 and 9 included is present more than once in a same block of 3*3 size
// if a value is repeated withing a block it terminates the process with exits with code 1.

fn check_blocks(sudoku: &Array2<Gray>){
    let mut block = vec![(0,0),(1,0),(2,0), (0,1), (1,1), (2,1), (0,2), (1,2), (2,2)];
    // for every block present in a sudoku
    for _i in 1..10{
        let mut k = 0;
        // for every values prensent in our block
        for tuple in  block.iter(){
            k += 1;
            let curr = sudoku.arr[9 * tuple.1 + tuple.0].value;
            // for every values remaining in our block
            for other_tuple in  block.iter().skip(k){
                let next = sudoku.arr[9 * other_tuple.1 + other_tuple.0].value;
                if next == curr{
                    process::exit(1);
                }
            } 
        }
        if k % 9 == 0 {
            if block[8].0 % 8 != 0{
                block = block.iter_mut().map(|x|(x.0 + 3, x.1)).collect::<Vec<(usize, usize)>>();
            } else {
                block = block.iter_mut().map(|x|(x.0 - 6, x.1 + 3)).collect::<Vec<(usize, usize)>>();
            }
        }
    }
}

// function check_sudoku calls all check_row, check_col and check_block functions in order to check the whole sudoku
fn check_sudoku(sudoku: &mut Array2<Gray>){
    check_rows(sudoku);
    check_cols(sudoku);
    check_blocks(sudoku);
}

fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let mut sudoku = Array2::new(img.pixels, (img.width as usize, img.height as usize));
    check_sudoku(&mut sudoku);
    process::exit(0);
}