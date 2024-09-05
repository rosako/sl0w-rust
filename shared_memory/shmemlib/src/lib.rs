extern crate libc;

use libc::{shmctl, shmid_ds, IPC_STAT, EINVAL, getpwuid, passwd};
use std::io;
use std::ffi::CStr;
use std::ptr;

const SHM_STAT: i32 = 13;  // Define SHM_STAT manually
const SHM_INFO: i32 = 14;  // Define SHM_INFO manually

pub struct Segment {
    pub key: String,
    pub shmid : String,
    pub owner : String,
    pub perms : String,
    pub segsz : String,
    pub nattch : String
}


pub fn shm_get_struct(shmid : i32) -> Segment{
    let mut shm_info = unsafe { std::mem::zeroed::<shmid_ds>() };
    let ret = unsafe { shmctl(shmid, SHM_STAT, &mut shm_info) };
    if ret == -1 {
        //let err = io::Error::last_os_error();
        //eprintln!("Failed to get info for segement id {}: {}", shmid, err);
    }else {
        //println!("success : ret code {}", ret);
    }

    
    Segment {
        key : format!("0x{:08x}", shm_info.shm_perm.__key),
        shmid : format!("{}", shmid),
        owner : format!("{}", uid_to_str(shm_info.shm_perm.uid)),
        perms : format!("{}", shm_info.shm_perm.mode & 0x777),
        segsz : format!("{}", shm_info.shm_segsz),
        nattch : format!("{}", shm_info.shm_nattch)
    }


}


pub fn shm_get_segments(max_ids : i32) -> Vec<Segment>{

    let mut segments_vector: Vec<Segment> = Vec::new();

    let mut shm_info : shmid_ds = unsafe { std::mem::zeroed() };


    for id in 0..max_ids {
        let ret = unsafe { shmctl(id, SHM_STAT, &mut shm_info) };

        if ret != -1 {
            segments_vector.push(shm_get_struct(id));
        }
    }

    segments_vector

}


impl Segment {
    pub fn to_line(&self) -> String{
        format!("
                {}\t{}\t{}\t\t\t{}\t{}\t\t{}",
                self.key,
                self.shmid,
                self.owner,
                self.perms,
                self.segsz,
                self.nattch
                )
    }
}





fn uid_to_str(owner_uuid : u32) -> String{
    let uuid_pwd = unsafe { getpwuid(owner_uuid) };
    if uuid_pwd.is_null() {
        eprintln!("Failed to get the owner name for UID {}", owner_uuid);
        return format!("unwknown");;
    }

    let pw_name = unsafe {
        CStr::from_ptr((*uuid_pwd).pw_name).to_string_lossy().into_owned()
    };

    format!("{}({})", owner_uuid, pw_name)

}




//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------



pub fn shm_get(shmid : i32){
    let mut shm_info = unsafe { std::mem::zeroed::<shmid_ds>() };
    let ret = unsafe { shmctl(shmid, SHM_STAT, &mut shm_info) };
    if ret == -1 {
        let err = io::Error::last_os_error();
        eprintln!("Failed to get info for segement id {}: {}", shmid, err);
    }else {
        //println!("success : ret code {}", ret);
    }

    println!("0x{:08x} {} {} {} {} {} {} {}",
             shm_info.shm_perm.__key,
             shmid,
             shm_info.shm_segsz,
             shm_info.shm_nattch,
             shm_info.shm_cpid,
             shm_info.shm_lpid,
             shm_info.shm_perm.mode & 0x777,
             uid_to_str(shm_info.shm_perm.uid));

}


pub fn test_dataset() -> Vec<Segment>{

    vec![
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("32771"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("524288"),
            nattch: String::from("2"),
        },
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("4"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("4194304"),
            nattch: String::from("2")
        },
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("8"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("4194304"),
            nattch: String::from("2")
        },
        Segment{
            key:String::from("0x00006676"),
            shmid: String::from("32778"),
            owner: String::from("urk3l"),
            perms: String::from("666"),
            segsz: String::from("260"),
            nattch: String::from("0")
        },
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("11"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("4194304"),
            nattch: String::from("2")
        },

        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("32783"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("524288"),
            nattch: String::from("2")
        },
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("16"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("4194304"),
            nattch: String::from("2")
        },
        Segment{
            key:String::from("0x00000000"),
            shmid: String::from("19"),
            owner: String::from("urk3l"),
            perms: String::from("600"),
            segsz: String::from("4194304"),
            nattch: String::from("2")
        },

    ]


}




