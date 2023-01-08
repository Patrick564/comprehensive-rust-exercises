fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix = [[0; 3]; 3];

    for row in 0..3 {
        for col in 0..3 {
            new_matrix[col][row] = matrix[row][col];
        }
    }

    new_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        print!("|");
        for col in 0..3 {
            print!(" {} ", row[col]);
        }
        print!("|\n");
    }
}

pub fn day01_morning() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
