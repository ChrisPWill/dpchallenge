#![feature(convert)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;

fn main() 
{
    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut fl_str = String::new();
    let _ = reader.read_line(&mut fl_str);
    let mut words = fl_str.as_str().split_whitespace();

    let width  = words.next().unwrap().parse::<usize>().unwrap();
    let height = words.next().unwrap().parse::<usize>().unwrap();

    // Init canvas
    let mut canvas = Vec::with_capacity(width);
    for _ in 0..height 
    {
        let mut row = Vec::with_capacity(height);
        for _ in 0..width 
        {
            row.push(0);
        }
        canvas.push(row);
    }
    
    // Init counts
    let mut counts = HashMap::new();
    counts.insert(0, width*height);

    // Place sheets
    for line in reader.lines() 
    {
        let line = line.unwrap().clone();
        let mut words = line.as_str().split_whitespace();
        let c = words.next().unwrap().parse::<usize>().unwrap();
        let x = words.next().unwrap().parse::<usize>().unwrap();
        let y = words.next().unwrap().parse::<usize>().unwrap();
        let w = words.next().unwrap().parse::<usize>().unwrap();
        let h = words.next().unwrap().parse::<usize>().unwrap();
        place_sheet(&mut canvas, &mut counts, c, x, y, w, h);
    }

    // Print canvas
    for row in canvas.iter()
    {
        for c in row.iter() {
            print!("{}", c);
        }
        println!("");
    }

    // Print counts (unsorted, should sort)
    let mut counts_sorted: Vec<(&usize, &usize)> = counts.iter().collect();
    counts_sorted.sort();
    for &(c, count) in counts_sorted.iter() {
        println!("{} {}", c, count);
    }
}

fn place_sheet(canvas: &mut Vec<Vec<usize>>, counts: &mut HashMap<usize, usize>,
               c: usize, x: usize, y: usize, w: usize, h: usize) 
{
    if !counts.contains_key(&c) 
    {
        counts.insert(c, 0);
    }
    for j in y..y+h 
    {
        let row = canvas.get_mut(j).unwrap();
        for i in x..x+w 
        {
            let old_c = row.get_mut(i).unwrap();
            *counts.get_mut(old_c).unwrap() -= 1;
            *old_c = c;
            *counts.get_mut(&c).unwrap() += 1;
        }
    }
}
