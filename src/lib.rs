use std::error::Error;

pub struct Config {
    pub files: Vec<String>,
    pub line_nums: bool,
}

struct Footer {
    line_num: usize,
}

impl Config{
    pub fn build(mut args: impl Iterator<Item = String>,)-> Result<Config, Box<dyn Error>>{
    let mut files: Vec<String> = vec![];
    let mut line_nums = false;

    // looking for -f
    while let Some(arg) = args.next() {
        if arg == "-f" {
            if let Some(file) = args.next() {
                files = file.split("-").map(|s| s.to_string()).collect();
            }
        } else if arg == "-b"{
            line_nums = true;
        }
    }


    Ok(Config { files , line_nums})


    }

    fn print_footer(&self, footer: Footer){
        println!("Output Info: ");
        if self.line_nums {
            println!("Total lines: {}", footer.line_num);
        }

    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut line_num = 0;

    for file in &config.files {
        let content = match std::fs::read_to_string(file){
            Ok(content) => content,
            Err(err) => return Err(Box::new(err)),
        };

        for line in content.lines() {
            if line != "" {
                line_num += 1;
            }
            println!("{}", line);
        }
    }


    config.print_footer(Footer{line_num});
    Ok(())
}

#[cfg(test)]
mod tests {

}
