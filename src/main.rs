use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

fn read_files(filename: &str) -> io::Result<Vec<i32>>{
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let numeros: Vec<i32> = reader
        .lines()
        .filter_map(|line| line.ok()?.parse().ok())
        .collect();

    Ok(numeros)
}

fn bubble_sort(vetor: &mut Vec<i32>){
    let tamanho_vetor = vetor.len();
    for i in 0..tamanho_vetor{
        for j in 0..tamanho_vetor-i-1{
            if vetor[j] > vetor[j+1]{
                vetor.swap(j, j+1);
            }
        }
    }
}

fn save_files(numbers: &Vec<i32>, filename: &str) -> io::Result<()>{
    let path = Path::new(filename);
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    for &number in numbers{
        writeln!(writer, "{}", number)?;
    }

    Ok(())
}

fn main()-> io::Result<()>{
    let file_path = "numeros.txt";
    let mut numbers = read_files(file_path)?;
    println!("Numeros antes da ordenação: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Numeros depois da ordenação {:?}", numbers);

    numbers.iter().for_each(|&num| print!("{}", num));
    println!();

    save_files(&numbers, "numeros_ordenados.txt")?;

    Ok(())
}
