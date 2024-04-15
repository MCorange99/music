use std::{collections::HashMap, io::Write};



pub fn simple_prompt(p: &str) -> String {

    print!("{c}prompt{r}: {p} > ",
        c=anstyle::AnsiColor::Cyan.render_fg(),
        r=anstyle::Reset.render()
    );

    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    buf.trim().to_string()
}

pub fn prompt_with_list(p: &str, options: &[&str]) -> usize {
    println!("{c}prompt{r}: {p}",
        c=anstyle::AnsiColor::Cyan.render_fg(),
        r=anstyle::Reset.render()
    );

    for (i, op) in options.iter().enumerate() {
        println!("    - {}: {}", i, op);
    }

    print!("> ");
    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    if let Ok(num) = buf.parse::<usize>() {
        if num <= options.len() {
            return num;
        } else {
            return prompt_with_list(p, options);
        }
    } else {
        return prompt_with_list(p, options);
    }
}

pub fn prompt_with_list_or_str(p: &str, options: &[String]) -> String {
    println!("{c}prompt{r}: {p} (select with number or input text)",
        c=anstyle::AnsiColor::Cyan.render_fg(),
        r=anstyle::Reset.render()
    );

    for (i, op) in options.iter().enumerate() {
        println!("    - {}: {}", i, op);
    }

    print!("> ");
    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    if let Ok(num) = buf.trim().parse::<usize>() {
        if let Some(g) = options.get(num) {
            return g.clone();
        } else {
            return prompt_with_list_or_str(p, options);
        }
    } else {
        return buf.trim().to_string();
    }
}



pub fn prompt_with_map(p: &str, options: HashMap<&str, &str>) -> String {
    println!("{c}prompt{r}: {p}",
        c=anstyle::AnsiColor::Cyan.render_fg(),
        r=anstyle::Reset.render()
    );

    let mut keys = Vec::new();

    for (k, v) in &options {
        println!("    - {}: {}", k, v);
        keys.push(k.trim().to_lowercase())
    }

    print!("> ");

    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    if !keys.contains(&buf.trim().to_lowercase()) {
        return prompt_with_map(p, options);
    }
    buf.trim().to_string()
}

pub fn prompt_bool(p: &str, default: Option<bool>) -> bool {
    if default == Some(true) {
        println!("{c}prompt{r}: {p} (Y/n)",
            c=anstyle::AnsiColor::Cyan.render_fg(),
            r=anstyle::Reset.render()
        );
    } else if default == Some(false) {
        println!("{c}prompt{r}: {p} (y/N)",
            c=anstyle::AnsiColor::Cyan.render_fg(),
            r=anstyle::Reset.render()
        );
    } else {
        println!("{c}prompt{r}: {p} (y/n)",
            c=anstyle::AnsiColor::Cyan.render_fg(),
            r=anstyle::Reset.render()
        );
    }
    print!("> ");

    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    if buf.trim().is_empty() {
        match default {
            Some(true) => return true,
            Some(false) => return false,
            None => {
                return prompt_bool(p, default);
            }
        }
    }

    match buf.to_lowercase().trim() {
        "y" => true,
        "n" => false,
        c => {
            log::error!("'{c}' is invalid, type y (yes) or n (no)");
            return prompt_bool(p, default);
        }
    }
}
