use hashbrown::HashMap;
use std::str;

pub fn process_barcodes(
    bar_found: HashMap<String, i32>,
    min_count: i32,
    min_barcodes: usize,
) -> (String, String, String) {
    // merge barcode IDs to lineages
    let lineages = merge_barcodes(bar_found, min_count);

    // save all barcode info into String
    let log_barcodes = format_data(lineages.clone());

    // filter lineages using input parameters
    let filtered_lineages = filter_lineages(lineages.clone(), min_barcodes);

    // get non-inclusive lineages sorted by nb occurrences
    let vect_lineages = non_inclusive_lineages(filtered_lineages);

    // check if mixture of lineages
    let mut mixture = "no";
    if vect_lineages.len() > 1 {
        mixture = "yes";
    }

    // convert to String
    let formatted_lineages: Vec<String> = vect_lineages
        .iter()
        .map(|(lineage_name, med_value)| format!("{} ({})", lineage_name, med_value))
        .collect();

    let result = formatted_lineages.join(", ");

    (result, mixture.to_string(), log_barcodes)
}

fn merge_barcodes(b_found: HashMap<String, i32>, min_occurences: i32) -> HashMap<String, Vec<i32>> {
    let mut merged_lineages: HashMap<String, Vec<i32>> = HashMap::new();

    for (barcode_id, nb_occurences) in &b_found {
        // only consider barcode IDs with abundances >= minimum count
        if nb_occurences >= &min_occurences {
            let parts: Vec<&str> = barcode_id.split('_').collect();
            let lineage = parts[0].to_string();
            match merged_lineages.get(&lineage) {
                Some(_vect_nb) => {
                    merged_lineages
                        .get_mut(&lineage)
                        .unwrap()
                        .push(nb_occurences.to_owned());
                }
                None => {
                    merged_lineages.insert(lineage.clone(), Vec::new());
                    merged_lineages
                        .get_mut(&lineage)
                        .unwrap()
                        .push(nb_occurences.to_owned());
                }
            }
        }
    }
    merged_lineages
}

fn format_data(data: HashMap<String, Vec<i32>>) -> String {
    // convert hashmap into a string of the following format: key (nb,nb,nb), key2 (nb,nb,nb), ...
    let mut sorted_keys: Vec<&String> = data.keys().collect();
    sorted_keys.sort();

    sorted_keys
        .iter()
        .map(|&key| {
            let values = data.get(key).unwrap();
            let values_string = values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ");
            format!("{} ({})", key, values_string)
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn filter_lineages(
    lineages: HashMap<String, Vec<i32>>,
    min_barcodes: usize,
) -> HashMap<String, i32> {
    // filter lineages with at least min_barcodes barcodes
    let mut filtered_lineages: HashMap<String, i32> = HashMap::new();

    for (lineage_id, vect_nb) in &lineages {
        if vect_nb.len() >= min_barcodes {
            let med_value = median(vect_nb);
            filtered_lineages.insert(lineage_id.to_string(), med_value);
        }
    }
    filtered_lineages
}

fn median(values: &[i32]) -> i32 {
    let mut sorted_values = values.to_owned();
    sorted_values.sort();
    let len = sorted_values.len();
    if len % 2 == 0 {
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2
    } else {
        sorted_values[len / 2]
    }
}

fn non_inclusive_lineages(lineages: HashMap<String, i32>) -> Vec<(String, i32)> {
    let all_keys: Vec<String> = lineages.keys().cloned().collect();
    let mut final_vect = vec![];

    for (lin, med_value) in lineages {
        let mut not_included = true;
        for key in all_keys.clone() {
            if key.starts_with(lin.as_str()) && lin != key {
                not_included = false;
                break;
            }
        }

        if not_included {
            final_vect.push((lin, med_value));
        }
    }
    final_vect
}
