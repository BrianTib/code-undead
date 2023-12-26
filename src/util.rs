use std::{
    convert::From,
    fs::{self, File},
    io::{self, ErrorKind, BufRead, Read, Write},
    ops::{Bound, RangeBounds},
    path::Path,
    str::FromStr,
    time::SystemTime
};

pub fn collect_input_str(prompt: Option<&str>) -> Result<String, io::Error> {
    collect_input::<String>(prompt)
}

pub fn collect_input<T>(prompt: Option<&str>) -> Result<T, io::Error> 
    where T: FromStr 
{
    if let Some(prompt) = prompt {
        println!("{prompt}");
    }

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    // Clear the console
    print!("{}[2J", 27 as char);

    input.trim()
        .to_string()
        .parse::<T>()
        .or(Err(io::Error::from(ErrorKind::Other)))
}

/// Sends messages as options onto the consoel and returns the index of the one chosen
pub fn collect_with_options(text: &str, options: &[&str]) -> Result<usize, &'static str> {
    if options.len() <= 1 {
        return Err("Not enough options were provided");
    }

    // Clear the console
    //print!("{}[2J", 27 as char);
    let mut msg = format!("{} (Respond with {}-{})\n", text, 1, options.len());
    for (i, option) in options.iter().enumerate() {
        msg.push_str(&format!("{}-) {}\n", i + 1, option));
    }

    println!("{msg}");

    // Collect the option from the user casted as the appropriate type
    let response = collect_input::<usize>(None);
    if let Ok(response) = response {
        // Check that the number is within the given range
        if (1..=options.len()).contains(&response) {
            return Ok(response - 1);
        }
    }

    println!("That doesnt look like a valid esponse. Please try again");
    collect_with_options(text, options)
}

pub fn random() -> f32 {
    // Create a seed from UNIX EPOCH until now in nanoseconds
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        // Reversing the bits of the seed can help improve randomness.
        // This is particularly useful when the seed is derived from time,
        // as it helps to disperse the bits and avoid patterns related to
        // the system's clock.
        .reverse_bits();

    // Reduce the size of the seed to avoid overflow
    // This technically reduces our randomness but by a negligible amount
    let seed = seed / 10e+28_f64 as u128;
    // Divide by the closest maximum value depending on the amount of digits
    let max = 10u128.pow(seed.to_string().len() as u32) - 1;

    seed as f32 / max as f32
}

// Returns a random number from within a range
pub fn random_range<T>(range: T) -> f32
where
    T: RangeBounds<f32>
{
    // Our randomly generated scalar (0.0 - 1.0)
    let r = random();

    // Extract the inner f32 from the ranges, whether theyre inclusive or not
    let min = match range.start_bound() {
        Bound::Included(&x) => x.into(),
        Bound::Excluded(&x) => x.into(),
        Bound::Unbounded => f32::MIN,
    };

    let max = match range.end_bound() {
        Bound::Included(&x) => x.into(),
        Bound::Excluded(&x) => x.into(),
        Bound::Unbounded => f32::MAX,
    };

    min + (r * (max - min))
}

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file()
    } else {
        false
    }
}

/// Loads the contents of a file as a string
pub fn load_from_file(file_path: &str) -> Result<String, io::Error> {
    if !file_exists(file_path) {
        let _ = save_to_file(file_path, "");
    }

    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Saves the content of a file from a string
pub fn save_to_file(file_path: &str, data: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Loads a file's binary data
pub fn load_from_file_bin(file_path: &str) -> Result<Vec<u8>, io::Error> {
    if !file_exists(file_path) {
        let _ = save_to_file_bin(file_path, &Vec::new());
    }

    fs::read(file_path)
}

/// Saves a buffer to a file
pub fn save_to_file_bin(file_path: &str, buffer: &Vec<u8>) -> Result<(), io::Error> {
    fs::write(file_path, buffer)?;
    Ok(())
}