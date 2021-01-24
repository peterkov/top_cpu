use std::{process::{Command, Stdio}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
        let asd: Vec<&str> = raw_line.split_ascii_whitespace().collect();
        Process { 
            pid: asd[0],
            gid: asd[1],
            user: asd[2],
            size: asd[3].parse::<u32>().unwrap() / 1024,
            pcpu: asd[4].parse::<f32>().unwrap(),
            cmd: asd[5]
        }
    }
}

fn main() {
    let mut command = Command::new("ps");
    command.args(&["-Ao", "pid,pgrp,user,size,pcpu,comm"]);
    command.stdout(Stdio::piped());

    let output = command
        .spawn()
        .unwrap()
        .wait_with_output().unwrap().stdout;

    let asd= String::from_utf8_lossy(&output[..]);
    let mut lines: Vec<&str> = asd.split("\n").collect();
    
    let raw_processes = lines.drain(1..)
        .filter(|l| !l.is_empty())    
        .map(|l| Process::new(&l))
        .collect::<Vec<Process>>();

    let mut result: Vec<Process> = vec![];
    raw_processes
        .into_iter()
        .fold::<&mut Vec<Process>, _>(&mut result, |res, p| {
            let vbwe = res.into_iter().find(|l| l.gid == p.gid);
            match vbwe {
                Some(_) => {
                    let asd = vbwe.unwrap();
                    asd.size += p.size;
                    asd.pcpu += p.pcpu;
                }
                None => res.push(p)
            }
            res
        });

    result.sort_by(|a, b| a.pcpu.partial_cmp(&b.pcpu).unwrap());

    println!("{} : cpu: {:.1} | mem: {}", result[result.len()-1].cmd, result[result.len()-1].pcpu, result[result.len()-1].size);
}
