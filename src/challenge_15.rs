use std::iter::Sum;
const HASH_FACTOR: u32 = 17;
const REMAINDER_DIVIDER:u32 = 256;
struct Step<'a>(&'a str);

impl <'a> Step<'a> {
    fn hash(&self)-> u32{
        let mut current_value = 0;
        for c in self.0.chars() {
            current_value += c as u32;
            current_value*=HASH_FACTOR;
            current_value%= REMAINDER_DIVIDER;
        }
        current_value
    }
}

pub fn step_1(input_content: &str) -> String {
    let steps:u32 = input_content.split(',').map(|step| Step(step)).map(|step| step.hash()).sum();
    steps.to_string()
}


#[test]
fn should_return_valid_hash(){
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(step_1(input), "1320")
}