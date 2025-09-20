// Find all combinations of original RGB modifications that can be made that result in a new RGB pixel that maintains a given total change.
// This new, modified pixel will differ in channels to the original by 'total_change' and maintain the greyscale LSB value.

fn calculate_greyscale(r: u8, g: u8, b: u8) -> u8 {
    ((0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32)).round() as u8
}

fn extract_lsb(value: u8) -> u8 {
    value & 1
}

fn find_least_deviation(combinations: Vec<[u8;3]>, org_rgb: [u8;3]) -> [u8; 3] {
    let mut best_combination = combinations[0];
    let mut best_score = f64::MAX;
    
    for item in &combinations {
        let r_diff = (item[0] as i32 - org_rgb[0] as i32) as f64;
        let g_diff = (item[1] as i32 - org_rgb[1] as i32) as f64;
        let b_diff = (item[2] as i32 - org_rgb[2] as i32) as f64;
        
        // Calculate balance metrics
        let total_abs_change = r_diff.abs() + g_diff.abs() + b_diff.abs();
        let max_abs_change = r_diff.abs().max(g_diff.abs()).max(b_diff.abs());
        let mixed_sign_score = if (r_diff.signum() + g_diff.signum() + b_diff.signum()).abs() < 3.0 {
            0.5 // Bonus for mixed signs (not all positive/negative)
        } else {
            1.0
        };
        
        // Combined score: prefer small, balanced changes with mixed signs
        let balance_score = (max_abs_change * 2.0 + total_abs_change) * mixed_sign_score;
        
        if balance_score < best_score {
            best_score = balance_score;
            best_combination = *item;
        }
    }
    best_combination
}

pub fn find_valid_modification(r: u8, g: u8, b: u8, total_change: i32) -> [u8; 3] {
    let original_greyscale = calculate_greyscale(r, g, b);
    let original_lsb = extract_lsb(original_greyscale);

    let mut possible_combinations = Vec::new();
    
    for r_change in -total_change..=total_change {
        for g_change in -total_change..=total_change {
            for b_change in -total_change..=total_change {
                if r_change.abs() + g_change.abs() + b_change.abs() == total_change {
                    // Check if new values are within u8 range (0-255)
                    let new_r = r as i32 + r_change;
                    let new_g = g as i32 + g_change;
                    let new_b = b as i32 + b_change;
                    
                    if new_r >= 0 && new_r <= 255 && 
                       new_g >= 0 && new_g <= 255 && 
                       new_b >= 0 && new_b <= 255 {
                        
                        let modified_greyscale = calculate_greyscale(new_r as u8, new_g as u8, new_b as u8);
                        let modified_lsb = extract_lsb(modified_greyscale);
                        
                        if modified_lsb == original_lsb {
                            possible_combinations.push([new_r as u8, new_g as u8, new_b as u8]);
                        }
                    }
                }
            }
        }
    }

    possible_combinations.push([0, 0, 0]); // if no valid combinations are found, null values passed. Should indicate to skip the pixel.
    
    let best_combination = find_least_deviation(possible_combinations, [r,g,b]);

    best_combination
}