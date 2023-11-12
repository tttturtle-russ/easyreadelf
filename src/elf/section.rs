use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use crate::elf::header::parse_header;

pub struct ElfSection {
    index:u16,
    name:String,
    sh_name:u32,
    sh_type:u32,
    sh_flags:u64,
    sh_addr:u64,
    sh_offset:u64,
    sh_size:u64,
    sh_link:u32,
    sh_info:u32,
    sh_addralign:u64,
    sh_entsize:u64,
}

impl From<Vec<u8>> for ElfSection {
    fn from(data: Vec<u8>) -> Self {
        Self {
            index:0,
            name:String::new(),
            sh_name: u32::from_ne_bytes(data[0..4].try_into().unwrap()),
            sh_type: u32::from_ne_bytes(data[4..8].try_into().unwrap()),
            sh_flags: u64::from_ne_bytes(data[8..16].try_into().unwrap()),
            sh_addr: u64::from_ne_bytes(data[16..24].try_into().unwrap()),
            sh_offset: u64::from_ne_bytes(data[24..32].try_into().unwrap()),
            sh_size: u64::from_ne_bytes(data[32..40].try_into().unwrap()),
            sh_link: u32::from_ne_bytes(data[40..44].try_into().unwrap()),
            sh_info: u32::from_ne_bytes(data[44..48].try_into().unwrap()),
            sh_addralign: u64::from_ne_bytes(data[48..56].try_into().unwrap()),
            sh_entsize: u64::from_ne_bytes(data[56..64].try_into().unwrap()),
        }
    }
}

impl std::fmt::Display for ElfSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.index == 0{
            writeln!(f,"{:15}{:20}{:18}\t{:18}\t{:18}","0","NULL","NULL","0","0")?;
            return Ok(());
        }
        writeln!(f,"{:<15}{:<20}{:<#018x}\t{:<#018x}\t{:<#018x}",self.index,self.name,self.sh_addr,self.sh_offset,self.sh_size)
    }
}

pub fn parse_sections(elf:&mut File) -> io::Result<Vec<ElfSection>> {
    let header = parse_header(elf)?;
    let mut sections = Vec::new();
    elf.seek(io::SeekFrom::Start(header.e_shoff + (header.e_shstrndx as u64 * header.e_shentsize as u64)))?;
    elf.seek(io::SeekFrom::Current(24))?;
    let mut sh_offset = [0u8;8];
    let mut sh_size = [0u8;8];
    elf.read_exact(&mut sh_offset)?;
    elf.read_exact(&mut sh_size)?;
    let size: u64 = u64::from_le_bytes(sh_size);
    elf.seek(io::SeekFrom::Start(u64::from_le_bytes(sh_offset)))?;
    let mut tables = vec![0u8;size as usize];
    elf.read_exact(&mut tables)?;
    elf.seek(io::SeekFrom::Start(header.e_shoff))?;
    println!("{:15}{:20}{:16}\t{:16}\t{:16}","Index", "Name", "Address", "Offset","Size");
    for i in 0..header.e_shnum {
        let mut data = vec![0u8;header.e_shentsize as usize];
        elf.read_exact(&mut data)?;
        let mut section = ElfSection::from(data);
        section.index = i;
        let mut sh_name = section.sh_name;
        while tables[sh_name as usize] != 0 {
            section.name.push(tables[sh_name as usize] as char);
            sh_name += 1;
        }
        sections.push(section);
    }
    Ok(sections)
}