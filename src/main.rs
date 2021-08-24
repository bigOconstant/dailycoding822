use std::vec;

fn overlapping_intervals( input: &mut Vec<[i32;2]> ) -> Vec<[i32;2]>{
    let mut vec = Vec::new();

    for i in 0..input.len() {
        let mut found_merged = false;
        for j in 0..input.len() {
            
            if j != i {
                if input[j][0] < input[i][0] && input[j][1] > input[i][1]{
                    found_merged = true;
                } 
            }
        }
        if !found_merged {
            vec.push(input[i]);
        }

    } 
    return vec;
}



#[test]
fn check_answer_validity() {
    let mut vec = vec![[1,3],[5,8],[4,10],[20,25],[6,7]];
    let output = overlapping_intervals(&mut vec);

    assert_eq!(output,vec![[1,3],[4,10],[20,25]]);
}

fn main() {
     let mut vec = vec![[1,3],[5,8],[4,10],[20,25],[6,7]];
     let output = overlapping_intervals(&mut vec);
     for i in 0..output.len() {
        println!("{},{}",output[i][0],output[i][1]);
     }

}
