use std::io::stdin;

fn read_line() -> (Vec<Option<i32>>, Vec<(usize, usize)>) {
    let mut nums: Vec<u32> = Vec::new();
    for line in stdin().lines() {
        nums = line.expect("fail read").chars()
            .filter_map(|c| c.to_digit(10)).collect();
    }

    let mut emptyblocks = Vec::with_capacity(nums.len());
    let mut mem: Vec<Option<i32>> = Vec::with_capacity(nums.len() * 9);
    for (fileid, i) in (0..nums.len()).step_by(2).enumerate() {
        let (nwritten, nempty) = (nums[i], if i+1 < nums.len() { nums[i+1] } else { 0 });
        for _ in 0..nwritten { mem.push(Some(fileid as i32)); }
        for _ in 0..nempty { mem.push(None); }
        if nempty > 0 {
            let empty_pos = mem.len() as i32 - nempty as i32;
            emptyblocks.push((empty_pos as usize, nempty as usize));
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

fn cur_block_range(memory: & Vec<Option<i32>>, pos: usize) -> (usize, usize) {
    let mut start = pos as isize;
    let mut end = pos;
    while start >= 0 && memory[start as usize] == memory[pos] {
        start -= 1;
    }
    while end < memory.len() && memory[end] == memory[pos] {
        end += 1;
    }
    ((start + 1) as usize, (end - 1) as usize)
}

fn compact2(memory: &mut Vec<Option<i32>>, emptyblocks: &mut Vec<(usize, usize)>) {
    let mut end = memory.len() - 1;
    while end > 0 {
        if memory[end].is_none() {
            end -= 1;
            continue;
        }

        let (block_start, block_end) = cur_block_range(memory, end);
        let block_size = block_end - block_start + 1;
        for (pos, size) in emptyblocks.iter_mut() {
            if *pos >= block_start { break; }
            if *size >= block_size {
                for i in 0..block_size {
                    memory[*pos + i] = memory[block_start + i];
                    memory[block_start + i] = None;
                }
                *pos += block_size;
                *size -= block_size;
                break;
            }
        }
        end -= usize::min(block_size, end);
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

