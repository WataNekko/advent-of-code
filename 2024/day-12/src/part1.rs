use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct RegionId {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct RowChunk {
    plant: char,
    region_id: RegionId,
    range: Range<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn cost(self) -> u32 {
        self.area * self.perimeter
    }
}

// Consider this region while processing:
//..AAAAAAAAAA........
//..A.......AAAAAAAAAA
//..A.AAAAA.A........A
//..A.A.A.A.A.AAAA...A
//.AAAAAAAAAA.A.AAAAAA

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let mut regions: HashMap<RegionId, Region> = HashMap::new();

    let _ = input.lines().enumerate().fold(
        Default::default(),
        |(mut prev_row, mut curr_row): (Vec<_>, Vec<_>), (y, line)| {
            let row_chunks = line.chars().enumerate().chunk_by(|&(_, plant)| plant);
            let row_chunks = row_chunks
                .into_iter()
                .map({
                    |(plant, mut chunk)| {
                        let range = {
                            let (start_idx, _) = chunk.next().unwrap();
                            let end_idx = match chunk.last() {
                                Some((end_idx, _)) => end_idx,
                                None => start_idx,
                            };
                            start_idx..end_idx + 1
                        };
                        (plant, range)
                    }
                })
                .map({
                    let mut prev_row = prev_row.iter().peekable();
                    let mut merged_regions_mappings = HashMap::new();
                    let regions = &mut regions;
                    move |(plant, range)| {
                        let mut region_id = None;
                        let mut process_touching_region = |touching_region: &RowChunk| {
                            if touching_region.plant != plant {
                                return;
                            }

                            let touching_region =
                                match merged_regions_mappings.get(&touching_region.region_id) {
                                    Some(&merged_region_id) => RowChunk {
                                        region_id: merged_region_id,
                                        ..touching_region.clone()
                                    },
                                    None => touching_region.clone(),
                                };

                            let touching_perimeter = (touching_region.range.end.min(range.end)
                                - touching_region.range.start.max(range.start))
                                as u32;

                            match region_id {
                                None => {
                                    // record to regions by merging self to existing
                                    let region_id = *region_id.insert(touching_region.region_id);
                                    let region = regions.get_mut(&region_id).unwrap();

                                    region.area += range.len() as u32;
                                    region.perimeter +=
                                        (range.len() as u32 * 2 + 2) - (touching_perimeter * 2);
                                }
                                Some(region_id) => {
                                    // already recorded to regions -> merge with all remaining touching chunks
                                    if touching_region.region_id == region_id {
                                        // already merged -> remove touching perimeters
                                        let region = regions.get_mut(&region_id).unwrap();
                                        region.perimeter -= touching_perimeter * 2;
                                    } else {
                                        // merge regions
                                        let other_region =
                                            regions.remove(&touching_region.region_id).unwrap();
                                        let region = regions.get_mut(&region_id).unwrap();
                                        region.area += other_region.area;
                                        region.perimeter +=
                                            other_region.perimeter - touching_perimeter * 2;

                                        merged_regions_mappings
                                            .insert(touching_region.region_id, region_id);
                                    }
                                }
                            }
                        };

                        prev_row
                            .peeking_take_while(|chunk: &&RowChunk| chunk.range.end <= range.end)
                            .for_each(&mut process_touching_region);

                        if let Some(&last_touching_region) = prev_row
                            .peek()
                            .filter(|chunk: &&&RowChunk| chunk.range.start < range.end)
                        {
                            process_touching_region(last_touching_region);
                        }

                        let region_id = match region_id {
                            Some(id) => id,
                            None => {
                                let id = RegionId { x: range.start, y };
                                regions.insert(
                                    id,
                                    Region {
                                        area: range.len() as _,
                                        perimeter: range.len() as u32 * 2 + 2,
                                    },
                                );
                                id
                            }
                        };

                        RowChunk {
                            plant,
                            region_id,
                            range,
                        }
                    }
                });

            curr_row.extend(row_chunks);
            prev_row.clear();
            (curr_row, prev_row)
        },
    );

    let total_price = regions
        .into_values()
        .map(|region| region.cost())
        .sum::<u32>();

    Ok(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        assert_eq!(1930, process(input)?);
        Ok(())
    }
}
