/* vim: filetype=rust foldmethod=marker tabstop=8
 *
 * notdixon's - Syntactic
 * ======================
 *
 * Check the syntax of a given source code file to check for inconsistencies
 * and syntax errors.
 *
 * Copyright (C) 2022   notdixon <notdixon@gmx.com>
 *
 * {{{ LICENSE: GNU General Public License v2
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * The GNU General Public License is available in the 'LICENSE' file, or
 * on the GNU Licenses page: <https://gnu.org/licenses/gpl-2.0.html>
 * }}}
 *
 * Syntactic Source Code is available on GitHub
 * <https://github.com/notdixon/syntactic>
 */

#![allow(non_snake_case)]
mod syntactic;

use std::env;
use std::path::Path;
use colored::Colorize;

fn showHelp() {
    println!("\
Syntactic v0.1\n\
==============\n\
Usage: syntactic [-c <config>] -f <source file>\n\
\n\
 --conf     | -c       Specify a config file\n\
 --file     | -f       Specify an input file to check");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i;
    let mut configFileGiven = false;
    let mut configFile = "";
    i = 0;

    if env::args().len() == 1 {
        showHelp();
        std::process::exit(1);
    }

    for arg in args.iter() {
        if arg.contains("--conf") || arg.contains("-c") {
            if i + 2 > args.len() {
                println!("{} No config was specified", format!("[ERROR]").red().bold());
            } else {
                let filePath = Path::new(&args[i + 1]);
                if !filePath.exists() {
                    println!("A{}\t{} Configuration File {} doesn't exist", i + 1, format!("[ERROR]").red().bold(), format!("{}", &args[i + 1]).green().bold());
                } else {
                    configFileGiven = true;
                    configFile = &args[i + 1];
                }
            }
        }

        if arg.contains("--file") || arg.contains("-f") {
            if i + 2 > args.len() {
                println!("{} No source file was specified", format!("[ERROR]").red().bold());
            } else if args[i + 1].contains(".c") && !args[i + 1].contains(".cpp") {
                syntactic::csyntax::checkSyntax(&args[i + 1], configFileGiven, &configFile);
            } else {
                let filePath = Path::new(&args[i + 1]);
                if !filePath.exists() {
                    println!("A{}\t{} File {} doesn't exist", i + 1, format!("[ERROR]").red().bold(), format!("{}", &args[i + 1]).green().bold());
                } else {
                    println!("A{}\t{} File type unsupported: Not Implemented", i + 1, format!("[ERROR]").red().bold());
                }
            }
        }
        i += 1;
    }
}

// LICENSE-Identifier: GNU_GPL_v2
