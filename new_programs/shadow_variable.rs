//the key word let allows to create a new variable
//the variable will have a type, to declare the type
//of the variable more than once is a form of shadowing

//binding is done at two levels while the scope is defined explicitly
//two bindings occur when two new variables are created. 
//x as defined in the inner scope is new variable, however share the same
//variable name.
fn main() {
    let x = "This is my name";
    println!("{x}");
    {
        let x = "This is some other name";
        println!("{x}");
    }

//We can effectively show that the variable is being shadowed by using the same
//name and different data types. Since y is assigned to integer will it change the new
//variable to a different type?

    let y = 1;
    println!("{y}");
    
    {
        let y = "y + y";
        println!("{y}");
    }
}

floating point numbers are scalar types signed or unsigned
