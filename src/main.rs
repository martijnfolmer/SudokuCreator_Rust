/*
    Sudoku_creator : This is a collection of functions which allows us to generate both Sudokus
    that we can solve, as well as their solutions

    Right now, it can only generate standard sudokus of size 9x9. A sudoku is possible, as long as
    width = height and square_root(width) = postivie integer. Meaning, you can have sudokus
    of size 1x1, 4x4, 9x9, 16x16, 25x25, 36x36 and so on.

    Author : Martijn Folmer
    Date : 19-01-2024
 */

use rand::seq::SliceRandom;     // random slices
use rand::Rng;                  // random numbers
use std::collections::HashSet;  // Get a hashset (which is an unordered list of unique values)
use num::integer::sqrt;         // square root



/// Fill in a row with values, to create a filled in sudoku
///
/// # Arguments
/// `sudoku` - A 2D vector (`Vec<Vec<i32>>`) representing the Sudoku grid.
/// * 'numbers' - the numbers to fill in
/// 'row_index' - The row to fill in
/// 'column_offset' - the offset to start filling in values. So if column_offset = 3, we start filling in the values at column_idx = 3
fn fill_row(mut sudoku : Vec<Vec<i32>>, numbers:&Vec<i32>, row_index:usize, column_offset:usize) -> Vec<Vec<i32>>{

    for (i, &n) in numbers.iter().enumerate(){
        let mut idx = i + column_offset;
        if idx>=9{
            idx -= 9;
        }
        sudoku[row_index][idx] = n;
    }

    sudoku
}

/// Get a random number within a range
///
/// # Arguments
/// 'range' - the range from within the random number is selected
fn generate_random_number(range: std::ops::Range<i32>) -> i32{
    // will generate 1 number inside of the range given
    let mut rng = rand::thread_rng();

    // Generate the first random number
    let num = rng.gen_range(range);

    num
}

///Return two random numbers which are not the same as eachother, within a range
///
/// # Arguments
/// 'range' - the range from within the two numbers are selected
fn generate_two_unique_random_numbers(range: std::ops::Range<i32>) -> (usize, usize) {
    // will generate 2 numbers inside of range.
    // if range = 0..3, the numbers can be [0, 1, 2], so the possible returns are [0, 1], [0, 2], [1, 0], [1,2], [2, 0], [2, 1]
    let mut rng = rand::thread_rng();

    // Generate the first random number
    let num1 = rng.gen_range(range.clone());

    // Generate the second random number until it is different from the first
    let mut num2;
    loop {
        num2 = rng.gen_range(range.clone());
        if num2 != num1 {
            break;
        }
    }

    (num1.try_into().unwrap(), num2.try_into().unwrap())
}


/// Flip the values between 2 rows
///
/// # Arguments
/// `sudoku` - A 2D vector (`Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'row_idx1' - The index of one of the row to swap
/// 'row_idx2' - The index of the other of the row to swap
fn flip_row(mut sudoku: Vec<Vec<i32>>, row_idx1: usize, row_idx2: usize) -> Vec<Vec<i32>> {
    // Swap the rows directly without using temporary vectors
    for i in 0..sudoku[row_idx1].len() {
        let temp = sudoku[row_idx1][i];
        sudoku[row_idx1][i] = sudoku[row_idx2][i];
        sudoku[row_idx2][i] = temp;
    }

    // Return the sudoku
    sudoku
}

/// Randomly flipping the rows within a specific subgrid, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'number_of_attempts' - how many times to attempt to flip any two rows
/// 'row_lower_idx' - the upper most value of the rows of the subgrids
/// 'row_upper_idx' - the lower most value of the rows of the subgrids
fn flip_rows(mut sudoku: Vec<Vec<i32>>, number_of_attempts:i32, row_lower_idx:i32, row_upper_idx:i32) -> Vec<Vec<i32>>{
    // will randomly flip the rows between an upper and lower idx
    for _ in 0..number_of_attempts{
        let (random_num1, random_num2) = generate_two_unique_random_numbers(row_lower_idx..(row_upper_idx+1));
        sudoku = flip_row(sudoku, random_num1, random_num2);
    }

    //return the sudoku
    sudoku

}
/// Randomly flipping the rows within the subgrids, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn flip_all_rows(mut sudoku: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    // flip all of the sets of rows
    sudoku = flip_rows(sudoku, 5, 0, 2);
    sudoku = flip_rows(sudoku, 5, 3, 5);
    sudoku = flip_rows(sudoku, 5, 6, 8);

    sudoku
}

/// Randomly flipping the rows within the subgrids, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn flip_grid_rows(mut sudoku: Vec<Vec<i32>>) -> Vec<Vec<i32>>{

    let rows_to_swap1 = vec![0, 1, 2];
    let rows_to_swap2 = vec![3, 4, 5];
    let rows_to_swap3 = vec![6, 7, 8];

    let vector_of_vectors: Vec<Vec<usize>> = vec![rows_to_swap1, rows_to_swap2, rows_to_swap3];

    for _ in 0..5{
        let (random_num1, random_num2) = generate_two_unique_random_numbers(0..3);
        for i in 0..3{
            sudoku = flip_row(sudoku, vector_of_vectors[random_num1][i], vector_of_vectors[random_num2][i]);
        }
    }

    //return the sudoku
    sudoku
}


///Flip the values between 2 columns
///
/// # Arguments
/// `sudoku` - A 2D vector (`Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'column_idx1' - The index of one of the columns to swap
/// 'column_idx2' - The index of the other of the columns to swap
fn flip_column(mut sudoku: Vec<Vec<i32>>, column_idx1: usize, column_idx2:usize) -> Vec<Vec<i32>>{
    // swap the columns directly without using temporary vectors
    for i in 0..sudoku.len(){
        let temp = sudoku[i][column_idx1];
        sudoku[i][column_idx1] = sudoku[i][column_idx2];
        sudoku[i][column_idx2] = temp;
    }

    //Return the sudoku
    sudoku
}

/// Randomly flipping the columns within a specific subgrid, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'number_of_attempts' - how many times to attempt to flip any two columns
/// 'column_lower_idx' - the left most value of the columns of the subgrids
/// 'column_upper_idx' - the right most value of the columns of the subgrids
fn flip_columns(mut sudoku: Vec<Vec<i32>>, number_of_attempts:i32, column_lower_idx:i32, column_upper_idx:i32) -> Vec<Vec<i32>>{
    // will randomly flip the columns between and upper and lower idx
    for _ in 0..number_of_attempts{
        let (random_num1, random_num2) = generate_two_unique_random_numbers(column_lower_idx..(column_upper_idx+1));
        sudoku = flip_column(sudoku, random_num1, random_num2);
    }

    //return the sudoku
    sudoku
}


/// Randomly flipping the columns within the subgrids, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn flip_all_columns(mut sudoku: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    // flip all of the sets of rows

    sudoku = flip_columns(sudoku, 5, 0, 2);
    sudoku = flip_columns(sudoku, 5, 3, 5);
    sudoku = flip_columns(sudoku, 5, 6, 8);

    // return the sudoku
    sudoku
}

/// Randomly flipping the columns within the subgrids, to further randomize the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn flip_grid_columns(mut sudoku: Vec<Vec<i32>>) -> Vec<Vec<i32>>{

    let columns_to_swap1 = vec![0, 1, 2];
    let columns_to_swap2 = vec![3, 4, 5];
    let columns_to_swap3 = vec![6, 7, 8];

    let vector_of_vectors: Vec<Vec<usize>> = vec![columns_to_swap1, columns_to_swap2, columns_to_swap3];

    for _ in 0..5{
        let (random_num1, random_num2) = generate_two_unique_random_numbers(0..3);
        for i in 0..3{
            sudoku = flip_column(sudoku, vector_of_vectors[random_num1][i], vector_of_vectors[random_num2][i]);
        }
    }

    //return the sudoku
    sudoku
}

/// Rotate the sudoku by 90 degrees
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn rotate_90_degrees(sudoku: &mut Vec<Vec<i32>>) {
    let n = sudoku.len();
    let m = sudoku[0].len();
    let mut rotated_sudoku = vec![vec![0; n]; m];

    for i in 0..n {
        for j in 0..m {
            rotated_sudoku[j][n - 1 - i] = sudoku[i][j];
        }
    }

    *sudoku = rotated_sudoku;
}

/// Rotate the sudoku by 180 degrees
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn rotate_180_degrees(sudoku: &mut Vec<Vec<i32>>) {
    let n = sudoku.len();
    let m = sudoku[0].len();
    let mut rotated_sudoku = vec![vec![0; n]; m];

    for i in 0..n {
        for j in 0..m {
            rotated_sudoku[n - 1 - i][m - 1 - j] = sudoku[i][j];
        }
    }

    *sudoku = rotated_sudoku;
}

/// Rotate the sudoku by 270 degrees
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn rotate_270_degrees(sudoku: &mut Vec<Vec<i32>>) {
    let n = sudoku.len();
    let m = sudoku[0].len();
    let mut rotated_sudoku = vec![vec![0; n]; m];

    for i in 0..n {
        for j in 0..m {
            rotated_sudoku[m - 1 - j][i] = sudoku[i][j];
        }
    }

    *sudoku = rotated_sudoku;
}

/// Randomly rotates the sudoku by either 0, 90, 180 or 270 degrees
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn random_rotate(sudoku: &mut Vec<Vec<i32>>){
    // rotate the sudoku by 0, 90, 180 or 270 degrees.
    let rot_num = generate_random_number(0..3);

    if rot_num ==1 {
        rotate_90_degrees(sudoku);
    }
    else if rot_num == 2{
        rotate_180_degrees(sudoku);
    }
    else if rot_num == 3 {
        rotate_270_degrees(sudoku);
    }
}

/// Returns a vector with all values from a particular column in the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'column_idx' - Column index
fn get_column(sudoku: &Vec<Vec<i32>>, column_idx : i32) -> Vec<i32>{

    let mut column = Vec::new();
    let n = sudoku.len();

    for i in 0..n {
        column.push(sudoku[i][column_idx as usize]);
    }
    column
}

/// Returns a vector with all values from a particular row in the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'row_idx' - Row index
fn get_row(sudoku: &Vec<Vec<i32>>, row_idx : i32) -> Vec<i32>{

    let mut row = Vec::new();
    let n = sudoku.len();

    for i in 0..n {
        row.push(sudoku[row_idx as usize][i]);
    }
    row
}


/// Returns a vector with all values from a particular subgrid in the sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'row_idx1' - Top left row index
/// 'row_idx2' - Bottom right row index
/// 'column_idx1' - Top left column index
/// 'column_idx2' - Bottom right column index
fn get_subgrid(sudoku: &Vec<Vec<i32>>, row_idx1 : i32, row_idx2 : i32, column_idx1 : i32, column_idx2 : i32) -> Vec<i32> {
    let mut subgrid = Vec::new();

    for i in row_idx1..row_idx2{
        for j in column_idx1..column_idx2 {
            subgrid.push(sudoku[i as usize][j as usize]);
        }
    }
    subgrid
}

/// Returns True if there are no duplicates in the given Vec<i32>
///
/// # Arguments
/// * 'vec' - a reference to the Vec<i32> that we are checking for duplicates
fn is_vec_valid(vec: &Vec<i32>) -> bool {

    // A hash set = unordered set of unique elements, it does not allow duplicates. When we insert
    // and get false, it means the value is already inside of the hashmap.
    let mut seen = std::collections::HashSet::new();

    for &num in vec {
        if num != 0 {
            if !seen.insert(num) {
                // The number is already in the HashSet, meaning it's a duplicate non-zero number
                return false;
            }
        }
    }
    true
}

/// Returns True/False, based on whether all rules are followed for a specific spot defined by
/// (xco, yco). The rules are no duplicates in rows, columns or within the subgrid
///
///  # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'xco' - the column coordinate of the spot we want to check
/// 'yco' - the row coordinate of the spot we want to check
fn is_loc_valid(sudoku: &Vec<Vec<i32>>, xco : i32, yco:i32) -> bool {

    // Check if there are any errors for this grid space (meaning if there are any non-zero
    // duplicates)

    // get the values of the row and column
    let row = get_row(&sudoku, yco);
    let column = get_column(&sudoku, xco);

    // get the values of the subgrid
    let coor = get_subgrid_coor(xco, yco);
    let subgrid = get_subgrid(&sudoku, coor.0, coor.2, coor.1, coor.3);

    // return if the row, column and subgrid are all valid (so no non-zero duplicates)
    is_vec_valid(&row) && is_vec_valid(&column) && is_vec_valid(&subgrid)

}

/// Given a vector, return another vector with all numbers between 1 and 9 which are not present
/// in the original vector
///
/// # Arguments
/// * 'vec' - A reference to a Vec<i32>, which contains any i32 numbers
fn find_missing_numbers(vec: &Vec<i32>) -> Vec<i32> {
    let mut present_numbers = vec![];

    // Collect the unique non-zero numbers in the vector
    for &num in vec {
        if num != 0 {
            present_numbers.push(num);
        }
    }

    // Create a HashSet from the collected numbers (so all unique values)
    let present_set: std::collections::HashSet<_> = present_numbers.iter().cloned().collect();

    // Find the missing numbers between 1 and 9
    (1..=9)
        .filter(|&num| !present_set.contains(&num))
        .collect()
}


///Given three vectors, return all numbers which all three have in common
///
/// # Arguments
/// - 'vec1' - A vector with numbers <i32> in it
/// - 'vec2' - A vector with numbers <i32> in it
/// - 'vec3' - A vector with numbers <i32> in it
fn common_numbers(vec1: &Vec<i32>, vec2: &Vec<i32>, vec3: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = vec1.iter().cloned().collect();
    let set2: HashSet<_> = vec2.iter().cloned().collect();
    let set3: HashSet<_> = vec3.iter().cloned().collect();

    // HashSet has a intersection variable, that gets us all elements which are in both sets
    // .cloned() = clone the values, because else it would still reference the old values
    // .collect::<T>()  = a method to converte an iterater into a specific type
    // <HashSet<_>>() = the hasSet type, the underscore is a type inference placeholder
    let intersection_set = set1.intersection(&set2).cloned().collect::<HashSet<_>>()
        .intersection(&set3).cloned().collect::<HashSet<_>>();

    intersection_set.into_iter().collect()
}

///A Sudoku consists of several subgrids. This function returns the top left and bottom right
/// coordinates of the subgrid which contains the location (xco, yco)
///
/// # Arguments
/// 'xco' - The column coordinate of the spot we want to check
/// 'yco' - The row coordinate of the spot we want to check
fn get_subgrid_coor(xco : i32, yco:i32) -> (i32, i32, i32, i32){
    let x1 = (xco / 3) * 3;
    let y1 = (yco / 3) * 3;
    let x2 = x1 + 3;
    let y2 = y1 + 3;

    let coordinates = (x1, y1, x2, y2);
    coordinates
}

/// Given a location on a sudoku (x, y), return all of the numbers which can still be put inside
/// the field, whilst still following the rules of sudoku (no duplicates in row, column or subgrid)
///
///  # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
/// 'xco' - the column coordinate of the spot we want to check
/// 'yco' - the row coordinate of the spot we want to check
fn get_all_missing_numbers(sudoku: &Vec<Vec<i32>>, xco : i32, yco:i32) -> Vec<i32>{

    // get the values of the row and column
    let row = get_row(&sudoku, yco);
    let column = get_column(&sudoku, xco);

    // get the values of the subgrid
    let coor = get_subgrid_coor(xco,yco);
    let subgrid = get_subgrid(&sudoku, coor.1, coor.3, coor.0, coor.2);

    let missing_numbers_column = find_missing_numbers(&column);
    let missing_numbers_row = find_missing_numbers(&row);
    let missing_numbers_subgrid = find_missing_numbers(&subgrid);

    let missing_numbers_total = common_numbers(&missing_numbers_row, &missing_numbers_column, &missing_numbers_subgrid);
    missing_numbers_total
}

/// Returns a vector containing the locations of all empty grids inside of a sudoku
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn get_all_empty_fields(sudoku: &Vec<Vec<i32>>) -> Vec<(usize, usize)>{
    let mut empty_spots = vec![];

    let n = sudoku.len();
    let m = sudoku[0].len();

    for i in 0..n {
        for j in 0..m {
            if sudoku[i][j] == 0{
                empty_spots.push((j, i));   // column and row
            }
        }
    }
    empty_spots
}

///Returns True/False based on whether the current sudoku has all its spaces filled in, and follows
/// all of the rules (no duplicate numbers in rows, columns and subgrids)
///
/// # Arguments
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
fn check_if_sudoku_solved(sudoku: &Vec<Vec<i32>>) -> bool{

    // Check if there are any open spots left. If yes, we return false, this hasn't been solved
    let empty_loc = get_all_empty_fields(&sudoku);
    if empty_loc.len() > 0{
        return false;
    }

    // Check all rows, columns and subgrids for valid answers.
    let n = sudoku.len();                              // height sudoku
    let m = sudoku[0].len();                           // width sudoku
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];  // the numbers we want to check for

    // check all rows
    for i in 0..n {
        let row = get_row(&sudoku, i as i32);
        let all_numbers_present = numbers.iter().all(|&num| row.contains(&num));
        if !all_numbers_present{
            return false;
        }
    }

    // check all columns
    for i in 0..m{
        let column = get_column(&sudoku, i as i32);
        let all_numbers_present = numbers.iter().all(|&num| column.contains(&num));
        if !all_numbers_present{
            return false;
        }
    }

    // check all grids
    for i in 0..sqrt(m){
        for j in 0..sqrt(n){
            let subgrid = get_subgrid(&sudoku, (i * 3) as i32,  ((i + 1) * 3) as i32, (j * 3) as i32,((j + 1) * 3) as i32);
            let all_numbers_present = numbers.iter().all(|&num| subgrid.contains(&num));
            if !all_numbers_present{
                return false;
            }
        }
    }

    // if all conditions are met, return true.
    true
}


/// Generates and returns a sudoku which has been completely filled in
///
/// # Arguments
/// 'width' - The total width of the sudoku we want to generate (for a standard sudoku  = 9)
/// 'height' - The total height of the sudoku we want to generate (for a standard sudoku = 9)
fn generate_full_sudoku(width : usize, height : usize) -> Vec<Vec<i32>>{
    // This will generate a sudoku which is completely filled in and valid


    // usize = unsigned integer
    let mut sudoku: Vec<Vec<i32>> = vec![vec![0; width]; height];

    // get the numbers one through nine in random order
    let mut numbers: Vec<i32> = (1..10).collect();
    numbers.shuffle(&mut rand::thread_rng());

    // Fill in the sudoku
    sudoku = fill_row(sudoku, &numbers, 0, 0);
    sudoku = fill_row(sudoku, &numbers, 1, 3);
    sudoku = fill_row(sudoku, &numbers, 2, 6);
    sudoku = fill_row(sudoku, &numbers, 3, 1);
    sudoku = fill_row(sudoku, &numbers, 4, 4);
    sudoku = fill_row(sudoku, &numbers, 5, 7);
    sudoku = fill_row(sudoku, &numbers, 6, 2);
    sudoku = fill_row(sudoku, &numbers, 7, 5);
    sudoku = fill_row(sudoku, &numbers, 8, 8);

    // flip all of the rows within the sub grids
    sudoku = flip_all_rows(sudoku);

    // flip all of the columns within the sub grids
    sudoku = flip_all_columns(sudoku);

    // flip all large grid rows (for example, flip all rows with [0,1,2] with [6,7,8])
    sudoku = flip_grid_rows(sudoku);

    // flip all large grid columns (for example, flip all columns with [0,1,2] with [6,7,8])
    sudoku = flip_grid_columns(sudoku);

    // randomly rotate 0, 90, 180, 270 degrees
    random_rotate(&mut sudoku);
    // print_sudoku(&sudoku);

    // return the two dimensional array
    sudoku

}

/// Solves a sudoku, and returns True if it can be solved and False if it can't
///
/// # Arguments
/// * `sudoku_check` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid we want to solve
fn solve_sudoku(sudoku_check : &Vec<Vec<i32>>) -> bool{

    let mut sudoku_to_solve: Vec<Vec<i32>> = sudoku_check.clone();

    // for each empty spot, see if there is only 1 other number we can fill in. If so, we will
    // recheck all empty spots after we have filled it in.
    loop {
        let mut found:bool = false;
        let all_empty_loc = get_all_empty_fields(&sudoku_to_solve);
        for loc in all_empty_loc.iter() {
            if sudoku_to_solve[loc.1][loc.0] == 0 {
                // find out how many numbers we can get
                let all_missing_numbers = get_all_missing_numbers(&sudoku_to_solve, loc.0 as i32 ,loc.1 as i32);
                if all_missing_numbers.len() == 1 {
                    sudoku_to_solve[loc.1][loc.0] = all_missing_numbers[0];
                    found = true;
                }
            }
        }
        if !found{
            break
        }
    }

    sudoku_to_solve[0][0] = 0;

    // check if we need to do the forward propagations method
    let all_empty_loc = get_all_empty_fields(&sudoku_to_solve);
    if all_empty_loc.len()>0{

        let mut i :i32 = -1;
        loop{
            i+=1;

            let xloc = all_empty_loc[i as usize].0;
            let yloc = all_empty_loc[i as usize].1;
            let var_check = sudoku_to_solve[yloc][xloc];

            // we have gone of the edge, so we must take a step back
            if var_check == 9{
                sudoku_to_solve[yloc][xloc] = 0;
                i -= 2;
                if i < -1{
                    break;
                }
            }
            else{
                sudoku_to_solve[yloc][xloc] += 1;
                let valid = is_loc_valid(&sudoku_to_solve, xloc as i32, yloc as i32);
                if !valid{
                    i -= 1;
                }
            }

            if i>=(all_empty_loc.len()-1) as i32{
                break;
            }
        }
    }

    // check if we solved the sudoku
    let solved = check_if_sudoku_solved(&sudoku_to_solve);

    solved
}

/// Returns a a sudoku with empty spaces that we can solve, based on a filled in example
///  # Arguments
///
/// * `filled_sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid. This is completely filled in
/// 'num_to_delete' - How many fields we want to make empty in our new sudoku
fn generate_sudoku_to_solve(filled_sudoku : &Vec<Vec<i32>>, num_to_delete: i32) -> Vec<Vec<i32>>{

    // copy the filled sudoku
    let mut sudoku_to_solve: Vec<Vec<i32>> = filled_sudoku.clone();

    // start removing values
    let n = sudoku_to_solve.len();                              // height sudoku
    let m = sudoku_to_solve[0].len();                           // width sudoku

    let mut num_deleted = 0;                    // the number of grids we have deleted
    while num_deleted < num_to_delete {

        let mut old_val = 0;
        let mut xco = 0;
        let mut yco = 0;
        loop {

            xco = generate_random_number(0..m as i32);
            yco = generate_random_number(0..n as i32);
            if sudoku_to_solve[yco as usize][xco as usize] !=0{
                old_val = sudoku_to_solve[yco as usize][xco as usize];
                sudoku_to_solve[yco as usize][xco as usize] = 0;
                break;
            }
        }

        // we try to solve, if we can, we will leave it removed
        let solved = solve_sudoku(&sudoku_to_solve);
        if !solved{
            //reset and try again.
            sudoku_to_solve[yco as usize][xco as usize]=old_val;
        }
        else {
            num_deleted += 1;
        }
    }
    sudoku_to_solve
}



/// Prints a Sudoku grid represented by a 2D vector of integers.
///
/// # Arguments
///
/// * `sudoku` - A reference to a 2D vector (`&Vec<Vec<i32>>`) representing the Sudoku grid.
///
/// # Example
///
/// ```
/// let sudoku_grid = vec![
///     vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
///     vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
///     vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
///     vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
///     vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
///     vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
///     vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
///     vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
///     vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
/// ];
///
/// print_sudoku(&sudoku_grid);
/// ```
///
/// This will print the Sudoku grid to the console, with empty cells represented by spaces.
///
/// ```
///5   3           7
///6           1   9   5
///    9   8                   6
///8               6               3
///4           8       3           1
///7               2               6
///   6                   2   8
///         4   1   9           5
///         8           7   9
///
/// ```
///
/// Note: In the printed output, empty cells are represented by spaces.
///
/// # Panics
///
/// This function does not panic under normal circumstances.
///
/// # Safety
///
/// This function assumes that the input Sudoku grid is a valid representation.
fn print_sudoku(sudoku : &Vec<Vec<i32>>) {
    for row in sudoku {
        for element in row {
            if *element==0{             // get value from reference
                print!("{:4}", " ")
            }
            else {
                print!("{:4}", element);
            }
        }
        println!();
    }
    println!();
}



fn main() {

    // Create a filled in sudoku
    let sudoku = generate_full_sudoku(9, 9);
    println!("Filled sudoku");
    print_sudoku(&sudoku);

    // Start the process of creating an non filled
    let sudoku_to_solve = generate_sudoku_to_solve(&sudoku, 50);
    println!("To Solve Sudoku");
    print_sudoku(&sudoku_to_solve);

}
