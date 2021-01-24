use std::{process::{Command, Stdio}};
#[derive(Debug)]
struct Process<'a> {
    pid:  &'a str,
    gid: &'a str,
    user: &'a str,
    size: u32,
    pcpu: f32,
    cmd: &'a str,
}

impl<'a> Process<'a> {
    fn new(raw_line: &str) -> Process {
        let data: Vec<&str> = raw_line.split_ascii_whitespace().collect();

        Process { 
            pid: data[0],
            gid: data[1],
            user: data[2],
            size: data[3].parse::<u32>().unwrap() / 1024,
            pcpu: data[4].parse::<f32>().unwrap(),
            cmd: data[5]
        }
    }
}

fn main() {
    let mut result: Vec<Process> = vec![];
    let command = Command::new("ps")
        .args(&["-Ao", "pid,pgrp,user,size,pcpu,comm"])
        .stdout(Stdio::piped())
        .spawn().unwrap()
        .wait_with_output().unwrap()
        .stdout;
    let raw_string= String::from_utf8_lossy(&command[..]);
    let mut lines: Vec<&str> = raw_string.split("\n").collect();
    let raw_processes = lines.drain(1..)
        .filter(|l| !l.is_empty())    
        .map(|l| Process::new(&l))
        .collect::<Vec<Process>>();

    raw_processes
        .into_iter()
        .fold::<&mut Vec<Process>, _>(&mut result, |res, p| {
            let process_line = res.into_iter().find(|l| l.gid == p.gid);
            match process_line {
                Some(_) => {
                    let grouped_item = process_line.unwrap();
                    grouped_item.size += p.size;
                    grouped_item.pcpu += p.pcpu;
                }
                None => res.push(p)
            }
            res
        });

    result.sort_by(|a, b| a.pcpu.partial_cmp(&b.pcpu).unwrap());

    println!("{} | cpu: {:.1}% | mem: {} Mb", result[result.len()-1].cmd, result[result.len()-1].pcpu, result[result.len()-1].size);
}
