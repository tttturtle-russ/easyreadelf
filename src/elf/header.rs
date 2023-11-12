use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read, Seek};
use std::io;

const DEFAULT_VERSION:u8 = 1;

pub struct ElfHeader{
    /*
    e_idnet:
        0-3:0x7f,E,L,F
        4:文件类型:0-illegal,1-32bit,2-64bit
        5:编码格式:0-illegal,1-little_endian,2-big_endian
        6:ELF文件头版本，值为:1
        7-15:保留位,用作扩展，0填充
     */
    pub e_ident: [u8; 16],
    pub e_type: HeaderType,
    pub e_machine:MachineType,
    pub e_version:u32,
    pub e_entry:u64,
    pub e_phoff:u64,
    pub e_shoff:u64,
    pub e_flags:u32,
    pub e_ehsize:u16,
    pub e_phentsize:u16,
    pub e_phnum:u16,
    pub e_shentsize:u16,
    pub e_shnum:u16,
    pub e_shstrndx:u16,
}

pub enum HeaderType {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LO_PROC,
    HI_PROC,
    UNKNOWN
}

pub enum BitType {
    BIT32,
    BIT64,
    UNKNOWN
}

impl BitType {
    fn from(data:u8) -> Self {
        match data {
            1 => {BitType::BIT32},
            2 => {BitType::BIT64},
            _ => {BitType::UNKNOWN}
        }
    }

    fn to_string(&self) -> String {
        match self {
            BitType::BIT32 => {String::from("32-bit file")},
            BitType::BIT64 => {String::from("64-bit file")},
            BitType::UNKNOWN => {String::from("UNKNOWN")},
        }
    }
}

enum Encoding {
    LITTLE_ENDIAN,
    BIG_ENDIAN,
    UNKNOWN
}

impl Encoding {
    fn from(data:u8) -> Self {
        match data {
            1 => {Encoding::LITTLE_ENDIAN},
            2 => {Encoding::BIG_ENDIAN},
            _ => {Encoding::UNKNOWN}
        }
    }

    fn to_string(&self) -> String {
        match self {
            Encoding::LITTLE_ENDIAN => {String::from("little endian")},
            Encoding::BIG_ENDIAN => {String::from("big endian")},
            Encoding::UNKNOWN => {String::from("UNKNOWN")},
        }
    }
}

impl HeaderType {
    fn from(data:u16) -> HeaderType {
        match data {
            0 => {HeaderType::NONE},
            1 => {HeaderType::REL},
            2 => {HeaderType::EXEC},
            3 => {HeaderType::DYN},
            4 => {HeaderType::CORE},
            0xff00 => {HeaderType::LO_PROC},
            0xffff => {HeaderType::HI_PROC},
            _ => {HeaderType::UNKNOWN}
        }
    }

    fn to_string(&self) -> String {
        match self {
            HeaderType::NONE => {String::from("Unknown file type")},
            HeaderType::REL => {String::from("REL(Relocatable file)")},
            HeaderType::EXEC => {String::from("EXEC(Executable file)")},
            HeaderType::DYN => {String::from("DYN(Position-Independent Executable file)")},
            HeaderType::CORE => {String::from("Core file")},
            HeaderType::LO_PROC => {String::from("LO_PROC(Processor-specific)")},
            HeaderType::HI_PROC => {String::from("HI_PROC(Processor-specific)")},
            HeaderType::UNKNOWN => {String::from("UNKNOWN")},
        }
    }
}

pub enum MachineType {
    NONE,
    M32,
    SPARC,
    Intel386,
    Motorola68K,
    Motorola88K,
    Intel860,
    MIPSBigEndian,
    MIPSRs4BigEndian,
    HP_PA_RISC,
    NCUBE,
    FujitsuVPP500,
    SPARC32Plus,
    Intel960,
    PowerPC,
    PowerPC64,
    IBM_S390,
    NECV800,
    FujitsuFR20,
    TRWRH32,
    MotorolaRCE,
    AdvancedRISC,
    DigitalAlpha,
    HitachiSH,
    SunSparcV9,
    SiemensTriCore,
    ARC,
    RenesasH8400,
    RenesasH8400H,
    RenesasH8S,
    RenesasH8500,
    IntelIA64,
    MIPSX,
    MotorolaColdFire,
    Motorola68HC12,
    MitsubishiMMA,
    SiemensPCP,
    NationalCompactRISC,
    AMD29K,
    MotorolaStarCore,
    ToyotaME16,
    STMicroelectronicsST100,
    AdvancedLogicCorpTinyJ,
    AMDX86_64,
    RESERVED,
}

impl MachineType {
    pub fn to_string(&self) -> String {
        match self {
            MachineType::NONE => String::from("NONE (Unknown arch)"),
            MachineType::M32 => String::from("M32 (AT&T WE 32100)"),
            MachineType::SPARC => String::from("SPARC"),
            MachineType::Intel386 => String::from("386 (Intel Architecture)"),
            MachineType::Motorola68K => String::from("68K (Motorola 68000)"),
            MachineType::Motorola88K => String::from("88K (Motorola 88000)"),
            MachineType::Intel860 => String::from("860 (Intel 80860)"),
            MachineType::MIPSBigEndian => String::from("MIPS (MIPS RS4000 Big-Endian)"),
            MachineType::MIPSRs4BigEndian => String::from("MIPS_RS4-BE (MIPS RS4000 Big-Endian)"),
            MachineType::HP_PA_RISC => String::from("PARISC (HP PA-RISC)"),
            MachineType::NCUBE => String::from("NCUBE"),
            MachineType::FujitsuVPP500 => String::from("VPP500 (Fujitsu VPP500)"),
            MachineType::SPARC32Plus => String::from("SPARC32PLUS (Sun's \"v8plus\")"),
            MachineType::Intel960 => String::from("960 (Intel 80960)"),
            MachineType::PowerPC => String::from("PPC (PowerPC)"),
            MachineType::PowerPC64 => String::from("PPC64 (64-bit PowerPC)"),
            MachineType::IBM_S390 => String::from("S390 (IBM S/390)"),
            MachineType::NECV800 => String::from("V800 (NEC V800)"),
            MachineType::FujitsuFR20 => String::from("FR20 (Fujitsu FR20)"),
            MachineType::TRWRH32 => String::from("RH32 (TRW RH-32)"),
            MachineType::MotorolaRCE => String::from("RCE (Motorola RCE)"),
            MachineType::AdvancedRISC => String::from("ARM (Advanced RISC Machines)"),
            MachineType::DigitalAlpha => String::from("Alpha (Digital Alpha)"),
            MachineType::HitachiSH => String::from("SH (Hitachi SH)"),
            MachineType::SunSparcV9 => String::from("SPARCV9 (Sun's v9)"),
            MachineType::SiemensTriCore => String::from("TRICORE (Siemens TriCore)"),
            MachineType::ARC => String::from("ARC"),
            MachineType::RenesasH8400 => String::from("H8_400 (Renesas H8/400)"),
            MachineType::RenesasH8400H => String::from("H8_400H (Renesas H8/400H)"),
            MachineType::RenesasH8S => String::from("H8S (Renesas H8S)"),
            MachineType::RenesasH8500 => String::from("H8_500 (Renesas H8/500)"),
            MachineType::IntelIA64 => String::from("IA_64 (Intel IA-64)"),
            MachineType::MIPSX => String::from("MIPS_X (MIPS-X)"),
            MachineType::MotorolaColdFire => String::from("COLDFIRE (Motorola ColdFire)"),
            MachineType::Motorola68HC12 => String::from("68HC12 (Motorola M68HC12)"),
            MachineType::MitsubishiMMA => String::from("MMA (Mitsubishi MMA)"),
            MachineType::SiemensPCP => String::from("PCP (Siemens PCP)"),
            MachineType::NationalCompactRISC => String::from("NCPU (National Semi. CompactRISC)"),
            MachineType::AMD29K => String::from("NDR1 (AMD 29K)"),
            MachineType::MotorolaStarCore => String::from("STARCORE (Motorola Star*Core)"),
            MachineType::ToyotaME16 => String::from("ME16 (Toyota ME16)"),
            MachineType::STMicroelectronicsST100 => String::from("ST100 (STMicroelectronics ST100)"),
            MachineType::AdvancedLogicCorpTinyJ => String::from("TINYJ (Advanced Logic Corp. TinyJ)"),
            MachineType::AMDX86_64 => String::from("X86_64 (AMD x86-64)"),
            MachineType::RESERVED => String::from("RESERVED"),
        }
    }

    fn from(value:u16) -> Self {
        match value {
            0 => MachineType::NONE,
            1 => MachineType::M32,
            2 => MachineType::SPARC,
            3 => MachineType::Intel386,
            4 => MachineType::Motorola68K,
            5 => MachineType::Motorola88K,
            7 => MachineType::Intel860,
            8 => MachineType::MIPSBigEndian,
            10 => MachineType::MIPSRs4BigEndian,
            11..=14 => MachineType::RESERVED,
            15 => MachineType::HP_PA_RISC,
            16 => MachineType::NCUBE,
            17 => MachineType::FujitsuVPP500,
            18 => MachineType::SPARC32Plus,
            19 => MachineType::Intel960,
            20 => MachineType::PowerPC,
            21 => MachineType::PowerPC64,
            22 => MachineType::IBM_S390,
            23..=35 => MachineType::RESERVED,
            36 => MachineType::NECV800,
            37 => MachineType::FujitsuFR20,
            38 => MachineType::TRWRH32,
            39 => MachineType::MotorolaRCE,
            40 => MachineType::AdvancedRISC,
            41 => MachineType::DigitalAlpha,
            42 => MachineType::HitachiSH,
            43 => MachineType::SunSparcV9,
            44 => MachineType::SiemensTriCore,
            45 => MachineType::ARC,
            46 => MachineType::RenesasH8400,
            47 => MachineType::RenesasH8400H,
            48 => MachineType::RenesasH8S,
            49 => MachineType::RenesasH8500,
            50 => MachineType::IntelIA64,
            51 => MachineType::MIPSX,
            52 => MachineType::MotorolaColdFire,
            53 => MachineType::Motorola68HC12,
            54 => MachineType::MitsubishiMMA,
            55 => MachineType::SiemensPCP,
            56 => MachineType::NationalCompactRISC,
            57 => MachineType::AMD29K,
            58 => MachineType::MotorolaStarCore,
            59 => MachineType::ToyotaME16,
            60 => MachineType::STMicroelectronicsST100,
            61 => MachineType::AdvancedLogicCorpTinyJ,
            62 => MachineType::AMDX86_64,
            _ => MachineType::RESERVED,
        }
    }
}

impl ElfHeader {
    fn parse_magic(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:40}","Magic: ")?;
        for x in &self.e_ident {
            write!(f, "{:02x} ", x)?;
        }
        write!(f, "\n")?;
        writeln!(f,"{:40}{}","BitType",BitType::from(self.e_ident[4]).to_string())?;
        write!(f,"{:40}{}\n","Encoding",Encoding::from(self.e_ident[5]).to_string())?;
        write!(f,"{:40}{}\n","Version:",DEFAULT_VERSION)
    }

    fn parse_type(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{:40}{}","Type",self.e_type.to_string())?;
        Ok(())
    }

    fn parse_machine(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{:40}{}","Machine",self.e_machine.to_string())?;
        Ok(())
    }

    fn parse_version(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        match self.e_version {
            0 => {write!(f,"{:40}Illegal version\n","Version:")?},
            1 => {write!(f,"{:40}1\n","Version:")?},
            _ => {return Err(std::fmt::Error::default())}
        };
        Ok(())
    }

    fn parse_entry(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{:#x}\n","Entry point address:",self.e_entry)
    }

    fn parse_phoff(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}(bytes into file)\n","Start of program headers:",self.e_phoff)
    }

    fn parse_shoff(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}(bytes into file)\n","Start of section headers:",self.e_shoff)
    }

    fn parse_flags(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{:#x}\n","Flags:",self.e_flags)
    }

    fn parse_ehsize(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}(bytes)\n","Size of this header:",self.e_ehsize)
    }

    fn parse_phentsize(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}(bytes)\n","Size of program headers:",self.e_phentsize)
    }

    fn parse_phnum(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}\n","Number of program headers:",self.e_phnum)
    }

    fn parse_shentsize(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}(bytes)\n","Size of section headers:",self.e_shentsize)
    }

    fn parse_shnum(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}\n","Number of section headers:",self.e_shnum)
    }

    fn parse_shstrndx(&self,f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:40}{}\n","Section header string table index:",self.e_shstrndx)
    }

}

impl From<Vec<u8>> for ElfHeader {
    fn from(value: Vec<u8>) -> Self {
        Self {
            e_ident: value[0..16].try_into().unwrap(),
            e_type: HeaderType::from(u16::from_ne_bytes(value[16..18].try_into().unwrap())),
            e_machine: MachineType::from(u16::from_ne_bytes(value[18..20].try_into().unwrap())),
            e_version: u32::from_ne_bytes(value[20..24].try_into().unwrap()),
            e_entry: u64::from_ne_bytes(value[24..32].try_into().unwrap()),
            e_phoff: u64::from_ne_bytes(value[32..40].try_into().unwrap()),
            e_shoff: u64::from_ne_bytes(value[40..48].try_into().unwrap()),
            e_flags: u32::from_ne_bytes(value[48..52].try_into().unwrap()),
            e_ehsize: u16::from_ne_bytes(value[52..54].try_into().unwrap()),
            e_phentsize: u16::from_ne_bytes(value[54..56].try_into().unwrap()),
            e_phnum: u16::from_ne_bytes(value[56..58].try_into().unwrap()),
            e_shentsize: u16::from_ne_bytes(value[58..60].try_into().unwrap()),
            e_shnum: u16::from_ne_bytes(value[60..62].try_into().unwrap()),
            e_shstrndx: u16::from_ne_bytes(value[62..64].try_into().unwrap()),
        }
    }
}

impl std::fmt::Display for ElfHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.parse_magic(f)?;
        self.parse_type(f)?;
        self.parse_machine(f)?;
        self.parse_version(f)?;
        self.parse_entry(f)?;
        self.parse_phoff(f)?;
        self.parse_shoff(f)?;
        self.parse_flags(f)?;
        self.parse_ehsize(f)?;
        self.parse_phentsize(f)?;
        self.parse_phnum(f)?;
        self.parse_shentsize(f)?;
        self.parse_shnum(f)?;
        self.parse_shstrndx(f)?;
        Ok(())
    }
}

pub fn parse_header(elf:&mut File) -> io::Result<ElfHeader> {
    elf.rewind()?;
    let mut data = vec![0u8;64];
    elf.read_exact(&mut data)?;
    Ok(ElfHeader::from(data))
}