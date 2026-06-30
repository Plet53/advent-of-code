use std::cmp::Ordering;
use std::fs;
use std::env;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String;
  match args.next() {
    Some(arg) => filename = arg,
    None => {
      panic!("remember the filename")
    },
  };

  let rawdata = fs::read_to_string(filename).unwrap();

  day1(rawdata.clone());
  println!();
  day2(rawdata);
}

// Compact file map by fragmenting when needed.
fn day1(data: String) {
  let initial_map = create_disk_map(data);

  println!("{:?}", initial_map);

  let mut spacers = initial_map.iter().filter(|(_, index)|index.is_none());
  let mut files = initial_map.iter().filter(|(_, index)|index.is_some()).map(|(size, index)| (*size, index.unwrap()));
  let mut leftover_file: Option<(u8, usize)> = None;
  let mut compact_map: Vec<(u8, usize)> = Vec::new();

  loop {
    match files.next() {
      Some(file) => compact_map.push(file),
      None => match files.next_back() {
        Some(file) => { compact_map.push(file); break },
        None => match leftover_file {
          Some(file) => { compact_map.push(file); break },
          None => break
        }
      }
    }
    let available_space = spacers.next();
    match available_space {
      Some((0, _)) => (),
      Some((mut space, _)) => {
        while space > 0 {
          let file_to_compact = match leftover_file {
            Some(file) => {
              leftover_file = None;
              Some(file)
            },
            None => {
              match files.next_back() {
                Some(file) => Some(file),
                None => None
              }
            }
          };
          match file_to_compact {
            Some(file) => {
              match space.cmp(&file.0) {
                // Fill existing space, leave some part
                Ordering::Less => {
                  compact_map.push((space, file.1));
                  leftover_file = Some((file.0 - space, file.1));
                  space = 0;
                },
                // Fill space, track remainder.
                Ordering::Equal | Ordering::Greater => {
                  compact_map.push(file);
                  space -= file.0;
                },
              }
            }
            None => ()
          }
        }
      },
      None => ()
    }
  }

  for (size, index) in &compact_map {
    let rep = index.to_string().repeat(*size as usize);
    print!("{}", rep);
  }
  println!();

  let mut overall_index: isize = 0;
  let checksum: isize = compact_map.iter().map(|(size, file_index)| {
    overall_index += (*size) as isize;
    ((overall_index - (*size as isize))..overall_index).fold(0, |acc: isize, index: isize| { 
      acc + (index * (*file_index as isize))
    })
  }).sum();

  println!("Fragmented Checksum: {}", checksum);
}

// Defragment disk. Group like files
fn day2(data: String) {
  let initial_map = create_disk_map(data);

  let indexed_map = initial_map.iter().enumerate().map(|(index, entry)| {
    (index, vec![*entry], if entry.1.is_none() {entry.0} else {0})
  });

  let spacer_map: Vec<(usize, Vec<(u8, Option<usize>)>, u8)> = indexed_map.clone().filter(|(_, _, size)| *size > 0).collect();

  let mut compacted_map: Vec<(usize, Vec<(u8, Option<usize>)>, u8)> = indexed_map.filter(|(_, _, size)| *size == 0).rev().fold(spacer_map, |mut collection, file| {
    let mut inserted = false;
    let file_entry = file.1.first().unwrap();
    for spacer in collection.iter_mut() {
      if spacer.0 >= file.0 { break; }
      if spacer.2 >= file_entry.0 {
        spacer.2 -= file_entry.0;
        spacer.1.push(*file_entry);
        inserted = true;
        break;
      }
    }
    collection.push(
      if !inserted {
        file
      } else {
        (file.0, vec![(file_entry.0, None)], file_entry.0)
      }
    );
    collection
  });

  compacted_map.sort_by(|a, b| a.0.cmp(&b.0));

  let compact_map: Vec<(u8, Option<usize>)> = compacted_map.iter().flat_map(|(_, entries, space)| {
    let mut ret_col: Vec<(u8, Option<usize>)> = entries.iter().filter(|(_, entry)| entry.is_some()).map(|entry| *entry).collect();
    if *space > 0 {
      ret_col.push((*space, None));
    }
    ret_col
  }).collect();

  for (size, index) in &compact_map {
    let rep = match index {
      Some(val) => {
        val.to_string().chars().nth(0).unwrap()
      },
      None => '.'
    }.to_string().repeat(*size as usize);
    print!("{}", rep);
  }
  println!();

  let mut overall_index: isize = 0;
  let checksum: isize = compact_map.iter().map(|(size, file_index)| {
    overall_index += (*size) as isize;
    match file_index {
      Some(f_index) => {
        ((overall_index - (*size as isize))..overall_index).fold(0, |acc: isize, index: isize| {acc + (index * (*f_index as isize))})
      },
      None => 0
    }
  }).sum();

  println!("Unfragmented Checksum: {}", checksum);
}

// Each file entry is a contiguous set of characters.
// Tuple of Size and File Residence
fn create_disk_map(data: String) -> Vec<(u8, Option<usize>)> {
  (0..((data.len() / 2) + data.len() % 2)).map(|index| {
    let file = index * 2;
    (
      (
        data.get(file..file + 1).unwrap().parse::<u8>().unwrap(),
        Some(index)
      ),
      match data.get((file + 1)..(file + 2)) {
        Some(s) => Some((
          s.parse::<u8>().unwrap(),
          None
        )),
        None => None
      }
    )
  }).fold(Vec::new(), |mut collection, (file, spacer)|{
    collection.push(file);
    match spacer {
      Some(spacer) => { collection.push(spacer); },
      None => ()
    }
    collection
  })
}