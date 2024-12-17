pub fn _get_sample_input() -> &'static str {
    "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
}

pub fn _get_sample_input2() -> &'static str {
    "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
}

pub fn get_input() -> &'static str {
    "Register A: 53437164
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0"
}

/*
2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0

b=a%8
b=b^7
c=a/pow(2,b)
b=b^c
b=b^4
out(b%8)
a=a/8
jnz(0)
*/
