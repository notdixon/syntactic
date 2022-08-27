// vim: wrap!

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use colored::Colorize;

pub fn checkSyntax(file: &str, configGiven: bool, configFile: &str) {
    let fileRead = BufReader::new(File::open(file).unwrap());
    let lines: Vec<_> = fileRead.lines().map(|line| { line.unwrap() }).collect();
    let mut i = 1;
    let mut x = 1;
    let mut errors = 0;
    let mut warnings = 0;
    let mut expectingSingleLine = false;

    let mut checkIncludes = true;
    let mut checkComments = true;
    let mut checkBraces = true;

    // Read the config file
    if configGiven == true {
        let fileRead = BufReader::new(File::open(configFile).unwrap());
        let lines: Vec<_> = fileRead.lines().map(|line| { line.unwrap() }).collect();


        for line in lines.iter() {
    
            /* Check Includes */
            if line.contains("set checkIncludes") || line.contains("set check_includes") && line.contains("false") {
                println!("CONF\t{} Turned checking for includes {}", format!("[INFO] ").green().bold(), format!("OFF").red().bold());
                checkIncludes = false;
            }

            if line.contains("set checkComments") || line.contains("set check_comments") && line.contains("false") {
                println!("CONF\t{} Turned checking comment sanity {}", format!("[INFO] ").green().bold(), format!("OFF").red().bold());
                checkComments = false;
            }

            if line.contains("set checkBraces") || line.contains("set check_braces") && line.contains("false") {
                println!("CONF\t{} Turned checking for unnecessary braces {}", format!("[INFO] ").green().bold(), format!("OFF").red().bold());
                checkBraces = false;
            }
        }
    }

    for line in lines.iter() {

        /* Check Includes */
        if checkIncludes == true {
            if line.contains("#include <") {
                let includedFile = line.replace("#include <", "").replace(">", "");
                let fullFile = "/usr/include/".to_owned() + &includedFile;
    
                let includePath = Path::new(&fullFile);
    
                if !includePath.exists() {
                    println!("L{}\t{} Included file {} doesn't exist", i, format!("[ERROR]").bold().red(), format!("{}", &includedFile).green().bold());
                    errors += 1;
                }
            }

            if line.contains("#include \"") {
                if line.contains(".c") {
                    println!("L{}\t{} Don't include *.c Files - Include *.h Files", i, format!("[WARN] ").bold().yellow());
                    warnings += 1;
                }

                let includedLocalFile = line.replace("#include ", "").replace("\"", "");
                let split = file.split("/");
                let fileNameElements: Vec<&str> = split.collect();
                
                let mut includedFilePath: String = "".to_owned();
                let mut y = 0;

                for fileElement in fileNameElements.iter() {
                    if fileElement.contains(".c") {
                        break;
                    } else {
                        includedFilePath.push_str(fileElement);
                        includedFilePath.push_str("/");
                        y += y + 1;
                    }
                }

                includedFilePath.push_str(&includedLocalFile);
                
                let includeLocalPath = Path::new(&includedFilePath);
    
                if !includeLocalPath.exists() {
                    println!("L{}\t{} Locally included file {} doesn't exist", i, format!("[ERROR]").bold().red(), format!("{}", &includedLocalFile).green().bold());
                    errors += 1;
                }
            }
        }

        // GCC #warnings and #errors
        if line.contains("#warning") {
            let warningText = line.replace("#warning ", "");
            println!("L{}\t{} {}", i, format!("[WARN] ").bold().yellow(), warningText);
            warnings += 1;
        }

        if line.contains("#error") {
            let errorText = line.replace("#error ", "");
            println!("L{}\t{} {}", i, format!("[ERROR]").bold().red(), errorText);
            errors += 1;
        }

        /* Check Single Line If/Else Statements */
        if line.contains("if") || line.contains("else") && !line.contains("} else if") {
            if line.contains("}") {
                println!("L{}\t{} Consider splitting this onto multiple lines", i, format!("[WARN] ").bold().yellow());
                warnings += 1;
            }
        }

        /* Single Line If/Else Statements */
        if checkBraces == true {
            if line.contains("if") || line.contains("else") || line.contains("else if") {
                x = i;
                expectingSingleLine = true;
            }

            if expectingSingleLine == true && line.contains("}") {
                println!("L{}\t{} Braces are not required for this statement", x, format!("[WARN] ").bold().yellow());
                warnings += 1;
                expectingSingleLine = false;
            }
        }

        // Single Line Comments vs Multi-Line Comments on one line
        if checkComments == true {
            if line.contains("/*") && line.contains("*/") {
                println!("L{}\t{} For single line comments use // instead of /* */", i, format!("[WARN] ").bold().yellow());
                warnings += 1;
            }

            if line.contains("/* ") && !line.contains("/* \n") {
                println!("L{}\t{} Multi-line comments should have /* on its own line", i, format!("[WARN] ").bold().yellow());
                warnings += 1;
            }

            if line.contains("* ") && line.contains("*/") {
                println!("L{}\t{} Multi-line comments should have */ on its own line", i, format!("[WARN] ").bold().yellow());
                warnings += 1;
            }
        }

        i += 1;
    }

    println!("\n{} - {} Lines, {} Warnings, {} Errors", file, i, warnings, errors);
    if errors == 0 && warnings == 0 {
        println!("Looks good!");
    }
}
