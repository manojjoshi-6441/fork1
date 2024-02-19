use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
#[derive(Debug, Clone)]
struct Master1 {
    t1: String,
    t2: String,
    t3: String,
}
fn main() {
    let mut master_1_hashmap: HashMap<String, Master1> = HashMap::new();
    let mut master_2_hashmap: HashMap<String, String> = HashMap::new();
    let mut master_1_vec: Vec<String> = Vec::new();
    let mut master_2_vec: Vec<String> = Vec::new();
    let mut config_file_line = String::new();
    let mut mat_date = String::new();
    let mut master1_t1 = String::new();
    let mut master1_t2 = String::new();
    let mut master1_t3 = String::new();
    let mut four_point_concat = String::new();
    let mut currency = String::new();
    let mut as_on_date = String::new();
    let mut undrawn_amt = String::new();
    let mut outfile = File::create("Output.txt").expect("Failed to get output file");
    let mut i1_vec3: Vec<String> = Vec::new();
    let config_file = File::open("Configv1.txt").unwrap();
    let reader4 = BufReader::new(config_file);
    for line in reader4.lines() {
        config_file_line = line.expect("fail to read contents of a file").to_string();
    }
    let file = File::open("M1.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let master_1_line = line.expect("fail to read contents of master 1 file");
        master_1_vec = master_1_line.split("~#~").map(|s| s.to_string()).collect();

        if !master_1_hashmap.contains_key(&master_1_vec[0]) {
            let mut master_struct = Master1 {
                t1: master_1_vec[1].to_string(),
                t2: master_1_vec[2].to_string(),
                t3: master_1_vec[3].to_string(),
            };
            master_1_hashmap.insert(master_1_vec[0].to_string(), master_struct);
        }
    }
    let mut file2 = File::open("M2.txt").unwrap();
    let reader2 = BufReader::new(file2);
    for line in reader2.lines() {
        let master_file2_line = line.expect("fail to read contents of master 2 file");
        let mut master_2_vec: Vec<String> = master_file2_line
            .split("|")
            .map(|s| s.to_string())
            .collect();
        if !master_2_hashmap.contains_key(&master_2_vec[0].to_string()) {
            master_2_hashmap.insert(master_2_vec[0].to_string(), master_2_vec[1].to_string());
        }
    }
    let mut file3 = File::open("I1v1.txt").unwrap();
    let reader3 = BufReader::new(file3);
    for line in reader3.lines() {
        let i1_file_line = line.expect("Failed to read contents of i1 file");
        let mut i1_vec3: Vec<String> = i1_file_line.split("|").map(|s| s.to_string()).collect();
        let mut account_number:String = i1_vec3[0].to_string();
        let mut customer_id:String = i1_vec3[1].to_string();
        let mut out:String = i1_vec3[7].to_string();
        let mut product_code_prefix :String= out[3..10].to_string();
        let mut sanct_amt:String= i1_vec3[2].to_string();
        let mut distributed_amt = i1_vec3[3].to_string();
        let mut cf_type=String::new();
        if i1_vec3[4] == "P" || i1_vec3[4] == "PR1" {
            if i1_vec3[3] > 0.to_string() {
                cf_type = "PRINICIPAL".to_string();
            }
        } else if i1_vec3[4] == "I" || i1_vec3[4] == "INT" {
            cf_type = "INTEREST".to_string();
        } else {
             cf_type = "PRINICIPAL".to_string();
        }
        mat_date = i1_vec3[5].to_string();
        if master_1_hashmap.contains_key(&i1_vec3[1]) {
            let data = master_1_hashmap.get(&i1_vec3[1]).unwrap();
             master1_t1 = data.t1.to_string();
            } 
            else {
             master1_t1 = "Null".to_string();
        }
        if master_1_hashmap.contains_key(&i1_vec3[1]) {
            let data1 = master_1_hashmap.get(&i1_vec3[1]).unwrap();
            master1_t2 = data1.t2.to_string();
            } 
            else {
            master1_t2 = "Null".to_string();
        }
        if master_1_hashmap.contains_key(&i1_vec3[1]) {
            let data3 = master_1_hashmap.get(&i1_vec3[1]).unwrap();
            master1_t3 = data3.t3.to_string();
            } 
            else {
            master1_t3 = "Null".to_string();
        }
        let mut master1_category = &i1_vec3[6];
        let mut master1_product_code = &i1_vec3[7];
        let mut master1_gl_code = &i1_vec3[8];
        let mut four_point_concat=String::new();
        four_point_concat = format!("{}{}{}{}{}",master1_category, "|", master1_product_code, "|", master1_gl_code
        );
        let mut currency:String = "INR".to_string();
        let mut as_on_date:String = config_file_line.to_string();
        let mut gl_description=String::new();
        if master_2_hashmap.contains_key(&i1_vec3[8]) {
            let mut data4 = master_2_hashmap.get(&i1_vec3[8]).unwrap();
             gl_description = data4.to_string();
        } else {
            gl_description = "NUll".to_string();
        }
    let revised_sanct_amt:f64=sanct_amt.parse().unwrap();
    let revised_distributed_amt:f64=distributed_amt.parse().unwrap();
    println!("{}",revised_sanct_amt);
    println!("{}",revised_distributed_amt);
    undrawn_amt=(((revised_sanct_amt-revised_distributed_amt))*revised_sanct_amt/100.0).to_string();
    let mut final_output_text = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",account_number,"|",customer_id,"|",product_code_prefix,"|",sanct_amt,"|",distributed_amt,"|",cf_type,"|",mat_date,"|",master1_t1,"|",master1_t2,"|",master1_t3,"|",four_point_concat,"|",currency,"|",as_on_date,"|",undrawn_amt,"|",gl_description);
    outfile.write_all(&final_output_text.as_bytes()).expect("Failed to write to a file");
    }
}
