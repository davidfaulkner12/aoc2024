#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct FileId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BlockIndex(usize);

struct File {
    id: FileId,
    blocks: Vec<BlockIndex>,
}

#[derive(Debug, Default)]
struct Disk {
    blocks: Vec<(BlockIndex, FileId)>,
    free: Vec<(BlockIndex, usize)>,
}

fn checksum(disk: &Disk) -> usize {
    disk.blocks
        .iter()
        .map(|(block_index, file_id)| block_index.0 * file_id.0)
        .sum()
}

fn defrag(disk: &mut Disk) {
    // Assume that we have a max here
    let max_idx = disk.blocks[disk.blocks.len() - 1].0;

    let mut forward_idx = 0;
    let mut backwards_idx = disk.blocks.len() - 1;
    let mut backwards_block_idx = max_idx;

    for i in 0..=max_idx.0 {
        if i > backwards_block_idx.0 {
            break;
        }
        // i is not free space
        if disk.blocks[forward_idx].0 .0 == i {
            // println!("Block {} has file {:?}", i, disk.blocks[forward_idx]);
            forward_idx += 1;
        } else {
            //println!(
            //    "Block {} is free, shifting block {:?}",
            //    i, disk.blocks[backwards_idx]
            //);

            disk.blocks[backwards_idx].0 = BlockIndex(i);
            backwards_idx -= 1;
            backwards_block_idx = disk.blocks[backwards_idx].0;
        }
    }
}

fn smart_defrag(disk: &mut Disk) {
    let mut files: Vec<(FileId, BlockIndex, usize, usize)> = Vec::new();

    let mut files_index = usize::MAX;

    for (i, block) in disk.blocks.iter().enumerate() {
        if FileId(files_index) != block.1 {
            files_index = block.1 .0;
            files.push((block.1, block.0, 1, i));
        } else {
            files[files_index].2 += 1
        }
    }

    files.reverse();

    // We try each file only once
    for (file, start, size, disk_vec_index) in files {
        //println!("Looking for a fit with {file:?} {start:?} {size:?} {disk_vec_index:?}");

        // Look for a fit
        let space =
            disk.free
                .iter()
                .enumerate()
                .find_map(|(free_vec_index, (free_index, free_size))| {
                    //println!("Considering free slot at {free_index:?} (size: {free_size:?})");
                    // Only look at left
                    if free_index.0 > start.0 {
                        None
                    } else if size <= *free_size {
                        Some((free_vec_index, *free_index, *free_size))
                    } else {
                        None
                    }
                });

        if let Some((free_vec_index, free_index, free_size)) = space {
            // println!("Found a space!");
            // Carefully maintain all the invariants here
            // Adjust the free size
            let new_free_index = disk.free[free_vec_index].0 .0 + size;
            disk.free[free_vec_index] = (BlockIndex(new_free_index), free_size - size);

            for i in 0..size {
                disk.blocks[i + disk_vec_index] = (BlockIndex(free_index.0 + i), file);
            }
        }
    }
}

fn parse(data: &str) -> Disk {
    let mut cur_index = 0;
    let mut cur_id = 0;
    let mut disk = Disk::default();

    for s in data.as_bytes().chunks(2) {
        let filesize = s[0] - b'0';
        for _ in 0..filesize {
            disk.blocks.push((BlockIndex(cur_index), FileId(cur_id)));
            cur_index += 1;
        }
        cur_id += 1;

        if s.len() > 1 && s[1] >= b'0' {
            let freespace = s[1] - b'0';
            disk.free.push((BlockIndex(cur_index), freespace as usize));

            cur_index += freespace as usize;
        }
    }

    disk
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, convert::identity, fs};

    use super::*;

    const SIMPLE_DATA: &str = "12345";
    const TEST_DATA: &str = "2333133121414131402";

    fn str_to_disk(s: &str) -> Vec<(BlockIndex, FileId)> {
        s.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != b'.')
            .map(|(i, c)| (BlockIndex(i), FileId((*c - b'0') as usize)))
            .collect()
    }

    #[test]
    fn test_simple_defrag() {
        // We enforce an ordered invariant here
        let mut disk = Disk {
            blocks: vec![
                (BlockIndex(0), FileId(0)),
                (BlockIndex(3), FileId(1)),
                (BlockIndex(4), FileId(1)),
                (BlockIndex(5), FileId(1)),
                (BlockIndex(10), FileId(2)),
                (BlockIndex(11), FileId(2)),
                (BlockIndex(12), FileId(2)),
                (BlockIndex(13), FileId(2)),
                (BlockIndex(14), FileId(2)),
            ],
            // We dont' use this in this test
            free: vec![],
        };

        defrag(&mut disk);

        disk.blocks.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
        assert_eq!(disk.blocks, str_to_disk("022111222"));
    }

    #[test]
    fn test_simple_parse() {
        let disk = parse(TEST_DATA);

        assert_eq!(
            disk.blocks,
            str_to_disk("00...111...2...333.44.5555.6666.777.888899")
        );

        assert_eq!(
            disk.free,
            vec![
                (BlockIndex(2), 3),
                (BlockIndex(8), 3),
                (BlockIndex(12), 3),
                (BlockIndex(18), 1),
                (BlockIndex(21), 1),
                (BlockIndex(26), 1),
                (BlockIndex(31), 1),
                (BlockIndex(35), 1),
                (BlockIndex(40), 0),
            ]
        );
    }

    #[test]
    fn test_sample_defrag() {
        let mut disk = parse(TEST_DATA);
        defrag(&mut disk);

        let mut sorted = disk.blocks.clone();

        sorted.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
        assert_eq!(
            sorted,
            str_to_disk("0099811188827773336446555566..............")
        );

        assert_eq!(checksum(&disk), 1928);
    }

    #[test]
    fn test_actual_problem() {
        let data = fs::read_to_string("data/day9.txt").unwrap();

        let mut disk = parse(&data);
        defrag(&mut disk);

        assert_eq!(checksum(&disk), 6299243228569);
    }

    #[test]
    fn test_smart_defrag() {
        let mut disk = parse(TEST_DATA);

        smart_defrag(&mut disk);

        assert_eq!(checksum(&disk), 2858);
    }

    #[test]
    fn test_smart_defrag_problem() {
        let data = fs::read_to_string("data/day9.txt").unwrap();
        let mut disk = parse(&data);

        smart_defrag(&mut disk);
        assert_eq!(checksum(&disk), 6326952672104);
    }
}
