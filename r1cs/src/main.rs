/// Helper functions for vector and matrics multiplication
fn matrix_multiply(matrix: &[&[i32]], vector: &[i32]) -> Vec<i32> {
    matrix.iter().map(|row| {
        row.iter().zip(vector.iter()).map(|(r,v)| r * v).sum()
    }).collect()
}

fn verify_r1cs(l_matrix: &[&[i32]], r_matrix: &[&[i32]], o_matrix: &[&[i32]], witness: &[i32]) -> bool {
    let l_result = matrix_multiply(l_matrix, witness);
    let r_result = matrix_multiply(r_matrix, witness);
    let o_result = matrix_multiply(o_matrix, witness);

    for i in 0..l_result.iter().len() {
        if l_result[i] * r_result[i] != o_result[i] {
            return false;
        }
    }
    true
}

fn example_1() {
    println!("--- Example 1: Simple Multiplication ---");
    // Witness layout: [1, out, x, y]
    // Equation: x * y = out
    
    let l: &[&[i32]] = &[&[0, 0, 1, 0]]; 
    let r: &[&[i32]] = &[&[0, 0, 0, 1]]; 
    let o: &[&[i32]] = &[&[0, 1, 0, 0]]; 

    let x = 5;
    let y = 10;
    let out = x * y;
    let witness = vec![1, out, x, y];

    let result = verify_r1cs(l, r, o, &witness);
    println!("R1CS Valid: {}", result);
}



fn main() {
    example_1();
}