fn main(){
    //this declared variable is assigned the value of a string
    //this means rust compiler is in the process of writing this function
    let x = "pass this to a value"; 
    //the function is meant to output a value of strings
    //in this case it is being used to read the declared function x
    println!("{x}");
    //write
    let x = "       {x}";
    //read
    println!("{x}");
    let some_variable = 3;
    // some_variable is being used in an arithmetic operation
    let some_variable = some_variable + 4;
    //declaring a variable _var, that is a declaration and not dependent
    //on any specific use unless stated explicitly
    let _your_variable = "move this value";
    let _my_variable = "move this value to your_variable";
    //in 'layman's' term these variables are meant to be stored
    //the variables are stored and then used by a declaration 
    //for which we have it as an output of a string
    let your_variable = _my_variable;
    println!("{your_variable}");
    let my_variable = _your_variable;
    println!("{my_variable}");
    
    {
        let mut y = "another value";
        println!("{y}");
        //mutable value
        y = "        {y}";
        println!("{y}");
    }

    {
        //this is to demonstrate that a declaration can always be 
        //assigned if and only if the declared variable is used within
        //the program
        let your_variable = "pass on";
        let _your_variable = your_variable;
        let my_variable = _your_variable;
        println!("{my_variable}");
    }

    {
        //this to show that the prefixed declaration isn't necessary
        let my_variable = "pass on again";
        let your_variable = _my_variable;
        println!("{your_variable}");
    }

    println!("{some_variable}");

}
