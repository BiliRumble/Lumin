use std::collections::HashMap;

fn onani(n: i32, dic: &HashMap<i32, String>) -> String {
    let mut result = String::new();
    let mut n = n;
    while n > 50 {
        let digi = (n as f64).log10() / 1.6989700043360188047862611052755f64;
        let tmp = n % (50_i32.pow(digi as u32));
        if digi as i32 == 1 {
            result.push_str(&format!(
                "({})*((0-7)^2+1)+",
                dic[&(&(n - tmp) / 50_i32.pow(digi as u32))]
            ));
        } else {
            result.push_str(&format!(
                "({})*((0-7)^2+1)^({})+",
                dic[&(&(n - tmp) / 50_i32.pow(digi as u32))],
                dic[&(digi as i32)]
            ));
        }
        n -= n - tmp;
    }
    result + &dic[&n]
}

// 传入任意自然数n，返回n的Onani数
pub fn onani_number(n: i32) -> String {
    let dic: HashMap<i32, String> = [
        (0, "0*7*2*1".to_string()),
        (1, "0*7+2-1".to_string()),
        (2, "(0*7+2)*1".to_string()),
        (3, "0*7+2+1".to_string()),
        (4, "0+7-2-1".to_string()),
        (5, "(0+7-2)*1".to_string()),
        (6, "0+7-2+1".to_string()),
        (7, "(0+7)*(2-1)".to_string()),
        (8, "0+7+2-1".to_string()),
        (9, "(0+7+2)*1".to_string()),
        (10, "0+7+2+1".to_string()),
        (13, "(0+7)*2-1".to_string()),
        (14, "(0+7)*2*1".to_string()),
        (15, "(0+7)*2+1".to_string()),
        (21, "(0+7)*(2+1)".to_string()),
        (48, "(0-7)^2-1".to_string()),
        (49, "(0-7)^2*1".to_string()),
        (50, "(0-7)^2+1".to_string()),
        (11, "((0+7)*2-1+0-7)*2-1".to_string()),
        (12, "((0+7)*2-1+0-7)*2*1".to_string()),
        (16, "((0+7)*2+1+0-7)*2*1".to_string()),
        (17, "((0+7)*2+1+0-7)*2+1".to_string()),
        (18, "0+7+2-1+0+7+2+1".to_string()),
        (19, "(0+7)*2-1+0+7-2+1".to_string()),
        (20, "((0+7)*2+1+0+7-2)*1".to_string()),
        (22, "((0+7)*2-1+0+7+2)*1".to_string()),
        (23, "(0+7)*2-1+0+7+2+1".to_string()),
        (24, "0+7+2*(1+0+7)+2-1".to_string()),
        (25, "(0+7-2-1+0)*7-2-1".to_string()),
        (26, "((0+7-2-1+0)*7-2)*1".to_string()),
        (27, "(0+7-2-1+0)*7-2+1".to_string()),
        (28, "(0+7)*2+1+0+7*2-1".to_string()),
        (29, "(0+7+2-1+0+7)*2-1".to_string()),
        (30, "(0+7+2-1+0+7)*2*1".to_string()),
        (31, "(0+7+2-1+0+7)*2+1".to_string()),
        (32, "((0-7+2)*(1+0-7)+2)*1".to_string()),
        (33, "(0-7+2)*(1+0-7)+2+1".to_string()),
        (34, "(0+7+2+1+0+7)*2*1".to_string()),
        (35, "((0-7)*2+1+0+7)^2-1".to_string()),
        (36, "((0-7)*2+1+0+7)^2*1".to_string()),
        (37, "(0+7-2)*(1+0+7)-2-1".to_string()),
        (38, "((0+7-2)*(1+0+7)-2)*1".to_string()),
        (39, "(0+7-2)*(1+0+7)-2+1".to_string()),
        (40, "(0-7)^2-1+0-7-2+1".to_string()),
        (41, "(0+7-2)*(1+0+7)+2-1".to_string()),
        (42, "((0+7-2)*(1+0+7)+2)*1".to_string()),
        (43, "(0+7-2)*(1+0+7)+2+1".to_string()),
        (44, "(0-7)^2-1+0-7+2+1".to_string()),
        (45, "(0-7)^2-1+0*7-2-1".to_string()),
        (46, "((0-7)^2-1+0*7-2)*1".to_string()),
        (47, "(0-7)^2-1+0*7-2+1".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    onani(n, &dic)
}
