use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use crate::elf::header::parse_header;

pub struct ElfSegment {
    p_type:SegmentType,
    p_flags:SegmentFlags,
    p_offset:u64,
    p_vaddr:u64,
    p_paddr:u64,
    p_filesz:u64,
    p_memsz:u64,
    p_align:u64,
}

enum SegmentFlags {
    NULL,
    X,
    W,
    WX,
    R,
    RX,
    RW,
    RWX,
    UNKNOWN
}

impl SegmentFlags {
    fn to_string(&self) -> String {
        match self {
            SegmentFlags::NULL => {String::from("NULL")},
            SegmentFlags::X => {String::from("X")},
            SegmentFlags::W => {String::from("W")},
            SegmentFlags::WX => {String::from("WX")},
            SegmentFlags::R => {String::from("R")},
            SegmentFlags::RX => {String::from("RX")},
            SegmentFlags::RW => {String::from("RW")},
            SegmentFlags::RWX => {String::from("RWX")},
            SegmentFlags::UNKNOWN => {String::from("UNKNOWN")}
        }
    }

    fn from(data:Vec<u8>) -> SegmentFlags {
        let value = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        match value {
            0 => {SegmentFlags::NULL},
            1 => {SegmentFlags::X},
            2 => {SegmentFlags::W},
            3 => {SegmentFlags::WX},
            4 => {SegmentFlags::R},
            5 => {SegmentFlags::RX},
            6 => {SegmentFlags::RW},
            7 => {SegmentFlags::RWX},
            _ => {SegmentFlags::UNKNOWN}
        }
    }
}

enum SegmentType {
    NULL,
    LOAD,
    DYNAMIC,
    INTERP,
    NOTE,
    SHLIB,
    PHDR,
    GNU_EH_FRAME = 0x6474e550,
    GNU_STACK = 0x6474e551,
    GNU_RELRO = 0x6474e552,
    GNU_PROPERTY = 0x6474e553,
    LOPROC = 0x70000000,
    HIPROC = 0x7fffffff,
    UNKNOWN
}

impl SegmentType {
    fn to_string(&self) -> String {
        match self {
            SegmentType::NULL => {String::from("NULL")},
            SegmentType::LOAD => {String::from("LOAD")},
            SegmentType::DYNAMIC => {String::from("DYNAMIC")},
            SegmentType::INTERP => {String::from("INTERP")},
            SegmentType::NOTE => {String::from("NOTE")},
            SegmentType::SHLIB => {String::from("SHLIB")},
            SegmentType::PHDR => {String::from("PHDR")},
            SegmentType::GNU_EH_FRAME => {String::from("GNU_EH_FRAME")},
            SegmentType::GNU_STACK => {String::from("GNU_STACK")},
            SegmentType::GNU_RELRO => {String::from("GNU_RELRO")},
            SegmentType::GNU_PROPERTY => {String::from("GNU_PROPERTY")},
            SegmentType::LOPROC => {String::from("LOPROC")},
            SegmentType::HIPROC => {String::from("HIPROC")},
            SegmentType::UNKNOWN => {String::from("UNKNOWN")}
        }
    }

    fn from(data:Vec<u8>) -> SegmentType {
        let value = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        match value {
            0 => {SegmentType::NULL},
            1 => {SegmentType::LOAD},
            2 => {SegmentType::DYNAMIC},
            3 => {SegmentType::INTERP},
            4 => {SegmentType::NOTE},
            5 => {SegmentType::SHLIB},
            6 => {SegmentType::PHDR},
            0x6474e553 => {SegmentType::GNU_PROPERTY},
            0x6474e550 => {SegmentType::GNU_EH_FRAME},
            0x6474e551 => {SegmentType::GNU_STACK},
            0x6474e552 => {SegmentType::GNU_RELRO},
            0x70000000 => {SegmentType::LOPROC},
            0x7fffffff => {SegmentType::HIPROC},
            _ => {SegmentType::UNKNOWN}
        }
    }
}

impl std::fmt::Display for ElfSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:<15}{:<#018x}\t{:<#018x}\t{:<#018x}\t{:<#018x}\t{:<#018x}\t{}",self.p_type.to_string(),self.p_offset,self.p_vaddr,self.p_paddr,self.p_filesz,self.p_memsz,self.p_flags.to_string())
    }
}

impl From<Vec<u8>> for ElfSegment {
    fn from(value: Vec<u8>) -> Self {
        Self {
            p_type: SegmentType::from(value[0..4].to_vec()),
            p_flags: SegmentFlags::from(value[4..8].to_vec()),
            p_offset: u64::from_ne_bytes(value[8..16].try_into().unwrap()),
            p_vaddr: u64::from_ne_bytes(value[16..24].try_into().unwrap()),
            p_paddr: u64::from_ne_bytes(value[24..32].try_into().unwrap()),
            p_filesz: u64::from_ne_bytes(value[32..40].try_into().unwrap()),
            p_memsz: u64::from_ne_bytes(value[40..48].try_into().unwrap()),
            p_align: u64::from_ne_bytes(value[48..56].try_into().unwrap()),
        }
    }
}

pub fn parse_segments(elf:&mut File) -> io::Result<Vec<ElfSegment>> {
    let header = parse_header(elf)?;
    let mut segments = Vec::new();
    println!("Entry point:{:#x}",header.e_entry);
    println!("There are {} program headers, starting at offset {}:",header.e_phnum,header.e_phoff);
    println!("{:<15}{:<18}\t{:<18}\t{:<18}\t{:<18}\t{:<18}\t{}","Type", "Offset", "VirtualAddr", "PhysAddr","FileSiz","MemSiz","Flags");
    // 从文件头开始偏移e_phoff个字节
    elf.seek(io::SeekFrom::Start(header.e_phoff))?;
    for _ in 0..header.e_phnum {
        let mut data = vec![0u8;header.e_phentsize as usize];
        elf.read_exact(&mut data)?;
        let segment = ElfSegment::from(data);
        segments.push(segment);
    }
    Ok(segments)
}