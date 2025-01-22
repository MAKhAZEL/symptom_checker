use std::io;

fn main() {
    println!("Welcome to the Symptom Checker by Hazel.M");
    println!("Enter a symptom to see possible conditions or type 'exit' to quit.");
    
    // The Symptom-To-Condition mappings
    let symptoms = vec![
	("headache", "Possible conditions: Migraine, Stress, Dehydration"),
	("fever", "Possible conditions: Flu, Infection, COVID-19"),
	("cough", "Possible conditions: Cold, Bronchitis, COVID-19"),
    ];
    
    loop {
	// This gets he user input
	println!("Enter a symptom:");
	let mut input = String::new();
	io::stdin()
	    .read_line(&mut input)
	    .expect("Failed to read input");
	let symptom = input.trim().to_lowercase();

	// To exit the condition
	if symptom == "exit" {
	    println!("Goodbye! Stay healthy.");
	    break;
	}

	// Matching the symptom
	let mut found = false;
	for &(key, condition) in &symptoms {
	    if symptom == key {
	        println!("{}", condition);
	        found = true;
	        break;
	    }
	}
	
	if !found {
	    println!("Symptom not recognized. Please try again.");
	}
    }
 
}
