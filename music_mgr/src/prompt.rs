use std::{collections::HashMap, io::Write};



pub fn simple_prompt(p: &str) -> String {

    print!("{c}prompt{r}: {p}",
        c=anstyle::AnsiColor::Magenta.render_fg(),
        r=anstyle::Style::new().render_reset()
    );

    // I dont care if it fails
    let _ = std::io::stdout().flush();
    
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    buf
}

pub fn prompt_with_options(p: &str, options: &[&str]) -> usize {
    println!("{c}prompt{r}: {p}",
        c=anstyle::AnsiColor::Magenta.render_fg(),
        r=anstyle::Style::new().render_reset()
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
            log::error!("Number not in range");
            return prompt_with_options(p, options);
        }
    } else {
        log::error!("Not a number");
        return prompt_with_options(p, options);
    }
}

pub fn prompt_with_named_options(p: &str, options: HashMap<&str, &str>) -> String {
    println!("{c}prompt{r}: {p}",
        c=anstyle::AnsiColor::Magenta.render_fg(),
        r=anstyle::Style::new().render_reset()
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
        return prompt_with_named_options(p, options);
    }
    buf
}
