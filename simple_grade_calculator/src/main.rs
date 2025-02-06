use std::io;

fn main() {
    // Create a mutable array to store grades
    let mut subject_grades: [f32; 5] = [0.0; 5];

    // Input grades for each subject
    for i in 0..5 {
        println!("Enter grade for subject {}: ", i + 1);
        
        let mut grade = String::new();
        
        io::stdin()
            .read_line(&mut grade)
            .expect("Failed to read line");
        
        // Parse the input into f32, handle potential errors
        match grade.trim().parse::<f32>() {
            Ok(parsed_grade) => {
                subject_grades[i] = parsed_grade;
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                return; // Or you could continue the loop
            }
        }
    }

    let average_grade = calculate_average(subject_grades);
    println!("The average grade is: {} points", average_grade);  

    // Grade classification logic remains the same
    if average_grade >= 90.0 {
        println!("You got an A");
    } else if average_grade >= 85.0 {
        println!("You got a A-");
    } else if average_grade >= 80.0 {
        println!("You got a B+");
    } else if average_grade >= 75.0 { 
        println!("You got a B");
    } else if average_grade >= 70.0 {
        println!("You got an B-");
    } else if average_grade >= 65.0 {
        println!("You got a C+");
    } else if average_grade >= 60.0 {
        println!("You got a C");
    } else if average_grade >= 55.0 {
        println!("You got a C-");
    } else if average_grade >= 50.0 {
        println!("You got a D+");
    } else if average_grade >= 45.0 {
        println!("You got an D");
    }
    else if average_grade >= 40.0 {
        println!("You got a D-");
    } else {
        println!("You got an E");
    }
}

fn calculate_average(grade: [f32; 5]) -> f32 {
    let sum = grade[0] + grade[1] + grade[2] + grade[3] + grade[4];
    sum / 5.0
}