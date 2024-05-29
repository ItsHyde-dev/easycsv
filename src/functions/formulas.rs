pub fn print_with_formula(path: String, formula: String) {}

// Should return some readable formula pattern
// The columns should be in placeholders like {{column_title}}
/*

   Ideas:
   1. Create an execution tree which will let us know the order of execution of functions
        - Find all the brackets first
        - Then tokenize everything further using bodmas

*/

struct ExecNode {
    operator: String,
    left: Box<ExecNode>,
    rigth: Box<ExecNode>,
}

fn parse_formula(formula: String) {
}

fn add(a: f64, b: f64) -> f64 {}
