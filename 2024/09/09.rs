use std::{collections::{HashMap, LinkedList}, io::{stdin, Read}};

fn read_line() -> (Vec<Option<i32>>, LinkedList<(usize, i32)>) {
    let mut nums: Vec<u32> = Vec::new();
    for line in stdin().lines() {
        nums = line.expect("fail read").chars()
            .filter_map(|c| c.to_digit(10)).collect();
    }

    let mut emptyblocks = LinkedList::new();
    let mut mem: Vec<Option<i32>> = Vec::with_capacity(nums.len() * 9);
    for (fileid, i) in (0..nums.len()).step_by(2).enumerate() {
        let (nwritten, nempty) = (nums[i], if i+1 < nums.len() { nums[i+1] } else { 0 });
        for _ in 0..nwritten { mem.push(Some(fileid as i32)); }
        for _ in 0..nempty { mem.push(None); }
        if nempty > 0 {
            let empty_pos = mem.len() as i32 - nempty as i32;
            emptyblocks.push_back((empty_pos as usize, nempty as i32));
        }
    }
    (mem, emptyblocks)
}

fn compact1(memory: &mut Vec<Option<i32>>) {
    let mut start = 0;
    let mut end = memory.len() - 1;
    while start < end {
        if memory[start].is_some() {
            start += 1;
        } else {
            if memory[end].is_some() {
                memory[start] = memory[end];
                memory[end] = None;
            }
            end -= 1;
        }
    }
}

fn print_memory(memory: & Vec<Option<i32>>) {
    for m in memory {
        match m {
            Some(x) => print!("{x}"),
            None => print!(".")
        }
    }
    println!();
}

fn compact2(memory: &mut Vec<Option<i32>>, emptyblocks: &mut LinkedList<(usize, i32)>) {
    let mut end = memory.len() - 1;
    while end > 0 {
        if memory[end].is_some() {
            let mut block_start = end;
            let block_end = end;
            while memory[block_end] == memory[block_start] {
                block_start -= 1;
                if block_start <= 0 { return; }
            }
            block_start += 1;

            let block_size = (block_end - block_start) + 1;
            for (pos, size) in emptyblocks.iter_mut() {
                if *pos > block_start { break; }
                if *size as usize >= block_size {
                    for i in 0..block_size {
                        memory[*pos + i] = memory[block_start + i];
                        memory[block_start + i] = None;
                    }
                    *pos += block_size;
                    *size -= block_size as i32;
                    break;
                }
            }
            end -= block_size;
        } else {
            end -= 1;
        }
    }
}

fn checksum(memory: & Vec<Option<i32>>) -> i64 {
    memory.iter().enumerate()
        .map(|(i, x)| i as i64 * x.unwrap_or(0) as i64).sum::<i64>()
}

pub fn main() {
    let (memory, emptyblocks) = read_line();
    {
        let mut memory_tmp = memory.clone();
        compact1(&mut memory_tmp);
        println!("checksum = {:?}", checksum(&memory_tmp));
    }
    {
        let mut memory_tmp = memory.clone();
        let mut emptyblocks_tmp = emptyblocks.clone();
        compact2(&mut memory_tmp, &mut emptyblocks_tmp);
        println!("checksum = {:?}", checksum(&memory_tmp));
    }
}

