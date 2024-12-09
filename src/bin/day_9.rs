use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_9").unwrap();
    let disk_map_digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let disk_map = DiskMap::new(&disk_map_digits);
    let mut disk = Disk::new(disk_map.clone());
    let mut contiguous_disk = Disk::new(disk_map);

    disk.fragment();
    println!("Noncontiguous Checksum: {}", disk.checksum());

    contiguous_disk.contiguous_fragment();
    println!("Contiguous Checksum: {}", contiguous_disk.checksum());
}

#[derive(Debug, Copy, Clone)]
enum DiskElement {
    File(usize, u8, u16),
    FreeSpace(usize, u8),
}

#[derive(Debug, Clone)]
struct DiskMap {
    elems: VecDeque<DiskElement>,
}

impl DiskMap {
    pub fn new(input: &[u8]) -> Self {
        let mut blocks = VecDeque::new();

        let mut id = 0;
        let mut mem_ptr = 0;
        for (i, v) in input.iter().enumerate() {
            if *v == 0 {
                continue;
            }

            let block = match i % 2 {
                0 => {
                    let block = DiskElement::File(mem_ptr, *v, id);
                    id += 1;
                    mem_ptr += *v as usize;
                    block
                }
                1 => {
                    let block = DiskElement::FreeSpace(mem_ptr, *v);
                    mem_ptr += *v as usize;
                    block
                }
                _ => panic!("Modulus Operator Failed Somehow!"),
            };

            blocks.push_back(block);
        }

        Self { elems: blocks }
    }

    pub fn disk_size(&self) -> usize {
        let mut size = 0;

        for elem in &self.elems {
            let len = match *elem {
                DiskElement::FreeSpace(_ptr, len) => len,
                DiskElement::File(_ptr, len, _id) => len,
            };
            size += len as usize;
        }

        size
    }
}

#[derive(Debug, Clone)]
struct Disk {
    data: Box<[u16]>,
    map: DiskMap,
}

impl Disk {
    pub fn new(map: DiskMap) -> Self {
        let mut data = vec![u16::MAX; map.disk_size()].into_boxed_slice();

        for elem in &map.elems {
            if let DiskElement::File(ptr, len, id) = *elem {
                for offset in 0..len as usize {
                    data[ptr + offset] = id;
                }
            }
        }

        Self { data, map }
    }

    pub fn fragment(&mut self) {
        'fragment: for from_ptr in (0..self.data.len()).rev() {
            if self.data[from_ptr] != u16::MAX {
                let mut found_spot = false;
                for to_ptr in 0..from_ptr {
                    if self.data[to_ptr] == u16::MAX {
                        self.data[to_ptr] = self.data[from_ptr];
                        self.data[from_ptr] = u16::MAX;
                        found_spot = true;
                    }
                }

                if !found_spot {
                    break 'fragment;
                }
            }
        }
    }

    pub fn contiguous_fragment(&mut self) {
        for potential_file_index in (0..self.map.elems.len()).rev() {
            if let DiskElement::File(file_ptr, file_len, id) = self.map.elems[potential_file_index]
            {
                'find_space: for potential_space_index in 0..potential_file_index {
                    match self.map.elems[potential_space_index] {
                        DiskElement::FreeSpace(free_ptr, free_len)
                            if free_ptr < file_ptr && free_len >= file_len =>
                        {
                            for offset in 0..file_len {
                                self.data[free_ptr + offset as usize] = id;
                            }

                            for offset in 0..file_len {
                                self.data[file_ptr + offset as usize] = u16::MAX;
                            }

                            self.map.elems[potential_file_index] =
                                DiskElement::FreeSpace(file_ptr, file_len);
                            self.map.elems[potential_space_index] = DiskElement::FreeSpace(
                                free_ptr + file_len as usize,
                                free_len.saturating_sub(file_len),
                            );

                            break 'find_space;
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn checksum(&self) -> u64 {
        let mut checksum = 0;

        for (i, block) in self.data.iter().enumerate() {
            if *block != u16::MAX {
                checksum += i as u64 * (*block) as u64;
            }
        }

        checksum
    }
}
