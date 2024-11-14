use std::usize;

// use num_bigint::BigInt;
// use num_traits::One;

pub struct Folding {
    pub fold_count: u64,
}

impl Folding {
    pub fn new() -> Self {
        Self {
            fold_count: 0,
        }
    }

    pub fn get_fold_count(&mut self, dimensions: &Vec<i32>, normal_only: bool, residue: i32, modulo: i32) -> u64 {
        self.foldings(dimensions, normal_only, residue, modulo);
        self.fold_count
    }

    /// Update the fold count after each valid folding is generated
    fn record_folding(&mut self, _above: &Vec<i32>, _below: &Vec<i32>, fold: i32) {
        self.fold_count += fold as u64;
    }

    /// Generate all possible foldings based on map dimensions, considering given constraints
    pub fn foldings(&mut self, dimensions: &Vec<i32>, normal_only: bool, residue: i32, modulo: i32) {
        self.fold_count = 0;
        let total_leaves = dimensions.iter().product::<i32>();
        let dimension_count = dimensions.len();

        let mut above = vec![0; (total_leaves + 1) as usize];
        let mut below = vec![0; (total_leaves + 1) as usize];
        let mut section_count = vec![0; (total_leaves + 1) as usize];
        let mut section_gap_offset = vec![0; (total_leaves + 1) as usize];
        let mut possible_gaps = vec![0; (total_leaves * total_leaves + 1) as usize];

        let mut cumulative_dims = vec![1; dimension_count + 1];
        let mut leaf_positions = vec![vec![0; (total_leaves + 1) as usize]; dimension_count + 1];
        let mut leaf_links = vec![vec![vec![0; (total_leaves + 1) as usize]; (total_leaves + 1) as usize]; dimension_count + 1];

        for i in 1..=dimension_count {
            cumulative_dims[i] = cumulative_dims[i - 1] * dimensions[i - 1];
        }

        for i in 1..=dimension_count {
            for leaf in 1..=total_leaves {
                leaf_positions[i][leaf as usize] = (leaf - 1) / cumulative_dims[i - 1] - ((leaf - 1) / cumulative_dims[i]) * dimensions[i - 1] + 1;
            }
        }

        for i in 1..=dimension_count {
            for leaf in 1..=total_leaves {
                for base in 1..=leaf {
                    let position_difference = leaf_positions[i][leaf as usize] - leaf_positions[i][base as usize];
                    leaf_links[i][leaf as usize][base as usize] = if position_difference % 2 == 0 {
                        if leaf_positions[i][base as usize] == 1 {
                            base
                        } else {
                            base - cumulative_dims[i - 1]
                        }
                    } else if leaf_positions[i][base as usize] == dimensions[i - 1] || base + cumulative_dims[i - 1] > leaf {
                        base
                    } else {
                        base + cumulative_dims[i - 1]
                    };
                }
            }
        }

        let mut gap_index = 0;
        let mut leaf_index = 1;

        while leaf_index > 0 {
            if !normal_only || leaf_index <= 1 || below[0] == 1 {
                if leaf_index > total_leaves {
                    self.record_folding(&above, &below, total_leaves);
                } else {
                    let mut unconstrained_sections = 0;
                    let mut possible_gap_index = section_gap_offset[leaf_index as usize - 1];
                    gap_index = possible_gap_index;

                    for i in 1..=dimension_count {
                        if leaf_links[i][leaf_index as usize][leaf_index as usize] == leaf_index {
                            unconstrained_sections += 1;
                        } else {
                            let mut m = leaf_links[i][leaf_index as usize][leaf_index as usize];
                            while m != leaf_index {
                                if modulo == 0 || leaf_index != modulo || m % modulo == residue {
                                    possible_gaps[possible_gap_index as usize] = m;
                                    if section_count[m as usize] == 0 {
                                        possible_gap_index += 1;
                                    }
                                    section_count[m as usize] += 1;
                                }
                                m = leaf_links[i][leaf_index as usize][below[m as usize] as usize];
                            }
                        }
                    }

                    if unconstrained_sections == dimension_count {
                        for base in 0..leaf_index {
                            possible_gaps[possible_gap_index as usize] = base;
                            possible_gap_index += 1;
                        }
                    }

                    for j in gap_index..possible_gap_index {
                        possible_gaps[gap_index as usize] = possible_gaps[j as usize];
                        if section_count[possible_gaps[j as usize] as usize] == dimension_count - unconstrained_sections {
                            gap_index += 1;
                        }
                        section_count[possible_gaps[j as usize] as usize] = 0;
                    }
                }
            }

            while leaf_index > 0 && gap_index == section_gap_offset[leaf_index as usize - 1] {
                leaf_index -= 1;
                below[above[leaf_index as usize] as usize] = below[leaf_index as usize];
                above[below[leaf_index as usize] as usize] = above[leaf_index as usize];
            }

            if leaf_index > 0 {
                gap_index -= 1;
                above[leaf_index as usize] = possible_gaps[gap_index as usize];
                below[leaf_index as usize] = below[above[leaf_index as usize] as usize];
                below[above[leaf_index as usize] as usize] = leaf_index;
                above[below[leaf_index as usize] as usize] = leaf_index;
                section_gap_offset[leaf_index as usize] = gap_index;
                leaf_index += 1;
            }
        }
    }
}
